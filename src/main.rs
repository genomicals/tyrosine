
/*
An AI consists of many connections from one node to another
The order of these nodes is basically random, the id has no correlation to the structure
Need to convert list of relationships into an acyclic graph,
    then to a form that is easily computed
We need to find the number of nodes so we can allocate a vector of the correct size
Once we have a vector, each computation can simply store its result in the correct index of the next node
When a node computes itself, simply add its bias and apply normalization, then broadcast to correct indices

If a node comes after another node, then logically it must come after all of the first's dependencies

Potential redesign: don't force the neuralnetwork to output anything, all outputs are 0 by default




CURRENT TODO
Implement NeuralNet::new(), which takes all the edges and organizes them and retrieves the number of total nodes

NOTE FOR NEW()
Make sure all output nodes are placed at the end in the correct order
    even if no nodes take from output nodes, they can still be in a bad order or even have internal nodes between them


*/




pub struct Architect {
    mutability: f32 // represents the probability of a given phenotype to have a genetic mutation.
}


pub struct Phenotype {
    neuralnet: NeuralNet,
}



/// Facilitates NeuralNet calculations
pub struct Node {
    id: usize,
    bias: f32,
    outputs: Vec<(usize, f32)>, //the output node and the weight
}


/// The neural network of a single phenotype
pub struct NeuralNet {
    node_count: usize, //does not align with the size of self.active_nodes, but rather total nodes
    active_nodes: Vec<Node>, //nodes that participate in calculations
    output_size: usize,
}
impl NeuralNet {
    pub fn new(active_nodes: Vec<Node>, output_size: usize, node_count: usize) -> Self {
        NeuralNet { node_count, active_nodes, output_size }
    }

    /// Calculate an output given an input
    pub fn calculate(&self, input: &[f32]) -> Vec<f32> {
        let mut vals: Vec<f32> = Vec::with_capacity(self.node_count);
        vals[..input.len()].copy_from_slice(input); //copy input into vals
        let mut val: f32; //will be updated for each node, to reduce reallocations

        for node in &self.active_nodes { //assuming order has been sorted to remove conflicts
            val = (vals[node.id] + node.bias).tanh(); //apply bias and normalize
            for edge in &node.outputs {
                vals[edge.0] = vals[edge.0] + val * edge.1; //apply weight and add
            }
        }

        // point of annoyance: have to copy all of the output values before returning
        // after which the original values are destroyed
        // ideally you could just return a subset of the vector
        //Vec::from(&vals[input.len()..input.len()+self.output_size])
        //Vec::from(&vals[&vals.len()-self.output_size..]) //grab the last output_size number of nodes as the output
        vals.drain(0..&vals.len()-self.output_size);
        vals
    }
}


fn main() {
    println!("Hello, world!");
}

