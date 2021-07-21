use ansi_term::Colour;

mod nn;
mod layer;
mod perceptron;
mod exam;

use nn::*;
use exam::*;

fn main() {

    //Generate the trial, with inputs and outputs
    let trial = Exam::gen_and();

    let input_size = 2;

    //Define the structure that the neural net will have
    let layer_structure = vec![7, 3, 4, 2, 1];

    //Prepare neural net
    let mut initial_nn = NeuralNet::new(input_size, layer_structure);

    println!("{:?}", initial_nn);

    //Test the net with the initial values (for later comparison)
    let first_run = initial_nn.examine(&trial);
    println!("{}{:.3}\n", Colour::Red.paint("First run difference: "), first_run);

    //Evolution parameters
    let children_per_iter = 1000;    //From how many children will the senior of the next gen be chosen
    let changes_per_iter = 20;       //How many changes each layer will have when reproducing
    let generations = 2000;          //After how many generations will the evolution process stop
    let percent_conns_changed = 0.5; //How many connections connected each node will be changed, in %
    let percent_change = 0.15;       //How much will the weights change, in %

    //Print data
    println!("Children per iteration:\t{}
Changes per iteration:\t{}
No. of generations:\t{}
% connections changed:\t{}
% change in each conn.:\t{}\n", children_per_iter, changes_per_iter, generations, percent_conns_changed, percent_change);

    //Evolve and store the perfected net in a variable
    let mut evolved =
        initial_nn.evolve(&trial, children_per_iter, changes_per_iter, generations, percent_conns_changed, percent_change);

    //Test the evolved net
    let last_run = evolved.examine_and_print(&trial);
    println!("{}{:.3}\n", Colour::Green.paint("Last run difference: "), last_run);
}