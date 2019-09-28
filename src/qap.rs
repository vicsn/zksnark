use std::ops;
use crate::r1cs::R1CS;

#[derive(Clone)]
pub struct QAP {
    pub a: std::vec::Vec<Polynomial>,
    pub b: std::vec::Vec<Polynomial>,
    pub c: std::vec::Vec<Polynomial>,
}

impl QAP {
    // set vectors
    // TODO: check if this way of passing parameters is proper Rust-like style
    pub fn add_a(&mut self, a: std::vec::Vec<f64>) {
        self.a.push(Polynomial::from(a));
    }

    pub fn add_b(&mut self, b: std::vec::Vec<f64>) {
        self.b.push(Polynomial::from(b));
    }

    pub fn add_c(&mut self, c: std::vec::Vec<f64>) {
        self.c.push(Polynomial::from(c));
    }

    pub fn evaluate(&self) {
        for x in 1i32..4 {
            for i in 0..6 {
                println!("A[{}]({})={}", i, x, math::round::ceil(  (self.a[i].value[0]*(x.pow(0) as f64) + 
                                                                    self.a[i].value[1]*(x.pow(1) as f64) + 
                                                                    self.a[i].value[2]*(x.pow(2) as f64) + 
                                                                    self.a[i].value[3]*(x.pow(3) as f64)) 
                                                                    as f64, 1) as i32)
            }
        }
        for x in 1i32..4 {
            for i in 0..6 {
                println!("B[{}]({})={}", i, x, math::round::ceil(  (self.b[i].value[0]*(x.pow(0) as f64) + 
                                                                    self.b[i].value[1]*(x.pow(1) as f64) + 
                                                                    self.b[i].value[2]*(x.pow(2) as f64) + 
                                                                    self.b[i].value[3]*(x.pow(3) as f64)) 
                                                                    as f64, 1) as i32)
            }
        }
        for x in 1i32..4 {
            for i in 0..6 {
                println!("C[{}]({})={}", i, x, math::round::ceil(  (self.c[i].value[0]*(x.pow(0) as f64) + 
                                                                    self.c[i].value[1]*(x.pow(1) as f64) + 
                                                                    self.c[i].value[2]*(x.pow(2) as f64) + 
                                                                    self.c[i].value[3]*(x.pow(3) as f64)) 
                                                                    as f64, 1) as i32)
            }
        }
    }
}

// TODO: could replace T by Polynomial 
impl R1CS<std::vec::Vec<Polynomial>> for QAP { 
    fn a(&self) -> &std::vec::Vec<Polynomial> {
        &self.a
    }

    fn b(&self) -> &std::vec::Vec<Polynomial> {
        &self.b
    }

    fn c(&self) -> &std::vec::Vec<Polynomial> {
        &self.c
    }
}

// we're using a wrapper aroudn std::vec::Vec<f64> so we can implement an ops::Add function for the
// Polynomial
#[derive(Clone)]
pub struct Polynomial {
    pub value: std::vec::Vec<f64>,
}

impl From<std::vec::Vec<f64>> for Polynomial {
    fn from(item: std::vec::Vec<f64>) -> Self {
        Polynomial { value: item }
    }
}

// TODO: check if this is using references correctly
// TODO: this only works if both polynomials are of the same length
impl ops::Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        assert!(self.value.len() == rhs.value.len());
        let mut sum = Polynomial { value: vec![0.0; rhs.value.len()] };
        for ((sumref, lval), rval) in sum.value.iter_mut().zip(&self.value).zip(&rhs.value) {
            *sumref = lval + rval;
        }
        sum
    }
}

// TODO: this only works if self.len() >= rhs.len()
impl ops::Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        assert!(self.value.len() >= rhs.value.len());
        let mut subtraction = self.clone();
        for (i, rval) in rhs.value.iter().rev().enumerate() {
            subtraction.value[self.value.len() - 1 - i] -= rval;
        }
        subtraction
    }
}

impl ops::Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let max_power = rhs.value.len() + self.value.len() - 2;
        let mut multiplication = Polynomial { value: vec![0.0; max_power + 1] };
        for (i, lval) in self.value.iter().enumerate() {
            for (j, rval) in rhs.value.iter().enumerate() {
                multiplication.value[max_power - i - j] += lval*rval;
            }
        }
        multiplication
    }
}

impl ops::Index<usize> for Polynomial {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl std::iter::Sum for Polynomial {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>
    {
        iter.fold(Self { value: vec![0.0; 4] }, |a, b| a + b)
    }
}
