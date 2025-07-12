use std::collections::{HashMap, HashSet, VecDeque};
use crate::genome::Genome;



pub struct Phenotype {
    pub genome: Genome,
    pub toposorted_nodes: Vec<u32>, //ids
}
impl Phenotype {
    /// TODO
    /// Generates a Phenotype from Genome and checks if it's a valid genome
    pub fn from_genome(genome: Genome) -> Option<Phenotype> {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new(); //maps dependencies to their output
        let mut in_degree: HashMap<u32, usize> = HashMap::new(); //incoming degree of each node

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
        let mut frontier: VecDeque<u32> = in_degree.iter()
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
}


