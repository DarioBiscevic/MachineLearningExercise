use rand::Rng;

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
        println!("Generating OR trial");
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


    //1 = above line
    //0 = under line
    pub fn gen_points_linear(m: f64, q: f64, n_points: usize) -> Vec<Exam>{

        println!("Generating {} points along the line y={m}*x+{q}", n_points, m=m, q=q);

        let mut rng = rand::thread_rng();

        let mut trials = Vec::new();

        for _ in 0..n_points{
            let x = rng.gen_range(0.0..100.0);
            let y = rng.gen_range(0.0..100.0);

            let y_on_line = m*x+q;

            let expected;

            if y > y_on_line{
                expected = 1.0;
            }else{
                expected = 0.0;
            }

            trials.push(Exam::new(vec![x, y], expected));
        }

        trials
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