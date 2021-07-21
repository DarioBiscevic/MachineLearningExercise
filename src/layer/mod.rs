use rand::Rng;

use crate::perceptron::*;

#[derive(Clone)]
pub struct Layer{
    n: usize,               //What layer is it in the net
    nodes: Vec<Perceptron>, //All the nodes inside the layer
}

impl Layer{

    //Creates a new layer
    pub fn new(n: usize, layer_size: usize, previous_layer_size: usize) -> Layer{
        let mut nodes = Vec::new();

        //Fill the layer with nodes
        for _ in 0..layer_size{
            nodes.push(
                /*
                 * 'previous_layer_size' is used to determine how many weights each node has to prepare.
                 * 
                 * number of nodes in the previous layer == number of weights to prepare (one for each input)
                 */
                Perceptron::default(Function::Sigmoid, previous_layer_size)
            );
        }

        Layer{
            n,
            nodes
        }
    }

    //Returns the size of the layer
    pub fn get_size(&self) -> usize{
        self.nodes.len()
    }

    //Returns all the outputs of all the nodes contained in the layer
    pub fn get_outputs(&self) -> Vec<f64>{
        let mut outputs = Vec::new();

        for node in self.nodes.iter(){
            outputs.push(node.get_output())
        }

        outputs
    }

    //Makes each node calculate its output
    pub fn calculate_values(&mut self, input: &[f64]){
        for node in self.nodes.iter_mut(){
            node.calculate(&input);
        }
    }

    //Mutates the layer
    pub fn mutate(&mut self, n_changes: usize, percent_conns_changed: f64, percent_change: f64){
        let mut rng = rand::thread_rng();

        //For how many times the layer must be mutated, mutate it
        for _ in 0..n_changes{
            //Choose a random node to mutate
            let node_to_change = rng.gen_range(0..self.nodes.len());

            //Muatet the chosen node
            self.nodes[node_to_change].mutate(percent_conns_changed, percent_change);
        }
    }
}