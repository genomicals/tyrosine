use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};
use crate::genome::{ConnectionGene, Genome, GlobalInnovator};



#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Phenotype {
    pub genome: Genome,
    pub toposorted_nodes: Vec<usize>, //ids
}
impl Phenotype {
    /// Repeatedly mutates a genome until it gets a valid Phenotype
    pub fn from_mutation(genome: &Genome, innovator: &mut GlobalInnovator, innovations: &mut HashMap<(usize, usize), usize>) -> Phenotype {
        loop {
            let mut gc = genome.clone();
            gc.mutate(innovator, innovations);
            match Phenotype::from_genome(gc) {
                Some(x) => { //successfully generate a phenotype
                    return x;
                },
                None => {
                    continue;
                },
            }
        }
    }


    /// Generates a Phenotype from Genome and checks if it's a valid genome
    pub fn from_genome(genome: Genome) -> Option<Phenotype> {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new(); //maps dependencies to their output
        let mut in_degree: HashMap<usize, usize> = HashMap::new(); //incoming degree of each node

        // initialize, necessary so that all nodes, reachable or not, are captured, including inputs
        for node in &genome.node_genes {
            in_degree.insert(node.id, 0);
        }

        // population both hashmaps
        for conn in genome.connection_genes.iter().filter(|c| c.enabled) {
            graph.entry(conn.in_node).or_default().push(conn.out_node);
            *in_degree.entry(conn.out_node).or_insert(0) += 1;
        }

        // find our starting places
        let mut frontier: VecDeque<usize> = in_degree.iter()
            .filter(|(_, deg)| **deg == 0)
            .map(|(&id, _)| id)
            .collect();

        // sort all the nodes
        let mut sorted = Vec::new();
        while let Some(node) = frontier.pop_front() {
            sorted.push(node); //once a node is being processed, push it to the sorted vector
            if let Some(outputs) = graph.get(&node) { //if the node has children, decrement their degree
                for &output in outputs {
                    let deg = in_degree.get_mut(&output).unwrap();
                    *deg -= 1;
                    if *deg == 0 { //once a node has 0 dependencies left to be processed, push it to the frontier
                        frontier.push_back(output);
                    }
                }
            }
        }

        // detect cycles, invalid network if it's recurrent
        if sorted.len() != genome.node_genes.len() {
            return None;
        }

        Some(Phenotype {
            genome,
            toposorted_nodes: sorted,
        })
    }


    /// Pass input through the neural network and generate an output
    pub fn activate(&self, inputs: &[f64]) -> Vec<f64> {
        // one of the network inputs is the bias, ensure the number of inputs lines up
        assert_eq!(inputs.len() + 1, self.genome.num_inputs, "Number of inputs ({}) didn't match expected amount ({}).", inputs.len(), self.genome.num_inputs - 1);

        // note, entries only exist once the value is calculated, otherwise it will be missing here
        let mut node_values: HashMap<usize, f64> = HashMap::new();

        // initialize input values
        node_values.insert(0, 1.0); //bias node
        for i in 1..self.genome.num_inputs {
            node_values.insert(i, inputs[i]);
        }

        // incoming connections references for each node id
        let incoming = self.genome.connection_genes.iter()
            .filter(|conn| conn.enabled)
            .fold(HashMap::<usize, Vec<&ConnectionGene>>::new(), |mut acc, conn| {
                acc.entry(conn.out_node).or_default().push(conn);
                acc
            });

        // evaluate nodes in topological order
        // NOTE this can be optimized by pre-collecting the weights as such:
        //     HashMap<usize, Vec<(usize, f64)>> // out_node → [(in_node, weight)]
        // but if performance is fine then don't bother
        for &node_id in &self.toposorted_nodes {
            if node_values.contains_key(&node_id) {
                continue; //node value already initialized, i do wonder if this is an error
            }

            let sum: f64 = incoming.get(&node_id)
                .unwrap_or(&vec![])
                .iter()
                .map(|conn| node_values.get(&conn.in_node).unwrap_or(&0.0) * conn.weight)
                .sum();

            node_values.insert(node_id, sum.tanh());
        }

        let output_start = self.genome.num_inputs; //outputs start right after the inputs
        let output_end = output_start + self.genome.num_outputs;

        let outputs = self.genome.node_genes[output_start..output_end].iter()
            .map(|n| *node_values.get(&n.id).unwrap_or(&0.0))
            .collect::<Vec<f64>>();

        outputs
    }
}


