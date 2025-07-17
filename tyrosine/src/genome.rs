use std::collections::{BTreeSet, HashMap};

use rand::{seq::{IndexedMutRandom, IndexedRandom}, Rng};
use serde::{Serialize, Deserialize};
use rand_distr::{Distribution, Normal};



#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalInnovator {
    pub innov: usize,
}
impl GlobalInnovator {
    pub fn new() -> Self {
        GlobalInnovator { innov: 0 }
    }

    /// Get the next innov number and increment internally
    pub fn next(&mut self) -> usize {
        let innov = self.innov;
        self.innov += 1;
        innov
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct NodeGene {
    pub id: usize,
}



#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ConnectionGene {
    pub in_node: usize,
    pub out_node: usize,
    pub weight: f64,
    pub enabled: bool,
    pub innov: usize,
}
impl ConnectionGene {
    /// Returns the IDs of the surrounding nodes, might be unnecessary
    pub fn get_id(&self) -> (usize, usize) {
        (self.in_node, self.out_node)
    }
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Genome {
    pub num_inputs: usize,
    pub num_outputs: usize,
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
    /// Create a new genome with the specified number of inputs and outputs
    pub fn new(num_inputs: usize, num_outputs: usize) -> Self {
        let node_genes = (0..(num_inputs + 1 + num_outputs)).into_iter()
            .map(|i| NodeGene { id: i})
            .collect();

        Genome {
            num_inputs: num_inputs + 1,
            num_outputs,
            node_genes,
            connection_genes: Vec::new(),
        }
    }


    /// Generate a child from two parent genomes, no mutations applied
    /// The first parent will be favored over the second
    pub fn crossover(fit_parent: &Genome, unfit_parent: &Genome) -> Genome {
        let mut child_connections: Vec<ConnectionGene> = Vec::new();        

        let mut fitter_map = HashMap::new();
        for conn in &fit_parent.connection_genes {
            fitter_map.insert(conn.innov, conn);
        }

        let mut unfit_map = HashMap::new();
        for conn in &unfit_parent.connection_genes {
            unfit_map.insert(conn.innov, conn);
        }

        // iterate over both parents, grabbing all innov numbers and saving them to compare
        let all_innovs: BTreeSet<usize> = fitter_map.keys().chain(unfit_map.keys()).cloned().collect();
        for innov in all_innovs { //iterate through all combined innov numbers
            match (fitter_map.get(&innov), unfit_map.get(&innov)) {
                (Some(&a), Some(&b)) => {
                    // matching gene: pick randomly
                    child_connections.push(if rand::random() { a.clone() } else { b.clone() });
                }
                (Some(&a), None) => {
                    // excess: take from fitter
                    child_connections.push(a.clone());
                }
                (None, Some(_b)) => {
                    // gene only in less-fit: ignore
                }
                (None, None) => unreachable!(),
            }
        }

        Genome {
            num_inputs: fit_parent.num_inputs,
            num_outputs: fit_parent.num_outputs,
            node_genes: fit_parent.node_genes.clone(), //fitter parent has same structure, guaranteed to have the same nodes
            connection_genes: child_connections,
        }
    }


    /// Master mutate function, calls the other mutate functions
    /// NOTE: no guarantee that the genome produced is valid
    pub fn mutate(&mut self, innovator: &mut GlobalInnovator, innovations: &mut HashMap<(usize, usize), usize>) {
        let mut rng = rand::rng();

        self.mutate_weights_and_toggle();

        if rng.random::<f64>() < CONNECTION_MUTATION_RATE {
            self.add_connection(innovator, innovations);
        }

        if rng.random::<f64>() < NODE_MUTATION_RATE {
            self.add_node(innovator, innovations);
        }
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


    /// TODO take innovations list
    /// A type of mutation, chooses one connection to split up
    pub fn add_node(&mut self, innovator: &mut GlobalInnovator, innovations: &mut HashMap<(usize, usize), usize>) {
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

        // ensure we reuse innov numbers and remember any new innovations
        let innov0;
        let innov1;
        match innovations.get(&(chosen.in_node, new_id)) {
            Some(x) => { //innovation already exists
                innov0 = *x;
            },
            None => { //new innovation
                innov0 = innovator.next();
                innovations.insert((chosen.in_node, new_id), innov0);
            },
        }
        match innovations.get(&(new_id, chosen.out_node)) {
            Some(x) => { //innovation already exists
                innov1 = *x;
            },
            None => { //new innovation
                innov1 = innovator.next();
                innovations.insert((new_id, chosen.out_node), innov1);
            },
        }

        // create two new connections
        let connection_0 = ConnectionGene {
            in_node: chosen.in_node,
            out_node: new_id,
            weight: chosen.weight, //preserve the functionality of the old connection
            enabled: true,
            innov: innov0,
        };
        let connection_1 = ConnectionGene {
            in_node: new_id,
            out_node: chosen.out_node,
            weight: 1.0, //ensure the other connection still functions like the old connection
            enabled: true,
            innov: innov1,
        };

        // disable and modify old connection
        chosen.enabled = false;
        chosen.weight = 1.0;

        // push to genome
        self.connection_genes.push(connection_0);
        self.connection_genes.push(connection_1);

        // sort the connection genes by innov number and the nodes by id
        self.connection_genes.sort_by_key(|c| c.innov);
        self.node_genes.sort_by_key(|n| n.id);
    }


    /// TODO take in innovations list
    /// A type of mutation, finds two unconnected nodes and adds a connection
    pub fn add_connection(&mut self, innovator: &mut GlobalInnovator, innovations: &mut HashMap<(usize, usize), usize>) {
        if self.node_genes.len() < 2 {
            return;
        }

        // find existing connections (nondirectional)
        let connected: Vec<(usize, usize)> = self.connection_genes.iter()
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

        // randomly pick a connection from the possibilities
        let mut rng = rand::rng();
        let chosen = candidates.choose(&mut rng).copied().unwrap(); //safe unwrap, checked above

        // ensure we reuse innov numbers and remember any new innovations
        let innov;
        match innovations.get(&(chosen.0, chosen.1)) {
            Some(x) => { //innovation already exists
                innov = *x;
            },
            None => { //new innovation
                innov = innovator.next();
                innovations.insert((chosen.0, chosen.1), innov);
            },
        }

        self.connection_genes.push(ConnectionGene {
            in_node: chosen.0,
            out_node: chosen.1,
            weight: 1.0,
            enabled: true,
            innov,
        });

        // sort the connection genes by innov number
        self.connection_genes.sort_by_key(|c| c.innov);
    }
}





