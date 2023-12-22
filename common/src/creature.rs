use std::collections::HashSet;
use crate::{genome::Genome, topo::{genome_to_buckets, buckets_to_topo}};


/* =====================
         TRAITS
===================== */


/// Generic trait for all creature types
pub trait Creature {
    fn from_genome(genome: Genome, input_size: u32, output_size: u32) -> Option<Self>
        where Self: Sized;
    fn calculate(input: &[f32]) -> Vec<f32>;
}



/* =====================
        STRUCTS
===================== */


/// Helper struct for A6Creature
pub struct Arrays {
    mul: Vec<f32>, //connection weight
    src: Vec<u32>, //connection in_node
    dest: Vec<u32>, //connection out_node
    lookup: Vec<f32>, //node bias at first, later normalized node value

    calc_threads: Vec<u32>, //threads for calculation
    norm_threads: Vec<u32>, //threads for normalization
}


/// A creature with its neural net represented as a set of six arrays.
///
/// GPU compatible.
pub struct A6Creature {
    pub gpu: bool, //temporary, may be able to detect gpu automatically
    pub arrays: Arrays,
}
impl Creature for A6Creature {
    fn from_genome(genome: Genome, input_size: u32, output_size: u32) -> Option<Self> {
        let buckets = genome_to_buckets(&genome)?; //to buckets
        let input_nodes: HashSet<u32> = genome //hash input node ids
            .nodes[0..input_size as usize]
            .iter()
            .map(|x| x.id)
            .collect();
        let output_nodes: HashSet<u32> = genome //hash output node ids
            .nodes[input_size as usize..(input_size + output_size) as usize]
            .iter()
            .map(|x| x.id)
            .collect();
        let topo = buckets_to_topo(&buckets, &input_nodes, &output_nodes)?;

        // declare arrays
        let mut mul = Vec::with_capacity(buckets.connection_lookup.len());
        let mut src = Vec::with_capacity(buckets.connection_lookup.len());
        let mut dest = Vec::with_capacity(buckets.connection_lookup.len());
        let mut lookup = Vec::with_capacity(buckets.outward_connections.len());
        let mut calc_threads = Vec::with_capacity(topo.len());
        let mut norm_threads = Vec::with_capacity(topo.len());



    }


    fn calculate(input: &[f32]) -> Vec<f32> {
        todo!()
    }
}



