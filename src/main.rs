//! # zksnark_tutorial
//!
//! `zksnark_tutorial` aims to describe how to create zksnarks to prove statements 
//! 
//! proves knowledge of: x**3 + x + 5 == 35
//!
//! # Sources
//!
//! * [quadratic-arithmetic-programs-from-zero-to-hero](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649)
//! * [matrix-rank](https://stattrek.com/matrix-algebra/matrix-rank.aspx)
//! 

mod types;
use crate::types::*;

/// Evaluate x**3 + x + 5
///
/// # arguments
///
/// * `x` - an integer
/// 
/// # return value
///
/// * x**3 + x + 5
fn qeval(x: u32) -> u32 {
    let y = x.pow(3);
    y + x + 5
}


/// We convert the original code, which may contain arbitrarily complex statements and expressions,
/// into a sequence of statements that are of two forms:
/// (1) x = y (where y can be a variable or a number)
/// (2) x = y (op) z (where op can be +, -, *, / and y and z can be variables, numbers or themselves
/// sub-expressions)
///
/// # arguments
///
/// * `equation` - the polynomial equation to flatten, represented as a Vec<(multicpliant,power)>
/// 
/// # return value
///
/// * The flattened equation
// TODO: we don't support minus or division yet
fn flatten(equation: std::vec::Vec<(u32, u32)>) -> Result<FlattenedEquation, std::string::String> {
    // let mut flattened: std::vec::Vec<std::vec::Vec<(u32, u32)>> = vec![vec![]];
    let mut flattened = FlattenedEquation {
        operands: vec![],
        operators: vec![],
    };

    for &element in equation.iter() { // TODO why are we using a reference here?
        // multipland must be larger than 0
        if element.0 == 0 {
            // std::string::String::from("Error Happens!") // TODO how to return an error
            ()
        }

        // add entry for multiplicand bigger than 1
        if element.0 != 1 {
            flattened.add_operand((element.0, 0));
            flattened.add_operator(FlatteningOperator::Multiply);
        }
        
        // add entry for exponent of 1 or 0
        if element.1 == 0 || element.1 == 1 {
            flattened.add_operand((element.0, 1));
        
        // add entry for exponent bigger than 1
        } else {
            flattened.add_operand((element.0, 1));
            for _i in 1..(element.1 - 1) {
                flattened.add_operand((element.0, 1));
                flattened.add_operator(FlatteningOperator::Multiply);
            }
        }
        
        if element != *equation.last().unwrap() { // TODO: this is probably inefficient?
            flattened.add_operator(FlatteningOperator::Add);
        } else {
            println!("not adding!");
        }
    }

    Ok(flattened)
}

// From:    x**3 + x + 5
// To:      [(1,3),(1,1),(5,0)]; 
// To:      [((1,1),(1,1),(1,1)),(1,1),(5,0)]

// given: (1,3),(1,1)(5,0)
// (1)
// let one = 1              //                  //
// let sym_1 = x * x        // [1, '*', (1,1)]  // 3 = 1 * 1
// let y = sym_1 * x        // [2, '*', (1,1)]  // 4 = 1 * 3
// let sym_2 = y + x        // [3, '+', (1,1)]  // 5 = 4 + 1
// let out = sym_2 + 5      // [4, '+', (5,0)]  // 2 = 5 + 5
// 
// (2) '~one', 'x', '~out', 'sym_1', 'y', 'sym_2'
fn gates_to_r1cs(flattened: FlattenedEquation) -> Result<std::vec::Vec<u32>, std::string::String> {
    
    let result = flattened.calculate(3);
    Ok(result)
    
    // let triples = vec![vec![0; flattened_equation.operands.len() - 1]];
    // let outgoing_var_index = 3; // the first three indexes are already set // TODO: improve this explanation
    // for &operand in flattened.operands.iter() {
    //     let a = vec![0; params];
    //     let b = vec![0; params];
    //     let c = vec![0; params];
        
    //     for &multiplicand in addend.iter() {
    //         let a_m = vec![0; params];
    //         let b_m = vec![0; params];
    //         let c_m = vec![0; params];
            
    //         // if multiplicand.1 == 1 {
    //         //     a[1] = element.0;
    //         // } else if element.1 == 0 {
    //         //     a[1] = element.0;
    //         // } else {
    //         //     () // TODO: error
    //         // }

    //     }
    // }
    
    // let a = vec![0, 1, 0, 0, 0, 0];
    // let b = vec![0, 1, 0, 0, 0, 0];
    // let c = vec![0, 0, 0, 1, 0, 0];

    // let a = vec![0, 0, 0, 1, 0, 0];
    // let b = vec![0, 1, 0, 0, 0, 0];
    // let c = vec![0, 0, 0, 0, 1, 0];
    
    // let a = vec![0, 1, 0, 0, 1, 0];
    // let b = vec![1, 0, 0, 0, 0, 0];
    // let c = vec![0, 0, 0, 0, 0, 1];
    
    // let a = vec![5, 0, 0, 0, 0, 1];
    // let b = vec![1, 0, 0, 0, 0, 0];
    // let c = vec![0, 0, 1, 0, 0, 0];
    
    // let r1cs = vec![1, 3, 35, 9, 27, 30];
}

// s . a * s . b - s . c = 0
fn solve_constraint(a: std::vec::Vec<u32>, b: std::vec::Vec<u32>, c: std::vec::Vec<u32>, s: std::vec::Vec<u32>) -> bool {
    let r1: u32 = s.iter().zip(a.iter()).map(|(x, y)| x * y).sum();
    let r2: u32 = s.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let r3: u32 = s.iter().zip(c.iter()).map(|(x, y)| x * y).sum();
    r1 * r2 - r3 == 0
}

// s . a * s . b - s . c = 0
fn validate_constraints(a: std::vec::Vec<u32>, b: std::vec::Vec<u32>, c: std::vec::Vec<u32>, s: std::vec::Vec<u32>) -> bool {
    let r1: u32 = s.iter().zip(a.iter()).map(|(x, y)| x * y).sum();
    let r2: u32 = s.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let r3: u32 = s.iter().zip(c.iter()).map(|(x, y)| x * y).sum();
    r1 * r2 - r3 == 0
}

fn main() {
    let b = qeval(3);
    println!("{}", b);
    
    let equations: std::vec::Vec<std::vec::Vec<(u32, u32)>> = vec![vec![(1,3),(1,1),(5,0)]];
    let mut flattened = Vec::new();
    for equation in equations.iter() {
        let r1 = flatten(equation.to_vec());
        match r1 { // TODO is there another way to do this without cloning?
            Ok(value) => {
                    value.print();
                    flattened.push(value);
                    // let r2 = gates_to_r1cs(value);
                },
            Err(error) => println!("{}", error),
        }
    }
}
