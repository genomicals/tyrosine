use std::collections::{HashMap, HashSet};

use crate::genome::{Genome, ConnectionGene, NodeGene};


pub struct Buckets<'a> {
    outward_connections: HashMap<u32, Vec<u32>>,
    node_lookup: HashMap<u32, &'a NodeGene>,
    connection_lookup: HashMap<(u32, u32), &'a ConnectionGene>,
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
        match connection_lookup.insert((conn.in_node, conn.out_node), conn) { //push to lookup
            Some(_) => return None, //error if we've seen this before
            None => {}, //good
        }
        //if outward_connections.contains_key(&conn.in_node) { //add connection to bucket
        //    outward_connections.get_mut(&conn.in_node).unwrap().push(conn.out_node);
        //} else {
        //    outward_connections.insert(conn.in_node, vec![conn.out_node]);
        //}
        match outward_connections.get_mut(&conn.in_node) { //add connection to bucket
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





