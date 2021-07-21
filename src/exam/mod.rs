pub struct Exam{
    input: Vec<f64>,
    output: f64,
}

impl Exam{
    //Creates a new exam
    pub fn new(input: Vec<f64>, output: f64) -> Exam{
        Exam{
            input,
            output
        }
    }

    #[allow(dead_code)]
    pub fn gen_or() -> Vec<Exam> {
        println!("Generating NAND trial");
        let trial = vec![
            Exam::new(vec![0.0, 0.0], 0.0),
            Exam::new(vec![0.0, 1.0], 1.0),
            Exam::new(vec![1.0, 0.0], 1.0),
            Exam::new(vec![1.0, 1.0], 1.0)
        ];

        trial
    }

    #[allow(dead_code)]
    pub fn gen_xor() -> Vec<Exam> {
        println!("Generating XOR trial");
        let trial = vec![
            Exam::new(vec![0.0, 0.0], 0.0),
            Exam::new(vec![0.0, 1.0], 1.0),
            Exam::new(vec![1.0, 0.0], 1.0),
            Exam::new(vec![1.0, 1.0], 0.0)
        ];

        trial
    }

    #[allow(dead_code)]
    pub fn gen_and() -> Vec<Exam> {
        println!("Generating AND trial");
        let trial = vec![
            Exam::new(vec![0.0, 0.0], 0.0),
            Exam::new(vec![0.0, 1.0], 0.0),
            Exam::new(vec![1.0, 0.0], 0.0),
            Exam::new(vec![1.0, 1.0], 1.0)
        ];

        trial
    }

    #[allow(dead_code)]
    pub fn gen_nand() -> Vec<Exam> {
        println!("Generating NAND trial");
        let trial = vec![
            Exam::new(vec![0.0, 0.0], 1.0),
            Exam::new(vec![0.0, 1.0], 1.0),
            Exam::new(vec![1.0, 0.0], 1.0),
            Exam::new(vec![1.0, 1.0], 0.0)
        ];

        trial
    }

    //Generates empty exams
    #[allow(dead_code)]
    pub fn gen_all_zero(inputs: usize, times: usize) -> Vec<Exam>{
        let mut trial = Vec::new();

        for _ in 0..times{
            let vec = vec![0.0; inputs];
            trial.push(Exam::new(vec, 0.0));
        }

        trial
    }

    //Returns the output
    pub fn get_expected(&self) -> f64{
        self.output
    }

    //Returns the inputs
    pub fn get_input(&self) -> Vec<f64> {
        self.input.clone()
    }
}