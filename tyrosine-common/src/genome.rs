use ordered_float::OrderedFloat;
use rand::{self, rngs::ThreadRng, Rng};

use crate::{bytes_to_unicode_bits, unicode_bits_to_bytes};


#[derive(Clone, Debug, Hash)]
pub struct NodeGene {
    pub id: u32,
    pub bias: OrderedFloat<f32>,
}


#[derive(Clone, Debug, Hash)]
pub struct ConnectionGene {
    pub in_node: u32,
    pub out_node: u32,
    pub weight: OrderedFloat<f32>,
    pub enabled: bool,
    pub innov: u32,
}
impl ConnectionGene {
    /// Gets the ID of the ConnectionGene, represented by (in_node, out_node).
    pub fn get_id(&self) -> (u32, u32) {
        (self.in_node, self.out_node)
    }
}


#[derive(Clone, Debug, Hash)]
pub struct Genome {
    pub nodes: Vec<NodeGene>,
    pub connections: Vec<ConnectionGene>,

}
impl Genome {
    /// Creates a new randomized simple Genome.
    pub fn new_random(input_size: u32, output_size: u32, rng: &mut ThreadRng) -> Self {
        let mut nodes = Vec::with_capacity((input_size + output_size) as usize);
        let mut connections = Vec::with_capacity((input_size * output_size) as usize);

        // push all new nodes
        for i in 0..input_size+output_size {
            nodes.push(NodeGene {
                id: i,
                bias: OrderedFloat(rng.gen_range(-5.0..5.0))
            });
        }

        // push all new connections
        for i in 0..input_size {
            for j in 0..output_size {
                //println!("creating connection {} to {} where {}, {}", i, input_size + j, input_size, output_size);
                connections.push(ConnectionGene {
                    in_node: i,
                    out_node: input_size + j,
                    weight: OrderedFloat(rng.gen_range(-5.0..5.0)),
                    enabled: rng.gen_bool(0.5),
                    innov: i * output_size + j, //each innov is unique yet consistent
                })
            }
        }

        Genome { nodes, connections }
    }


    /// Attempts to create a genome from the provided non human readable bytes.
    ///
    /// Needs to have its innovation numbers filled by an owner struct.
    pub fn from_bytes_binary(bytes: &[u8]) -> Option<Self> {
        let mut connections = Vec::new();
        let mut offset = 0;

        // collect connection genes
        loop {
            if bytes.len() - offset > 0 && bytes[offset] > 1 { //if we found a delimiter then switch to nodes
                offset += 1;
                break;
            }
            if bytes.len() - offset < 13 { //ensure we have a valid size
                return None;
            }
            bytes.get(12)?; //ensure we have enough elements
            let new_connection = ConnectionGene { //create the connection gene
                in_node:
                    u32::from_le_bytes(
                        bytes[offset+1..offset+5].try_into().unwrap()
                    ),
                out_node:
                    u32::from_le_bytes(
                        bytes[offset+5..offset+9].try_into().unwrap()
                    ),
                weight:
                    OrderedFloat(f32::from_le_bytes(
                        bytes[offset+9..offset+13].try_into().unwrap()
                    )),
                enabled:
                    bool::from(bytes[offset] == 1), //appears first in the file
                innov:
                    0, //this should be filled in later by the GenerationManager

            };
            if new_connection.weight.is_nan() ||
                    new_connection.weight.is_infinite() { //disallow NaN and infinity
                return None;
            }

            connections.push(new_connection);
            offset += 13; //shift onto the next gene
        }

        let mut nodes = Vec::new();
        if (bytes.len() - offset) % 8 != 0 { //invalid file size
            return None;
        }

        // collect node genes
        loop {
            if offset >= bytes.len() { //end of file
                break;
            }
            let new_node = NodeGene { //create the node gene
                bias:
                    OrderedFloat(f32::from_le_bytes(
                        bytes[offset..offset+4].try_into().unwrap()
                    )),
                id:
                    u32::from_le_bytes(
                        bytes[offset+4..offset+8].try_into().unwrap()
                    ),
            };

            nodes.push(new_node);
            offset += 8; //shift onto the next gene
        }

        Some(Genome {
                nodes,
                connections,
        })
    }


    /// Attempts to create a genome from the provided non human readable bytes.
    ///
    /// Needs to have its innovation numbers filled by an owner struct.
    pub fn from_bytes_unicode(bytes: &[u8]) -> Option<Self> {
        Self::from_bytes_binary(&unicode_bits_to_bytes(bytes)?)
    }


    /// Converts the current genome into non human readable bytes.
    ///
    /// Important for saving a genome.
    pub fn as_bytes_binary(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(
            1 + self.connections.len()*13 + self.nodes.len()*8
        );
        for connection in &self.connections {
            bytes.push(connection.enabled as u8);
            bytes.extend_from_slice(&connection.in_node.to_le_bytes());
            bytes.extend_from_slice(&connection.out_node.to_le_bytes());
            bytes.extend_from_slice(&connection.weight.to_le_bytes());
        }
        bytes.push(2); //delimiter
        for node in &self.nodes {
            bytes.extend_from_slice(&node.bias.to_le_bytes());
            bytes.extend_from_slice(&node.id.to_le_bytes());
        }
        bytes
    }


    /// Converts the current genome into human readable bytes.
    ///
    /// Important for saving a genome.
    pub fn as_bytes_unicode(&self) -> Vec<u8> {
        bytes_to_unicode_bits(&self.as_bytes_binary())
    }
}





/* =====================
        TESTING
===================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes_invalid0() {
        let bytes = vec![];
        match Genome::from_bytes_binary(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid1() {
        let bytes = vec![0; 1];
        match Genome::from_bytes_binary(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid2() {
        let bytes = vec![0; 2];
        match Genome::from_bytes_binary(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid3() {
        let bytes = vec![0; 3];
        match Genome::from_bytes_binary(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid4() {
        let bytes = vec![0; 4];
        match Genome::from_bytes_binary(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid5() {
        let bytes = vec![0; 5];
        match Genome::from_bytes_binary(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid6() {
        let bytes = vec![0; 6];
        match Genome::from_bytes_binary(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid7() {
        let bytes = vec![0; 7];
        match Genome::from_bytes_binary(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid8() {
        let bytes = vec![0; 8];
        match Genome::from_bytes_binary(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid9() {
        let bytes = vec![0; 9];
        match Genome::from_bytes_binary(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_valid0() {
        let bytes = vec![2];
        match Genome::from_bytes_binary(&bytes) {
            None => assert!(false),
            Some(_) => assert!(true),
        }
    }
    #[test]
    fn from_bytes_valid1() {
        let mut bytes = vec![0; 9];
        bytes[0] = 2; //delimiter
        match Genome::from_bytes_binary(&bytes) {
            None => assert!(false),
            Some(_) => assert!(true),
        }
    }
    #[test]
    fn from_bytes_valid2() {
        let mut bytes = vec![0; 22];
        bytes[13] = 2; //delimiter
        match Genome::from_bytes_binary(&bytes) {
            None => assert!(false),
            Some(_) => assert!(true),
        }
    }
    #[test]
    fn to_from_test0() {
        let mut bytes = vec![0; 22];
        bytes[13] = 2; //delimiter
        let gen = match Genome::from_bytes_binary(&bytes) {
            None => {
                assert!(false);
                return;
            },
            Some(x) => x,
        };
        let bytes_new = gen.as_bytes_binary();
        assert_eq!(bytes, bytes_new);
    }
}



