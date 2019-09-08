#[derive(Debug)]
#[allow(dead_code)]
pub enum FlatteningOperator {
    Add,
    Substract,
    Multiply,
    Divide,
}

pub struct Gate {
    pub a: std::vec::Vec<u32>,
    pub b: std::vec::Vec<u32>,
    pub c: std::vec::Vec<u32>,
}

impl Gate {
    // set vectors
    pub fn set_vectors(&mut self, a: std::vec::Vec<u32>, b: std::vec::Vec<u32>, c: std::vec::Vec<u32>) { // TODO: check if this way of passing parameters is proper Rust-like style
        self.a = a;
        self.b = b;
        self.c = c;
    }

    // print the content of Gate
    pub fn print(&self) {
        for i in 0..(self.a.len()) {
            print!("{} ", self.a[i]);
        }
        print!("\n");
        for i in 0..(self.b.len()) {
            print!("{} ", self.b[i]);
        }
        print!("\n");
        for i in 0..(self.c.len()) {
            print!("{} ", self.c[i]);
        }
        print!("\n\n");
    }
}

// TODO: can we make the struct fields private?
pub struct FlattenedEquation {
    pub operands: std::vec::Vec<(u32, u32)>,
    pub operators: std::vec::Vec<FlatteningOperator>,
    pub gates: std::vec::Vec<Gate>,
}

impl FlattenedEquation {
    // add operand
    pub fn add_operand(&mut self, operand: (u32, u32)) { // TODO check if parameters are rust style
        self.operands.push(operand);
    }

    // add operator
    pub fn add_operator(&mut self, operator: FlatteningOperator) {
        self.operators.push(operator);
    }

    // add gate
    pub fn add_gate(&mut self, a: std::vec::Vec<u32>, b: std::vec::Vec<u32>, c: std::vec::Vec<u32>) {
        let gate = Gate {
            a: a,
            b: b,
            c: c,
        };
        self.gates.push(gate);
    }

    // print the content of Flattened_equation
    pub fn print(&self) {
        if self.operands.len() > 0 {
            print!("{}x{} ", self.operands[0].0, self.operands[0].1);
        }

        if self.operands.len() > 1 {
            for i in 0..(self.operators.len()) {
                print!("{:?} {}x{} ", self.operators[i], self.operands[i + 1].0, self.operands[i + 1].1);
            }
            print!("\n");
        }
        
        for i in 0..(self.gates.len()) {
            self.gates[i].print();
        }
        
        let diff = self.operands.len() - self.operators.len();
        if diff != 1 {
            panic!("length of operators is not length of operands -1. Difference is: {}.", diff); // TODO: this is not a logical function to panic!
        }
    }
    
    // calculate output of Flattened_equation given input
    pub fn calculate(&self, input: u32) -> std::vec::Vec<u32> {
        let mut latest: u32 = input; 
        let mut result: std::vec::Vec<u32> = vec![input];

        for i in 0..self.operators.len() {
            match self.operators[i] {
                FlatteningOperator::Add => {
                    latest = latest + self.operands[i + 1].0; // TODO: it may not be correct to ignore the exponent / second tuple value.
                    result[i] = latest;
                },
                FlatteningOperator::Substract => {
                    latest = latest - self.operands[i + 1].0;
                    result[i] = latest;
                },
                FlatteningOperator::Multiply => {
                    latest = latest * self.operands[i + 1].0;
                    result[i] = latest;
                },
                FlatteningOperator::Divide => {
                    latest = latest / self.operands[i + 1].0;
                    result[i] = latest;
                },
            }
        }
        result
    }
}
