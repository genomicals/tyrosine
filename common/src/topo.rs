use std::collections::{HashMap, HashSet};
use sorted_vec::SortedSet;
use crate::genome::{Genome, ConnectionGene, NodeGene};


pub struct BucketsWrapper<'a> {
    pub buckets: HashMap<u32, HashSet<u32>>,
    pub node_lookup: HashMap<u32, &'a NodeGene>,
    pub connection_lookup: HashMap<(u32, u32), &'a ConnectionGene>,
}


/// Toposorts a neural network's buckets into layers of node ids.
///
/// Returns None when neural network is cyclic.
pub fn buckets_to_topo(buckets_wrap: &BucketsWrapper, input_nodes: &HashSet<u32>, output_nodes: &HashSet<u32>) -> Option<Vec<Vec<u32>>> {
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
            .flat_map(|x| buckets_wrap.buckets[x].clone()) //turn into outward nodes
            .filter(|x| !output_nodes.contains(x)) //remove output nodes
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


/// Converts a genome to buckets with collapsed ids.
///
/// Returns None when genome has repeated genes.
pub fn genome_to_buckets(genome: &Genome) -> Option<BucketsWrapper> {
    let mut buckets_unmapped: HashMap<u32, HashSet<u32>> = HashMap::new();

    // create connection lookup table (and buckets)
    for conn in &genome.connections {
        if !conn.enabled { //skip any disabled genes
            continue;
        }

        // add connection to bucket
        match buckets_unmapped.get_mut(&conn.in_node) {
            Some(x) => {
                if !x.insert(conn.out_node) { return None }; //repeated gene
            },
            None => { buckets_unmapped.insert(conn.in_node, HashSet::from([conn.out_node])); }, //cannot already exist
        }
    }

    // create id map for active nodes (collapse the ids)
    let active_nodes: HashSet<u32> = buckets_unmapped
        .keys()
        .cloned()
        .collect();
    let all_nodes_vec: Vec<u32> = active_nodes
        .into_iter()
        .collect();
    let id_map: HashMap<u32, u32> = SortedSet::from(all_nodes_vec) //sort, then map
        .iter()
        .enumerate()
        .map(|x| (x.1.clone(), x.0 as u32) ) //(id_old, id_new)
        .collect();
    
    let mut connection_lookup: HashMap<(u32, u32), &ConnectionGene> = HashMap::new();
    let mut buckets: HashMap<u32, HashSet<u32>> = HashMap::new(); //buckets

    // create updated buckets
    for conn in &genome.connections {
        let mapped_in = id_map[&conn.in_node];
        let mapped_out = id_map[&conn.out_node];

        // build mapped outward_connections
        if buckets_unmapped.contains_key(&mapped_in) &&
                buckets_unmapped[&mapped_in].contains(&mapped_out) {
            match buckets.get_mut(&mapped_in) {
                Some(x) => { x.insert(mapped_out); }, //cannot be repeated
                None => { buckets.insert(mapped_in, HashSet::from([mapped_out])); }, //cannot already exist
            }
        } else {
            continue; //irrelevant connection
        }

        // build connection lookup
        match connection_lookup.insert((mapped_in, mapped_out), conn) {
            Some(_) => return None, //repeated gene
            None => {}, //good
        }
    }

    // create node lookup table
    let mut node_lookup = HashMap::with_capacity(id_map.len());
    for node in &genome.nodes {
        if id_map.contains_key(&node.id) {
            match node_lookup.insert(id_map[&node.id], node) {
                Some(_) => return None, //error if duplicate
                None => {}, //good
            }
        }
    }

    Some(BucketsWrapper {
        connection_lookup,
        node_lookup,
        buckets,
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



