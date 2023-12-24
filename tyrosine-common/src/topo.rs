use std::collections::{HashMap, HashSet};
use crate::genome::{Genome, ConnectionGene, NodeGene};


/// Creates a map for the node biases.
///
/// Input ids will be extracted too despite their bias meaning nothing.
pub fn create_bias_map(
    genome: &Genome,
    id_map: &HashMap<u32, u32>,
) -> HashMap<u32, f32> {
    let ids: Vec<u32> = id_map.keys().map(|x| *x).collect();
    let mut biases: HashMap<u32, f32> = HashMap::with_capacity(ids.len());
    for node in &genome.nodes {
        if ids.contains(&node.id) {
            biases.insert(id_map[&node.id], node.bias.0);
        }
    }
    biases
}


/// Reassigns ids in existing data structures.
///
/// Remaps buckets and topo.
pub fn remap_data_structures(
    buckets: &HashMap<u32, HashMap<u32, f32>>,
    topo: &Vec<Vec<u32>>,
    id_map: &HashMap<u32, u32>,
) -> (HashMap<u32, HashMap<u32, f32>>, Vec<Vec<u32>>) {
    // remap buckets
    let mut new_buckets = HashMap::with_capacity(buckets.len());
    for inp_pair in buckets {
        let mut new_inners = HashMap::with_capacity(inp_pair.1.len());
        for weight_pair in inp_pair.1 {
            new_inners.insert(id_map[&weight_pair.0], *weight_pair.1);
        }
        new_buckets.insert(id_map[&inp_pair.0], new_inners);
    }

    // remap topo
    let mut new_topo = Vec::with_capacity(topo.len());
    for layer in topo {
        new_topo.push(layer.iter().map(|x| id_map[x]).collect());
    }

    (new_buckets, new_topo)
}


/// Collapses the ids to make them continuous.
///
/// Returns an id map.
pub fn collapse_ids(
    topo: &Vec<Vec<u32>>, //has input nodes as the first layer
    output_ids: &HashSet<u32>
) -> HashMap<u32, u32> {
    let mut mapping = HashMap::new();
    let mut new_id = 0;

    // reassign ids in the topo
    for layer in topo {
        for id in layer {
            if !mapping.contains_key(id) {
                mapping.insert(*id, new_id);
                new_id += 1;
            }
        }
    }

    // reassign output nodes
    for id in output_ids {
        mapping.insert(*id, new_id);
        new_id += 1;
    }
    
    mapping
}


/// Toposorts a neural network's buckets into layers of node ids.
///
/// Returns None when neural network is cyclic.
pub fn toposort(
    buckets: &HashMap<u32, HashMap<u32, f32>>,
    input_ids: &HashSet<u32>,
) -> Option<Vec<Vec<u32>>> {
    let mut cur_layer: Vec<u32> = input_ids.iter().map(|x| *x).collect();
    let mut layers: Vec<Vec<u32>> = Vec::new();
    let mut past_layers: HashSet<Vec<u32>> = HashSet::new();

    let layer_copy = cur_layer.clone();
    past_layers.insert(layer_copy.clone()); //hash this layer
    layers.push(layer_copy); //remember this layer

    // iteratively generate and verify the next layer
    loop {
        let new_layer: Vec<u32> = cur_layer
            .iter()
            .flat_map(|x| buckets[x].keys().map(|x| *x).collect::<Vec<u32>>())
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
/// Removes all nodes that aren't read from.
/// Returns None when genome has repeated genes.
pub fn generate_buckets(
    genome: &Genome,
    output_ids: &HashSet<u32>,
) -> Option<HashMap<u32, HashMap<u32, f32>>> {
    let mut buckets: HashMap<u32, HashMap<u32, f32>> = HashMap::new(); //in_node, out_node, weight
    let mut active_nodes: HashSet<u32> = HashSet::new(); //keeps track of nodes we've seen excluding input nodes

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
        active_nodes.insert(conn.out_node); //remember this node
    }

    // now remove all nodes that don't own a bucket (we don't want to send values to these)
    let true_nodes: HashSet<u32> = buckets.keys().map(|x| *x).collect(); //all nodes with buckets
    let bad_nodes: Vec<u32> = active_nodes //all nodes to remove from the buckets (input nodes cannot appear in buckets)
        .difference(&true_nodes) //keep all nodes with buckets
        .map(|x| *x)
        .collect::<HashSet<u32>>()
        .difference(&output_ids) //keep all output nodes
        .map(|x| *x)
        .collect();
    for (inp_id, bucket) in &mut buckets {
        for id in &bad_nodes {
            bucket.remove(&id); //remove bad nodes from being sent values
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



