use std::collections::HashSet;
use crate::{genome::Genome, topo::{generate_buckets, toposort, collapse_ids, remap_data_structures, create_bias_map}};


/* =====================
         TRAITS
===================== */


/// Generic trait for all creature types
pub trait Creature {
    fn from_genome(genome: &Genome, input_size: u32, output_size: u32) -> Option<Self>
        where Self: Sized;
    fn calculate(&self, input: &[f32]) -> Vec<f32>;
    fn calculate_gpu(&self, input: &[f32]) -> Option<Vec<f32>>;
}



/* =====================
        STRUCTS
===================== */


/// Helper struct for A6Creature
#[derive(Clone)]
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
#[derive(Clone)]
pub struct AtomicCreature {
    pub arrays: Arrays,
}
impl Creature for AtomicCreature {
    fn from_genome(
        genome: &Genome,
        input_size: u32,
        output_size: u32
    ) -> Option<Self> where Self: Sized {
        let input_ids: HashSet<u32> = HashSet::from_iter(0..input_size);
        let output_ids: HashSet<u32> = HashSet::from_iter(input_size..output_size+input_size);

        // retrieve all data required
        let temp_buckets = generate_buckets(genome, &output_ids)?;
        let temp_topo = toposort(&temp_buckets, &input_ids)?;
        let id_map = collapse_ids(&temp_topo, &output_ids);
        let (buckets, topo) = remap_data_structures(&temp_buckets, &temp_topo, &id_map);
        let bias_map = create_bias_map(&genome, &id_map);
        let num_conns = buckets.iter().map(|x| x.1.len()).sum();
        
        // declare arrays
        let mut mul = Vec::with_capacity(num_conns);
        let mut src = Vec::with_capacity(num_conns);
        let mut dest = Vec::with_capacity(num_conns);
        let lookup;
        let mut calc_threads = Vec::with_capacity(topo.len());
        let mut norm_threads = Vec::with_capacity(topo.len());

        // fill in lookup
        let mut biases_vec = bias_map
            .iter()
            .collect::<Vec<(&u32, &f32)>>();
        biases_vec.sort_by_key(|x| x.0);
        lookup = biases_vec.iter().map(|x| *x.1).collect();

        // fill in the rest of the 5 arrays
        for i in 0..topo.len() { //keep layers in order when pushing
            let layer = &topo[i];
            //calc_threads.push(layer.len() as u32);
            if i > 0 { //don't push the input layer size to norm_threads
                norm_threads.push(layer.len() as u32);
            }
            let mut calc_count = 0; //don't know how many connections there are yet
            for in_node in layer {
                let out_nodes = &buckets[in_node];
                calc_count += out_nodes.len(); //keep track of these connections
                for (out_node, weight) in out_nodes {
                    mul.push(*weight);
                    src.push(*in_node);
                    dest.push(*out_node);
                }
            }
            calc_threads.push(calc_count as u32); //total number of outward connections in this layer
        }
        norm_threads.push(output_size); //want to normalize final output
        
        Some(AtomicCreature {
            arrays: Arrays {
                mul,
                src,
                dest,
                lookup,
                calc_threads,
                norm_threads,
            },
        })
    }


    fn calculate(&self, input: &[f32]) -> Vec<f32> {
        let mut lookup = self.arrays.lookup.clone();
        
        // move input into lookup
        for i in 0..input.len() {
            lookup[i] = input[i];
        }

        // feedforward
        let mut calc_offset: usize = 0;
        let mut thread_offset: usize = 0;
        for i in 0..self.arrays.calc_threads.len() { //iterate for each layer

            // calculate
            for j in 0..self.arrays.calc_threads[i] as usize { //for each connection in this layer
                lookup[self.arrays.dest[calc_offset + j] as usize] +=
                    lookup[self.arrays.src[calc_offset + j] as usize] * //grab the in node's value
                    self.arrays.mul[calc_offset + j]; //multiply it by the connection's weight
            }

            // normalize
            for j in 0..self.arrays.norm_threads[i] as usize { //for each out node in this layer
                lookup[thread_offset + j] = f32::tanh(lookup[thread_offset + j]);
            }

            // update offsets
            calc_offset += self.arrays.calc_threads[i] as usize;
            thread_offset += self.arrays.norm_threads[i] as usize;
        }

        lookup
    }

    
    fn calculate_gpu(&self, input: &[f32]) -> Option<Vec<f32>> {
        return None; //TODO for now
    }
}


pub struct FleekCreature {

}




