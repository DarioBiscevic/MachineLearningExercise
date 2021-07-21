use std::fmt;
use std::time::Instant;

use crate::exam::*;
use crate::layer::*;


#[derive(Clone)]
pub struct NeuralNet{
    layers: Vec<Layer>
}

impl NeuralNet{

    //Creates a new neural network, comprised of layers and nodes
    pub fn new(input_size: usize, layer_structure: Vec<usize>) -> NeuralNet{
        let mut layers = Vec::new();

        //Prepare the neural net according to the given layer structure
        for n in 0..layer_structure.len(){

            //Used to prepare the connections in each node in each layer
            let previous_layer_size;

            //The "previous layer" relative to the first layer is the input
            if n == 0{
                //previous_layer_size = layer_structure[n];
                previous_layer_size = input_size;
            }else{
                previous_layer_size = layer_structure[n-1];
            }

            //Prepare a layer knowing its size and the size of the previous layer
            layers.push(
                Layer::new(n, layer_structure[n], previous_layer_size)
            );
        }

        NeuralNet{
            layers
        }
    }

    //Creates an empty, dummy neural net
    fn new_empty() -> NeuralNet{
        NeuralNet{
            layers: Vec::new()
        }
    }

    //Returns the layer structure
    pub fn get_layer_structure(&self) -> Vec<usize>{
        let mut l_s = Vec::new();

        for i in 0..self.layers.len(){
            l_s.push(
                self.layers[i].get_size()
            );
        }

        l_s
    }

    //Returns the outputs of the last layer
    fn get_final_output(&self) -> Vec<f64>{
        self.layers[self.layers.len()-1].get_outputs()
    }

    //Makes the neural net try the trial
    pub fn examine(&mut self, trials: &[Exam]) -> f64 {

        let mut total_difference = 0.0;

        //The neural net must try each exam in the trial
        for exam in trials.iter(){

            //Dummy layer
            let mut previous_layer: &mut Layer = &mut Layer::new(0, 0, 0);

            //Execute each layer, and feed thm to the next ones
            for (index, layer) in self.layers.iter_mut().enumerate(){
                //If 'index' is 0, the considered layer is the first one, the one fed with inputs
                if index == 0{
                    layer.calculate_values(&exam.get_input());
                }else{
                    layer.calculate_values(&previous_layer.get_outputs());
                }

                previous_layer = layer;
            }

            //TODO: IMPROVE THIS
            let diff_from_expected = exam.get_expected() - self.get_final_output()[0];
            
            //Increase difference coefficient
            total_difference += diff_from_expected.abs();
        }

        total_difference
    }

    //Same es 'examine()', but prints some data
    pub fn examine_and_print(&mut self, trials: &[Exam]) -> f64 {

        let mut total_difference = 0.0;

        for exam in trials.iter(){

            let mut previous_layer: &mut Layer = &mut Layer::new(0, 0, 0);

            for (index, layer) in self.layers.iter_mut().enumerate(){
                //If 'index' is 0, the considered layer is the first one, the one fed with inputs
                if index == 0{
                    layer.calculate_values(&exam.get_input());
                }else{
                    layer.calculate_values(&previous_layer.get_outputs());
                }

                println!("Layer {}:\t{:?}", index, layer.get_outputs());

                previous_layer = layer;
            }

            //TODO: IMPROVE THIS
            let diff_from_expected = exam.get_expected() - self.get_final_output()[0];
            
            total_difference += diff_from_expected.abs();

            println!("Expected:\t{}", exam.get_expected());
            println!();
        }

        total_difference
    }

    //Generates children from a neural net
    fn reproduce(&mut self, children: usize, changes_per_layer: usize, percent_conns_changed: f64, percent_change: f64) -> Vec<NeuralNet>{
        //Array to fill
        let mut next_gen = Vec::new();

        //Used as the prototype of the children
        let original = self.clone();

        //For how many children there are, fill the array with mutated clones of the parent
        for _ in 0..children{
            let mut child = original.clone();

            //For every layer, mutate
            for layer in child.layers.iter_mut(){
                layer.mutate(changes_per_layer, percent_conns_changed, percent_change);
            }

            //Fill the array
            next_gen.push(child);
        }
        
        next_gen
    }
    
    //Whole evolution process; includes reproduction, trial and error, mutation
    //Finishes when the number of generations defined is reached
    pub fn evolve(&mut self, trials: &[Exam], children_each_iter: usize, changes_per_layer: usize, gens: usize, percent_conns_changed: f64, percent_change: f64) -> NeuralNet{
        //For timing purposes; ends after the evolution process
        let start_timer = Instant::now();
        
        //Dummy neural net
        let mut senior = NeuralNet::new_empty();
        
        //For how many gens it is needed
        for iteration in 0..gens{
            //Array of children to be filled
            let mut children;

            //If iteration == 0, the children must be generated from self, the calling neural net
            //Else, the children will be generated from the best net of the previous generation
            if iteration == 0{
                children = self.reproduce(children_each_iter, changes_per_layer, percent_conns_changed, percent_change);
            }else{
                children = senior.reproduce(children_each_iter, changes_per_layer, percent_conns_changed, percent_change);
            }

            //Array with all the scores
            let mut scores = Vec::new();

            //Fill the array with the scores of each network
            for net in children.iter_mut(){
                scores.push(net.examine(trials));
            }

            //Find the best score
            let mut best_score_index = 0;
            for (index, score) in scores.clone().into_iter().enumerate(){
                if score < scores[best_score_index]{
                    best_score_index = index;
                }
            }

            //Make the net with the best score the parent of the next gen
            senior = children[best_score_index].clone();
        }

        //Finish timing
        let duration = start_timer.elapsed();
        println!("Evolution finished in {:?}", duration);

        senior
    }
}

//Debug implementation
impl fmt::Debug for NeuralNet{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        f.debug_struct("NeuralNet")
         .field("layers", &self.layers.len())
         .field("layer_structure", &self.get_layer_structure())
         .finish()
    }
}