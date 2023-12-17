

#[derive(Clone, Debug, Hash)]
pub struct NodeGene {
    bias: f32,
}


#[derive(Clone, Debug, Hash)]
pub struct ConnectionGene {
    in_node: u32,
    out_node: u32,
    weight: f32,
    enabled: bool,
}


#[derive(Clone, Debug, Hash)]
pub struct Genome {
    nodes: Vec<NodeGene>,
    connections: Vec<ConnectionGene>,

}
impl Genome {
    pub fn from_bytes(string: &[u8]) {
        let mut byte_iter = string.iter();
        byte_iter.next();
        //grab the four datapoints at a time, and each
        //time see if the bool is actually a value
        //of 2, at which point start reading for
        //node genes, and if there are no connections
        //at all, then there will be at least one with
        //a bool value of 2 simply to delimit in the
        //file
    }
}






