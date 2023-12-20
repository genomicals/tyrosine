use ordered_float::OrderedFloat;

#[derive(Clone, Debug, Hash)]
pub struct NodeGene {
    id: u32,
    bias: OrderedFloat<f32>,
}


#[derive(Clone, Debug, Hash)]
pub struct ConnectionGene {
    in_node: u32,
    out_node: u32,
    weight: OrderedFloat<f32>,
    enabled: bool,
}


#[derive(Clone, Debug, Hash)]
pub struct Genome {
    nodes: Vec<NodeGene>,
    connections: Vec<ConnectionGene>,

}
impl Genome {
    /// Attempts to create a genome from the provided bytes.
    pub fn from_bytes(bytes: &[u8]) -> Option<(Self, u32, u32)> {
        let mut connections = Vec::new();

        // read input and output sizes
        if bytes.len() < 8 {
            return None
        }
        let input_size = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let output_size = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let mut offset = 8;

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
                    bool::from(bytes[offset] == 1) //appears first in the file
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

        Some((
            Genome {
                nodes,
                connections,
            },
            input_size, //for verification that this genome is compatible
            output_size,
        ))
    }


    /// Converts the current genome into bytes.
    ///
    /// Important for saving a genome.
    pub fn as_bytes(&self, input_nodes: u32, output_nodes: u32) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(
            9 + self.connections.len()*13 + self.nodes.len()*8
        );
        bytes.extend_from_slice(&input_nodes.to_le_bytes());
        bytes.extend_from_slice(&output_nodes.to_le_bytes());
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
        match Genome::from_bytes(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid1() {
        let bytes = vec![0; 1];
        match Genome::from_bytes(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid2() {
        let bytes = vec![0; 2];
        match Genome::from_bytes(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid3() {
        let bytes = vec![0; 3];
        match Genome::from_bytes(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid4() {
        let bytes = vec![0; 4];
        match Genome::from_bytes(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid5() {
        let bytes = vec![0; 5];
        match Genome::from_bytes(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid6() {
        let bytes = vec![0; 6];
        match Genome::from_bytes(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid7() {
        let bytes = vec![0; 7];
        match Genome::from_bytes(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid8() {
        let bytes = vec![0; 8];
        match Genome::from_bytes(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_invalid9() {
        let bytes = vec![0; 9];
        match Genome::from_bytes(&bytes) {
            None => assert!(true),
            Some(_) => assert!(false),
        }
    }
    #[test]
    fn from_bytes_valid0() {
        let mut bytes = vec![0; 9];
        bytes[0] = 5; //input
        bytes[4] = 8; //output
        bytes[8] = 2; //delimiter
        match Genome::from_bytes(&bytes) {
            None => assert!(false),
            Some((_, 5, 8)) => assert!(true),
            _ => assert!(false),
        }
    }
    #[test]
    fn from_bytes_valid1() {
        let mut bytes = vec![0; 17];
        bytes[0] = 5; //input
        bytes[4] = 8; //output
        bytes[8] = 2; //delimiter
        match Genome::from_bytes(&bytes) {
            None => assert!(false),
            Some(_) => assert!(true),
        }
    }
    #[test]
    fn from_bytes_valid2() {
        let mut bytes = vec![0; 30];
        bytes[0] = 5; //input
        bytes[4] = 8; //output
        bytes[21] = 2; //delimiter
        match Genome::from_bytes(&bytes) {
            None => assert!(false),
            Some(_) => assert!(true),
        }
    }
    #[test]
    fn to_from_test0() {
        let mut bytes = vec![0; 30];
        bytes[0] = 5; //input
        bytes[4] = 8; //output
        bytes[21] = 2; //delimiter
        let (gen, inp, outp) = match Genome::from_bytes(&bytes) {
            None => {
                assert!(false);
                return;
            },
            Some(x) => x,
        };
        let bytes_new = gen.as_bytes(inp, outp);
        assert_eq!(bytes, bytes_new);
    }
}





