use ordered_float::OrderedFloat;

#[derive(Clone, Debug, Hash)]
pub struct NodeGene {
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
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut offset = 0;
        let mut connections = Vec::new();

        // collect connection genes
        loop {
            if bytes[offset] > 1 { //if we found a delimiter then switch to nodes
                break;
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

            connections.push(new_connection);
            offset += 13; //shift onto the next gene
        }

        let mut nodes = Vec::new();

        // collect node genes
        loop {
            if let None = bytes.get(offset+3) { //check if we've hit the end of the file
                if let None = bytes.get(offset) { //we had some values still, file was invalid
                    break;
                }
                return None;
            }
            let new_node = NodeGene { //create the node gene
                bias:
                    OrderedFloat(f32::from_le_bytes(
                        bytes[offset..offset+4].try_into().unwrap()
                    )),
            };

            nodes.push(new_node);
            offset += 4; //shift onto the next gene
        }

        Some(Genome {
            nodes,
            connections,
        })
    }

    /// Converts the current genome into bytes.
    ///
    /// Important for saving a genome.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for connection in &self.connections {
            bytes.push(connection.enabled as u8);
            bytes.extend_from_slice(&connection.in_node.to_le_bytes());
            bytes.extend_from_slice(&connection.out_node.to_le_bytes());
            bytes.extend_from_slice(&connection.weight.to_le_bytes());
        }
        bytes.push(2); //delimiter
        for node in &self.nodes {
            bytes.extend_from_slice(&node.bias.to_le_bytes());
        }
        bytes
    }
}






