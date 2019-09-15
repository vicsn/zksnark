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
#[macro_use] extern crate itertools;

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
        gates: vec![],
    };
    
    let vars: usize = 6; // TODO how can we determine the size instead of hardcoding it?
    let mut _gate = Gate { 
        a: vec![0; vars],
        b: vec![0; vars],
        c: vec![0; vars],
    };
    
    let mut last_var_index: usize = 1; // NOTE: the first element is a special integer element
    let mut last_addend: usize = last_var_index;
    
    let mut a: std::vec::Vec<u32> = vec![0; vars]; // TODO how can we determine the size instead of hardcoding it?
    let mut b: std::vec::Vec<u32> = vec![0; vars];
    let mut c: std::vec::Vec<u32> = vec![0; vars];


    for &element in equation.iter() {
        // multipland must be larger than 0
        if element.0 == 0 {
            // std::string::String::from("Error Happens!") // TODO how to return an error
            ()
        }

        // NOTE: skip ahead for multiplicand of 1 and exponent of 1
        
        // TODO: simplifying constraint: we're always adding: (vars[0] or vars[1]) and (vars[i]) and (vars[i+1])
        // add entry for multiplicant bigger than 1, with x
        if element.0 != 1 && element.1 > 0{
            flattened.add_operand((element.0, 0));
            flattened.add_operator(FlatteningOperator::Multiply);
            
            a[0] = element.0;
            b[last_var_index] = 1;
            last_var_index += 1;
            c[last_var_index] = 1;
            flattened.add_gate(a, b, c);
            a = vec![0; vars]; // TODO: this is deadugly, can we reinitialize a, b and c? Perhaps by hiding this in the flattened object
            b = vec![0; vars];
            c = vec![0; vars];
        }
        
        // add entries for exponent bigger than 1
        if element.1 > 1 {
            flattened.add_operand((element.0, 1));
            let mut index: usize = 0; // TODO is there a way to re-use _i below ?
            for _i in 0..(element.1 - 1) {
                flattened.add_operand((element.0, 1));
                flattened.add_operator(FlatteningOperator::Multiply);

                a[last_var_index] = 1;
                b[last_var_index - index] = 1;
                last_var_index += 1;
                c[last_var_index] = 1;
                flattened.add_gate(a, b, c);
                index += 1;
                a = vec![0; vars]; // TODO: this is deadugly, can we reinitialize a, b and c? Perhaps by hiding this in the flattened object
                b = vec![0; vars];
                c = vec![0; vars];
            }
        }
        
        if element != *equation.first().unwrap() { // TODO: this is probably inefficient, compare with iterator location instead of the actual value
            flattened.add_operand((element.0, element.1));
            flattened.add_operator(FlatteningOperator::Add);
            
            a[last_addend] = 1;
            if element.0 > 1 && element.1 == 0 {
                a[0] = element.0;
            } else {
                a[last_var_index] = 1;  
            }
            b[0] = 1;               // NOTE: for an addition gate, the b vector simply multiplies by 1
            last_var_index += 1;
            c[last_var_index] = 1;
            flattened.add_gate(a, b, c);
            last_addend = last_var_index;
            a = vec![0; vars]; // TODO: this is deadugly, can we reinitialize a, b and c? Perhaps by hiding this in the flattened object
            b = vec![0; vars];
            c = vec![0; vars];
        }
    }

    Ok(flattened)
}

