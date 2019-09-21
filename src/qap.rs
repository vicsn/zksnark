pub struct QAP {
    pub A: std::vec::Vec<std::vec::Vec<f32>>,
    pub B: std::vec::Vec<std::vec::Vec<f32>>,
    pub C: std::vec::Vec<std::vec::Vec<f32>>,
}

impl QAP {
    // set vectors
    // TODO: check if this way of passing parameters is proper Rust-like style
    pub fn add_a(&mut self, a: std::vec::Vec<f32>) {
        self.A.push(a);
    }

    pub fn add_b(&mut self, b: std::vec::Vec<f32>) {
        self.B.push(b);
    }

    pub fn add_c(&mut self, c: std::vec::Vec<f32>) {
        self.C.push(c);
    }

    pub fn A(&self) -> &std::vec::Vec<std::vec::Vec<f32>> {
        &self.A
    }

    pub fn B(&self) -> &std::vec::Vec<std::vec::Vec<f32>> {
        &self.B
    }

    pub fn C(&self) -> &std::vec::Vec<std::vec::Vec<f32>> {
        &self.C
    }

    pub fn evaluate(&self) {
        for x in 0i32..4 {
            for i in 0..6 {
                println!("A[{}]({})={}", i, x, math::round::ceil(  (self.A[i][0]*(x.pow(0) as f32) + 
                                                                    self.A[i][1]*(x.pow(1) as f32) + 
                                                                    self.A[i][2]*(x.pow(2) as f32) + 
                                                                    self.A[i][3]*(x.pow(3) as f32)) 
                                                                    as f64, 1) as i32)
            }
        }
        for x in 0i32..4 {
            for i in 0..6 {
                println!("B[{}]({})={}", i, x, math::round::ceil(  (self.B[i][0]*(x.pow(0) as f32) + 
                                                                    self.B[i][1]*(x.pow(1) as f32) + 
                                                                    self.B[i][2]*(x.pow(2) as f32) + 
                                                                    self.B[i][3]*(x.pow(3) as f32)) 
                                                                    as f64, 1) as i32)
            }
        }
        for x in 0i32..4 {
            for i in 0..6 {
                println!("C[{}]({})={}", i, x, math::round::ceil(  (self.C[i][0]*(x.pow(0) as f32) + 
                                                                    self.C[i][1]*(x.pow(1) as f32) + 
                                                                    self.C[i][2]*(x.pow(2) as f32) + 
                                                                    self.C[i][3]*(x.pow(3) as f32)) 
                                                                    as f64, 1) as i32)
            }
        }
    }
}
