use std::collections::{HashMap, HashSet};

use crate::genome::{Genome, ConnectionGene, NodeGene};


pub struct Buckets<'a> {
    outward_connections: HashMap<u32, Vec<u32>>,
    node_lookup: HashMap<u32, &'a NodeGene>,
    connection_lookup: HashMap<(u32, u32), &'a ConnectionGene>,
}


/// Takes a set of buckets, input nodes, and output nodes, and toposorts them
pub fn buckets_to_topo(buckets: &Buckets, input_nodes: &HashSet<u32>, output_nodes: &HashSet<u32>) -> Option<Vec<Vec<u32>>> {
    let mut cur_layer: Vec<u32> = input_nodes.iter().map(|x| x.clone()).collect();
    let mut layers: Vec<Vec<u32>> = Vec::new();
    let mut past_layers: HashSet<Vec<u32>> = HashSet::new();

    let layer_copy = cur_layer.clone();
    past_layers.insert(layer_copy.clone()); //hash this layer
    layers.push(layer_copy); //remember this layer

    // iteratively generate and verify the next layer
    loop {
        // replace all nodes with their outwardly connected nodes
        let new_layer: Vec<u32> = cur_layer
            .iter()
            .flat_map(|x| buckets.outward_connections[x].clone())
            .filter(|x| !output_nodes.contains(x))
            .collect();

        if new_layer.len() == 0 { //finish if we've exhausted all layers
            break;
        }

        // ensure we haven't seen this before
        let layer_copy = new_layer.clone();
        if past_layers.contains(&layer_copy) {
            return None; //cyclic, invalid genome
        } else {
            past_layers.insert(layer_copy);
        }

        cur_layer = new_layer;
    }
    
    Some(layers)
}


/// Converts a genome to buckets
pub fn genome_to_buckets(genome: &Genome) -> Option<Buckets> {
    let mut outward_connections: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut connection_lookup: HashMap<(u32, u32), &ConnectionGene> = HashMap::new();

    // create connection lookup table (and buckets)
    for conn in &genome.connections {
        if !conn.enabled { //skip any disabled genes
            continue;
        }

        //push to lookup
        match connection_lookup.insert((conn.in_node, conn.out_node), conn) {
            Some(_) => return None, //error if we've seen this before
            None => {}, //good
        }

        // add connection to bucket
        match outward_connections.get_mut(&conn.in_node) {
            Some(x) => {
                x.push(conn.out_node);
            },
            None => {
                outward_connections.insert(conn.in_node, vec![conn.out_node]);
            },
        }
    }

    // create node lookup table
    let active_nodes: HashSet<u32> = outward_connections.keys().cloned().collect();
    let mut node_lookup = HashMap::with_capacity(active_nodes.len());
    for node in &genome.nodes {
        if active_nodes.contains(&node.id) {
            match node_lookup.insert(node.id, node) {
                Some(_) => return None, //error if duplicate
                None => {}, //good
            }
        }
    }

    Some(Buckets {
        connection_lookup,
        node_lookup,
        outward_connections,
    })

}





/* =====================
        TESTING
===================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        let z = vec![1, 2, 3, 4];
        let y = vec![1, 2, 3, 4];
        let mut x = HashSet::new();
        x.insert(z);
        x.insert(y);
        assert_eq!(x.len(), 1);
    }
}