fn lagrange_interpolation(coordinates: std::vec::Vec<(u32, u32)>) -> Result<std::vec::Vec<(f32, u32)>, std::string::String> {
    let mut total_function: std::vec::Vec<(f32, u32)> = vec![];
    let mut partial_functions: std::vec::Vec<std::vec::Vec<(f32, u32)>> = vec![];
    
    // create partial quadratic equations when two y coordinates are set to 0
    for i in 0..(coordinates.len()) {
        // cast uints to floats
        let mut adjusted_coordinates: std::vec::Vec<(f32, f32)> = coordinates.iter().map(|&e| (e.0 as f32, e.1 as f32)).collect();
        
        // set two y coordinates to 0
        if i != 0 { adjusted_coordinates[0].1 = 0.0; }
        if i != 1 { adjusted_coordinates[1].1 = 0.0; }
        if i != 2 { adjusted_coordinates[2].1 = 0.0; }
        
        // move non-0 y coordinate to index 0 of vector
        adjusted_coordinates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut multiplier: f32 = adjusted_coordinates[0].1 / (adjusted_coordinates[0].0 * adjusted_coordinates[0].0 - adjusted_coordinates[0].0*adjusted_coordinates[2].0 - adjusted_coordinates[1].0*adjusted_coordinates[0].0 + adjusted_coordinates[1].0*adjusted_coordinates[2].0);

        partial_functions.push(vec![
            (1.0 * multiplier, 2),
            (- (adjusted_coordinates[1].0 + adjusted_coordinates[2].0) * multiplier, 1),
            (adjusted_coordinates[1].0 * adjusted_coordinates[2].0 * multiplier, 0)
        ]);
    }
        
    // print to see if we did it right
    for i in 0..(partial_functions.len()) {
        for j in 0..(partial_functions[i].len()) {
            print!("{}x{} ", partial_functions[i][j].0, partial_functions[i][j].1);
        }
        println!("");
    }

    // sum partial functions
    for (func_1, func_2, func_3) in itertools::izip!(&partial_functions[0], &partial_functions[1], &partial_functions[2]) {
        // NOTE: the second coordinate is redundant information, as we always have 2-degree polynomials
        // check if second coordinates are really equal
        assert_eq!(func_1.1, func_2.1);
        assert_eq!(func_1.1, func_3.1);

        total_function.push((func_1.0 + func_2.0 + func_3.0, func_1.1));
    }
    // print to see if we did it right
    if total_function.len() > 0 {
        println!("{}x{}, {}x{}, {}x{}", total_function[0].0, total_function[0].1, total_function[1].0, total_function[1].1, total_function[2].0, total_function[2].1);
    }

    Ok(total_function)
}
    
fn r1cs_to_qap(flattened: FlattenedEquation) -> Result<std::vec::Vec<u32>, std::string::String> {
    
    // TODO: temporary print to see if we're doing things correctly
    let result = flattened.witness(3);
    for i in 0..(result.len()) {
        print!("{} ", result[i]);
    }
    println!("\n");

    let a = flattened.a();
    
    let tmp = vec![(1,3),(2,2),(3,4)];
    let res = lagrange_interpolation(tmp);
    
    // let mut coordinates: std::vec::Vec<u32> = a[0];
    // vec![];
    // for i in 0..(a.len()) {
    //     coordinates.push(a[i]);
    // }
    
    // TODO: temporary print to see if we're doing things correctly
    // for i in 0..(coordinates.len()) {
    //     print!("{}", coordinates[i]);
    // }
    // println!("");

    // now we're going to do lagrange interpolation on a set of (4 pairs of (x,y) coordinates)
    // where evaluating the polynomial at i gets you the first value of the ith a vector
    // (1,0)
    // (2,0)
    // (3,0)
    // (4,5)
    
    // for coordinates: lagrange_interpolation(coordinates);
    
    // next we simply sum the polynomials
    // let mut sum: std::vec::Vec = vec![0; 6]; // TODO: can we infer 6 from something?
    // for i in 0..(polys.len()) {
    //     sum[i] = polys[i][0] + polys[i][1] + polys[i][2];
    // }
    //

    Ok(result)
    
}

// s . a * s . b - s . c == 0
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
    for equation in equations.iter() {
        let r1 = flatten(equation.to_vec());
        match r1 { // TODO is there another way to do this without cloning?
            Ok(value) => {
                    value.print();
                    r1cs_to_qap(value);
                },
            Err(error) => println!("{}", error),
        }
    }
}
