use std::collections::HashSet;
//use crate::{genome::Genome, topo::{genome_to_buckets, buckets_to_topo}};
use crate::genome::Genome;


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
pub struct AtomicCreature {
    pub gpu: bool, //temporary, may be able to detect gpu automatically
    pub arrays: Arrays,
}
//impl Creature for AtomicCreature {
//    fn from_genome(genome: Genome, input_size: u32, output_size: u32) -> Option<Self> {
//        let buckets_wrap = genome_to_buckets(&genome)?; //to buckets
//        let input_nodes: HashSet<u32> = genome //hash input node ids
//            .nodes[0..input_size as usize]
//            .iter()
//            .map(|x| x.id)
//            .collect();
//        let output_nodes: HashSet<u32> = genome //hash output node ids
//            .nodes[input_size as usize..(input_size + output_size) as usize]
//            .iter()
//            .map(|x| x.id)
//            .collect();
//        let topo = buckets_to_topo(&buckets_wrap, &input_nodes, &output_nodes)?;
//
//        // declare arrays
//        let mut mul = Vec::with_capacity(buckets_wrap.connection_lookup.len());
//        let mut src = Vec::with_capacity(buckets_wrap.connection_lookup.len());
//        let mut dest = Vec::with_capacity(buckets_wrap.connection_lookup.len());
//        let mut lookup = Vec::with_capacity(buckets_wrap.buckets.len());
//        let mut calc_threads = Vec::with_capacity(topo.len());
//        let mut norm_threads = Vec::with_capacity(topo.len());
//
//        // fill in the first three arrays
//        for layer in topo {
//            let mut layer_size = 0;
//            norm_threads.push(layer.len());
//            let conns_tuples: Vec<(u32, Vec<u32>)> = layer //retrieve connections
//                .iter()
//                .map(|x| (*x, Vec::from_iter(buckets_wrap.buckets[x])) )
//                .collect();
//            let mut all_connections = Vec::new(); //all connections for this layer
//
//            // fill in the first three arrays
//            for inp in conns_tuples {
//                let new_connections: Vec<(u32, u32)> = inp.1.iter().map(|x| (inp.0, *x)).collect();
//                layer_size += new_connections.len();
//                for con in &new_connections {
//                    mul.push(buckets_wrap.connection_lookup[con].weight);
//                    src.push(buckets_wrap.connection_lookup[con].in_node);
//                    dest.push(buckets_wrap.connection_lookup[con].out_node);
//                }
//            }
//
//            calc_threads.push(layer_size);
//
//            //for con in &all_connections {
//            //    mul.push(buckets_wrap.connection_lookup[con].weight);
//            //    src.push(buckets_wrap.connection_lookup[con].in_node);
//            //    dest.push(buckets_wrap.connection_lookup[con].out_node);
//            //}
//        }
//
//
//        todo!()
//
//    }
//
//
//    fn calculate(input: &[f32]) -> Vec<f32> {
//        todo!()
//    }
//}


pub struct FleekCreature {

}




