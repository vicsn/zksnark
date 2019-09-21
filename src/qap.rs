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
}
