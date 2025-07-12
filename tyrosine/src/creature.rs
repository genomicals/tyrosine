use rand::{seq::{IndexedMutRandom, IndexedRandom}, Rng};
use serde::{Serialize, Deserialize};
use rand_distr::{Distribution, Normal};



#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalInnovator {
    pub innov: u32,
}
impl GlobalInnovator {
    pub fn new() -> Self {
        GlobalInnovator { innov: 0 }
    }

    /// Get the next innov number and increment internally
    pub fn next(&mut self) -> u32 {
        let innov = self.innov;
        self.innov += 1;
        innov
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct NodeGene {
    pub id: u32,
    //pub bias: f32, //remember to implemment this as an extra input node instead
}



#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionGene {
    pub in_node: u32,
    pub out_node: u32,
    pub weight: f64,
    pub enabled: bool,
    pub innov: u32,
}
impl ConnectionGene {
    /// Returns the IDs of the surrounding nodes, might be unnecessary
    pub fn get_id(&self) -> (u32, u32) {
        (self.in_node, self.out_node)
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Genome {
    pub num_inputs: u32, //excludes bias input
    pub num_outputs: u32,
    pub node_genes: Vec<NodeGene>,
    pub connection_genes: Vec<ConnectionGene>,
}
const CONNECTION_MUTATION_RATE: f64 = 0.15;
const NODE_MUTATION_RATE: f64 = 0.03; //should be the least common mutation type
const WEIGHT_MUTATION_RATE: f64 = 0.8;
const PERTUBATION_CHANCE: f64 = 0.9; //(1-pertubation_chance) is the chance of total replacement vs just a nudge
const PERTUBATION_STD: f64 = 0.1;
const REPLACEMENT_RANGE: f64 = 5.0;
const TOGGLE_MUTATION_RATE: f64 = 0.01;
impl Genome {

    ///// Run the input through the neural network
    //pub fn forward(&self, input: &[u32]) -> Vec<u32> {
    //    // ensure cyclic systems are... handled or maybe keep them from existing below
    //    // idk how we're gonna do this function, hopefully we don't modify the structs
    //    // maybe we don't do a forward here, and handle that in the population manager?
    //    // honestly if we detect a cycle here, lets just return a None or error or something
    //    todo!()
    //}


    /// Master mutate function, calls the other mutate functions
    /// NOTE: no guarantee that the genome produced is valid
    pub fn mutate(&mut self, innovator: &mut GlobalInnovator) {
        let mut rng = rand::rng();

        self.mutate_weights_and_toggle();

        if rng.random::<f64>() < CONNECTION_MUTATION_RATE {
            self.add_connection(innovator);
        }

        if rng.random::<f64>() < NODE_MUTATION_RATE {
            self.add_node(innovator);
        }

        todo!()
    }


    /// Apply mutations to internal weights
    pub fn mutate_weights_and_toggle(&mut self) {
        let mut rng = rand::rng();
        let normal = Normal::new(0.0, PERTUBATION_STD).unwrap(); //probably safe unwrap

        for connection in &mut self.connection_genes {
            if rng.random::<f64>() < TOGGLE_MUTATION_RATE {
                connection.enabled = !connection.enabled;
            }

            if rng.random::<f64>() > WEIGHT_MUTATION_RATE { //small chance we don't mutate
                continue;
            }

            if rng.random::<f64>() < PERTUBATION_CHANCE {
                // pertubate the weight
                let pertub_amount = normal.sample(&mut rng);
                connection.weight += pertub_amount;
            } else {
                // replace the weight
                let new_weight = rng.random_range(-REPLACEMENT_RANGE..REPLACEMENT_RANGE);
                connection.weight = new_weight;
            }
        }
    }


    /// A type of mutation, chooses one connection to split up
    pub fn add_node(&mut self, innovator: &mut GlobalInnovator) {
        if self.connection_genes.len() < 1 {
            return;
        }

        // choose existing connection
        let mut rng = rand::rng();
        let mut collected = self.connection_genes.iter_mut()
            .filter(|x| x.enabled)
            .collect::<Vec<&mut ConnectionGene>>();

        // disable the existing node
        let chosen = collected.choose_mut(&mut rng).unwrap(); //safe unwrap

        // create a new node
        let new_id = self.node_genes.last().unwrap().id + 1;
        self.node_genes.push(NodeGene { id: new_id});

        // create two new connections
        let connection_0 = ConnectionGene {
            in_node: chosen.in_node,
            out_node: new_id,
            weight: chosen.weight, //preserve the functionality of the old connection
            enabled: true,
            innov: innovator.next(),
        };
        let connection_1 = ConnectionGene {
            in_node: new_id,
            out_node: chosen.out_node,
            weight: 1.0,
            enabled: true,
            innov: innovator.next(),
        };

        // disable and modify old connection
        chosen.enabled = false;
        chosen.weight = 1.0;

        // push to genome
        self.connection_genes.push(connection_0);
        self.connection_genes.push(connection_1);
    }


    /// A type of mutation, finds two unconnected nodes and adds a connection
    pub fn add_connection(&mut self, innovator: &mut GlobalInnovator) {
        if self.node_genes.len() < 2 {
            return;
        }

        // find existing connections (nondirectional)
        let connected: Vec<(u32, u32)> = self.connection_genes.iter()
            .map(|c|
                if c.in_node < c.out_node {
                    (c.in_node, c.out_node)
                } else {
                    (c.out_node, c.in_node)
                }
            ).collect();

        // find possible new connections
        let mut candidates = Vec::new();
        for (i, a) in self.node_genes.iter().enumerate() {
            for b in &self.node_genes[i + 1..] {
                let pair = if a.id < b.id { (a.id, b.id) } else { (b.id, a.id) };
                
                // ensure the connect doesn't exist
                if connected.contains(&pair) {
                    continue;
                }

                // ensure no connections between inputs made...
                if pair.0 < self.num_inputs && pair.1 < self.num_inputs {
                    continue;
                }

                // or between outputs
                if (pair.0 >= self.num_inputs && pair.0 < self.num_outputs) &&
                        (pair.1 >= self.num_inputs && pair.1 < self.num_outputs) {
                    continue;
                }

                // push if all conditions met
                candidates.push(pair);
            }
        }

        // look, i know this is a problem if no more connections can be made,
        // but i'm banking on the number of possible connections growing exponentially
        // where the number of actual connections grows linearly
        //
        // worst case scenario add a check at the top of the function, i just
        // don't wanna do the math
        if candidates.len() == 0 {
            return;
        }

        // randomly pick a new node from the possibilities
        let mut rng = rand::rng();
        let chosen = candidates.choose(&mut rng).copied().unwrap(); //safe unwrap, checked above
        self.connection_genes.push(ConnectionGene {
            in_node: chosen.0,
            out_node: chosen.1,
            weight: 1.0,
            enabled: true,
            innov: innovator.next(),
        });
    }
}





