use rand::Rng;

#[derive(Clone)]
pub struct Perceptron{
    function: Function,     //The function used by the node
    conn_weights: Vec<f64>, //Array that stores all the weights of the connections
    output: f64,
}

impl Perceptron{

    //Prepares a node
    pub fn default(function: Function, n_weights: usize) -> Perceptron{
        let mut rng = rand::thread_rng();

        let mut conn_weights = Vec::new();

        for _ in 0..n_weights{
            conn_weights.push(rng.gen_range(-3.0..3.0));
        }

        Perceptron{
            function,
            conn_weights,
            output: 0.0,  //Default output is 0.0
        }
    }

    //Sums all the products of the multiplications between the weighs and the corresponding inputs
    pub fn calculate(&mut self, input: &[f64]){
        //For every input, multiply it with the corresponding weight and sum everything
        self.output = self.function.evaluate(input.iter()
                .enumerate()
                .map(|(index, elem)| elem*self.conn_weights[index])
                .sum()
            );
    }

    //Returns the output of the node
    pub fn get_output(&self) -> f64{
        self.output
    }

    //Mutates the node
    pub fn mutate(&mut self, percent_conns_changed: f64, percent_change: f64){
        //Calculate how many connections must be changed
        let conns_to_change = (self.conn_weights.len() as f64 * percent_conns_changed) as usize;

        let mut rng = rand::thread_rng();

        for _ in 0..conns_to_change{
            //Choose a random connection to change ('conn' is an index)
            let conn = rng.gen_range(0..self.conn_weights.len());

            //50/50 chance to change in positive or negative
            if rng.gen_bool(0.5){
                self.conn_weights[conn] += self.conn_weights[conn] * percent_change;
            }else{
                self.conn_weights[conn] -= self.conn_weights[conn] * percent_change;
            }
        }
    }
}



#[derive(Clone)]
pub enum Function{
    Sigmoid,
}

impl Function{
    pub fn evaluate(&self, x: f64) -> f64{
        match self{
            Function::Sigmoid => 1.0/(1.0+std::f64::consts::E.powf(-x)),
        }
    }
}