//! # zksnark_tutorial
//!
//! `zksnark_tutorial` aims to describe how to create zksnarks to prove statements 
//! 
//! proves knowledge of: x**3 + x + 5 == 35
//!
//! # Sources
//!
//! * [quadratic-arithmetic-programs-from-zero-to-hero](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649)
//! 


fn qeval(x: i32) -> i32 {
    let y = x.pow(3);
    y + x + 5
}

// The first step is a “flattening” procedure, where we convert the original code, which may
// contain arbitrarily complex statements and expressions, into a sequence of statements that are
// of two forms:
// (1) x = y (where y can be a variable or a number)
// (2) x = y (op) z (where op can be +, -, *, / and y and z can be variables, numbers or themselves
// sub-expressions)
// 
// [(multiplicant,power)]

///
/// Flatten an equation
///
/// # arguments
///
/// * `equation` - the polynomial equation to flatten, represented as a Vec<i32>
/// 
/// # return value
///
/// The flattened equation
pub fn flattening(equation: std::vec::Vec<(i32, i32)>) -> std::vec::Vec<(i32, i32)> {
//     for element in equation.iter() {
//         if (element == 2) {
//             // element.1;
//         }
        let tmp: std::vec::Vec<(i32, i32)> = vec![(1,5)];
        tmp
//         // let sym_1 = x * x
//         // let y = sym_1 * x
//         // let sym_2 = y + x
//         // let out = sym_2 + 5
//     }
}

fn main() {
    println!("Hello, world!");

    let _a = 5;
    let a: i32 = 6;
    let b = qeval(a);
    println!("{} - {}", a, b);
    
    let equations: std::vec::Vec<std::vec::Vec<(i32, i32)>> = vec![vec![(1,3),(1,1),(5,0)]];
    let mut flattened = Vec::new();
    for equation in equations.iter() {
        flattened.push(flattening(equation.to_vec()));
        for element in equation.iter() {
            println!("{}x**{}", element.0, element.1);
        }
    }
}
