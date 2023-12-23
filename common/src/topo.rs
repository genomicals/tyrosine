use std::collections::{HashMap, HashSet};
use crate::genome::{Genome, ConnectionGene, NodeGene};


pub struct BucketsWrapper<'a> {
    pub buckets: HashMap<u32, HashSet<u32>>,
    pub node_lookup: HashMap<u32, &'a NodeGene>,
    pub connection_lookup: HashMap<(u32, u32), &'a ConnectionGene>,
}


/// Toposorts a neural network's buckets into layers of node ids.
///
/// Returns None when neural network is cyclic.
pub fn toposort(
    buckets: HashMap<u32, HashMap<u32, f32>>,
    input_nodes: HashSet<u32>,
    output_nodes: HashSet<u32>,
) -> Option<Vec<Vec<u32>>> {
    let mut cur_layer: Vec<u32> = input_nodes.iter().map(|x| *x).collect();
    let mut layers: Vec<Vec<u32>> = Vec::new();
    let mut past_layers: HashSet<Vec<u32>> = HashSet::new();

    let layer_copy = cur_layer.clone();
    past_layers.insert(layer_copy.clone()); //hash this layer
    layers.push(layer_copy); //remember this layer

    // iteratively generate and verify the next layer
    loop {
        let new_layer: Vec<u32> = cur_layer
            .iter()
            .flat_map(|x| buckets[x].keys().map(|x| *x).collect::<Vec<u32>>()
            )
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
pub fn generate_buckets(genome: &Genome) -> Option<HashMap<u32, HashMap<u32, f32>>> {
    let mut buckets: HashMap<u32, HashMap<u32, f32>> = HashMap::new(); //in_node, out_node, weight

    // push all active connections to buckets
    for conn in genome.connections.iter().filter(|x| x.enabled) {
        match buckets.get_mut(&conn.in_node) {
            Some(outs) => { //bucket exists already
                if outs.contains_key(&conn.out_node) { return None; } //repeated gene
                else { outs.insert(conn.out_node, conn.weight.0); }
            },
            None => { //bucket doesn't exist yet
                buckets.insert(conn.in_node, HashMap::from([(conn.out_node, conn.weight.0)]));
            },
        }
    }

    Some(buckets)
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



