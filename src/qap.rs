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
    pub fn add_a(&mut self, a: std::vec::Vec<f32>) {
        self.a.push(Polynomial::from(a));
    }

    pub fn add_b(&mut self, b: std::vec::Vec<f32>) {
        self.b.push(Polynomial::from(b));
    }

    pub fn add_c(&mut self, c: std::vec::Vec<f32>) {
        self.c.push(Polynomial::from(c));
    }

    pub fn evaluate(&self) {
        for x in 1i32..4 {
            for i in 0..6 {
                println!("A[{}]({})={}", i, x, math::round::ceil(  (self.a[i].value[0]*(x.pow(0) as f32) + 
                                                                    self.a[i].value[1]*(x.pow(1) as f32) + 
                                                                    self.a[i].value[2]*(x.pow(2) as f32) + 
                                                                    self.a[i].value[3]*(x.pow(3) as f32)) 
                                                                    as f64, 1) as i32)
            }
        }
        for x in 1i32..4 {
            for i in 0..6 {
                println!("B[{}]({})={}", i, x, math::round::ceil(  (self.b[i].value[0]*(x.pow(0) as f32) + 
                                                                    self.b[i].value[1]*(x.pow(1) as f32) + 
                                                                    self.b[i].value[2]*(x.pow(2) as f32) + 
                                                                    self.b[i].value[3]*(x.pow(3) as f32)) 
                                                                    as f64, 1) as i32)
            }
        }
        for x in 1i32..4 {
            for i in 0..6 {
                println!("C[{}]({})={}", i, x, math::round::ceil(  (self.c[i].value[0]*(x.pow(0) as f32) + 
                                                                    self.c[i].value[1]*(x.pow(1) as f32) + 
                                                                    self.c[i].value[2]*(x.pow(2) as f32) + 
                                                                    self.c[i].value[3]*(x.pow(3) as f32)) 
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

// we're using a wrapper aroudn std::vec::Vec<f32> so we can implement an ops::Add function for the
// Polynomial
#[derive(Clone)]
pub struct Polynomial {
    pub value: std::vec::Vec<f32>,
}

impl From<std::vec::Vec<f32>> for Polynomial {
    fn from(item: std::vec::Vec<f32>) -> Self {
        Polynomial { value: item }
    }
}

// Add<Polynomial>
// TODO: check if this is using references correctly
// TODO: this may not be used at the moment
impl ops::Add for Polynomial {
    // type Output = Polynomial;
    type Output = Self;

    // fn add(self, _rhs: Polynomial) -> Polynomial {
    fn add(self, _rhs: Self) -> Self {
        let mut sum = Polynomial { value: vec![0.0; _rhs.value.len()] };
        for ((sumref, lval), rval) in sum.value.iter_mut().zip(&self.value).zip(&_rhs.value) {
            *sumref = lval + rval;
        }
        sum
    }
}

impl ops::Index<usize> for Polynomial {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

// impl std::iter::FromIterator<Polynomial> for Polynomial {
//     fn from_iter<I: IntoIterator<Item=Polynomial>>(iter: I) -> Self {
//         let mut c = MyCollection::new();

//         for i in iter {
//             c.add(i);
//         }

//         c
//     }
// }


pub fn sum_polynomials(polynomials: std::vec::Vec<Polynomial>) -> Polynomial {
    let mut sum = Polynomial {
        value: vec![0.0; polynomials[0].value.len()],
    };

    for i in 0..sum.value.len() {
        sum.value[i] += polynomials.iter().map(|polynomial| polynomial.value[i]).sum::<f32>();
    }

    sum
}

    // {
    //     iter.fold(Self { x: 0.0, y: 0.0 }, |a, b| Self {
    //         x: a.x + b.x,
    //         y: a.y + b.y,
    //     })
    // }
impl<'a> std::iter::Sum for Polynomial {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>
    {
        iter.fold(Self { value: vec![0.0; 4] }, |a, b| a + b)
        // let mut sum = Polynomial {
        //     // TODO: hardcoded 4 can be replaced by using std::iter::ExactSizeIterator and iter().len()
        //     value: vec![0.0; 4],
        // };

        // while iter.next().is_some() {
        //     sum = sum + iter;
            // for i in 0..4 {
            //     sum.value[i] += iter.collect::<Polynomial>()[i];
            // }
        // }
        // sum

        // for index in iter {
        //     // 0..sum.value.len() {
        //     for j in 0..polynomial[i][j].len() {
        //         sum.value[i] += index.unwrap();
        //         // ter.map(|polynomial| polynomial.value[i]).sum::<f32>();
        //         // sum.value[i] += iter.map(|polynomial| polynomial.value[i]).sum::<f32>();
        //     }
        // }

        // sum
        
        // iter.fold(Polynomial { value: vec![0.0, 4] }, |a, b| 
        //     for i in 0..a.len() {
        //         a.value[i] += b.value[i];
        //     },
        //     // let summy = Polynomial {
        //     //     // TODO: hardcoded 4 can be replaced by using std::iter::ExactSizeIterator and iter().len()
        //     //     value: a.value,
        //     // };
        //     summy
        //     // Polynomial {
        //     //     value: a.value,
        //     // }
        // )
    }
}
