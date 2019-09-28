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

mod gates;
mod r1cs;
mod qap;
use crate::gates::*;
use crate::qap::*;
use crate::r1cs::*;
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

// lagrange interpolation
fn interpolate(coordinates: std::vec::Vec<(u32, u32)>) -> std::vec::Vec<f32> {
    let mut total_function: std::vec::Vec<f32> = vec![];
    let mut partial_functions: std::vec::Vec<std::vec::Vec<f32>> = vec![];
    
    // create partial quadratic equations when two y coordinates are set to 0
    for i in 0..(coordinates.len()) {
        // cast uints to floats
        let mut mapped: std::vec::Vec<(f32, f32)> = coordinates.iter().map(|&e| (e.0 as f32, e.1 as f32)).collect();
        
        // set all except for one y coordinate to 0
        for j in 0..(mapped.len()) {
            if i != j { mapped[j].1 = 0.0; }
        }

        // move non-zero y coordinate to index 0 of vector
        mapped.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // calculate divisor: summation_i((X0 - Xi)) for i bigger than 0
        let mut divisor: f32 = 1.0;
        for j in 1..(mapped.len()) {
            divisor = divisor * (mapped[0].0 - mapped[j].0);    
        }

        // TODO: this only works for mapped.len() == 4
        // calculate multiplier: Y0 * multiplication_i((X - Xi)) for i bigger than 0
        // let mut multipliers: std::vec::Vec<f32> = vec![
        //     1.0,
        //     - (mapped[1].0 + mapped[2].0),
        //       (mapped[1].0 * mapped[2].0)
        // ];
        let multipliers: std::vec::Vec<f32> = vec![
            - (mapped[1].0 * mapped[2].0 * mapped[3].0),                                  // x0
              (mapped[1].0 * mapped[2].0) + (mapped[3].0 * (mapped[1].0 + mapped[2].0)),  // x1
            - (mapped[1].0 + mapped[2].0 + mapped[3].0),                                  // x2
            1.0                                                                           // x3
        ];

        // create and push partial function
        let mut partial_func: std::vec::Vec<f32> = vec![];
        for j in 0..(mapped.len()) {
            partial_func.push(mapped[0].1 * multipliers[j]/divisor);
        }
        partial_functions.push(partial_func);
    }
        
    // sum partial functions
    for (func_1, func_2, func_3, func_4) in itertools::izip!(&partial_functions[0], &partial_functions[1], &partial_functions[2], &partial_functions[3]) {
        total_function.push(func_1 + func_2 + func_3 + func_4);
    }

    // print to see if we did it right
    if total_function.len() > 0 {
        println!("{} {}x1 {}x2 {}x3", total_function[0], total_function[1], total_function[2], total_function[3]);
    }

    total_function
}
    
fn r1cs_to_qap(flattened: FlattenedEquation) -> QAP {
    let mut qap = QAP {
        a: vec![],
        b: vec![],
        c: vec![],
    };

    // now we're going to do lagrange interpolation on a set of (4 pairs of (x,y) coordinates)
    // where evaluating the polynomial at i gets you the first value of the ith a vector
    let a = flattened.a();
    let b = flattened.b();
    let c = flattened.c();
    for i in 0..6 {
        qap.add_a(interpolate(vec![(1, a[0][i]), (2, a[1][i]), (3, a[2][i]), (4, a[3][i])]));
        qap.add_b(interpolate(vec![(1, b[0][i]), (2, b[1][i]), (3, b[2][i]), (4, b[3][i])]));
        qap.add_c(interpolate(vec![(1, c[0][i]), (2, c[1][i]), (3, c[2][i]), (4, c[3][i])]));
    }

    qap
}

// s . a * s . b - s . c == 0
fn validate_constraints(a: std::vec::Vec<u32>, b: std::vec::Vec<u32>, c: std::vec::Vec<u32>, s: std::vec::Vec<u32>) -> bool {
    let r1: u32 = s.iter().zip(a.iter()).map(|(x, y)| x * y).sum();
    let r2: u32 = s.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let r3: u32 = s.iter().zip(c.iter()).map(|(x, y)| x * y).sum();
    r1 * r2 - r3 == 0
}


// t = A . s * B . s - C . s
// first argument 1: r1cs: R1CS or FlattenedEquation?
// first argument 2: qap: QAP
// TODO: we may not need to require the Clone trait
// TODO: ensure this works for both types of R1CS. See the function above
// fn evaluate_r1cs<U: Iter<U>, T: Clone + R1CS<U>>(vectors: T, s: std::vec::Vec<u32>) -> bool {
fn evaluate_r1cs(vectors: QAP, s: std::vec::Vec<u32>) -> bool {
    let r1 = s.iter().zip(vectors.a().iter()).map(|(x, y)| vec![(*x as f32)*y[0], (*x as f32)*y[1], (*x as f32)*y[2], (*x as f32)*y[3]]).collect::<Vec<_>>();//sum();
    let mut res: std::vec::Vec<Polynomial> = vec![];
    for i in 0..r1.len() {
        res.push(Polynomial { value: r1[i].clone() });
    }
    let sum = sum_polynomials(res);
   
    // let r2: std::vec::Vec<f32> = s.iter().zip(vectors.b().iter()).map(|(x, y)| vec![(x as f32)*y[0], x*y[1], x*y[2], x*y[3]]).sum();
    // let r3: std::vec::Vec<f32> = s.iter().zip(vectors.c().iter()).map(|(x, y)| vec![(x as f32)*y[0], x*y[1], x*y[2], x*y[3]]).sum();
    // let v1 = qap.a[0].clone();
    // let v2 = qap.a[0].clone();
    // let v3 = v1 + v2;

    // println!("---------------------------------");
    // for i in 0..qap.a[0].value.len() {
    //     println!("{} + {} = {}", qap.a[0].value[i], qap.a[0].value[i], v3.value[i]);
    // }

    // r1 * r2 - r3 == t / Z
    // A . s = [43.0, -73.333, 38.5, -5.166]
    // B . s = [-3.0, 10.333, -5.0, 0.666]
    // C . s = [-41.0, 71.666, -24.5, 2.833]
    true
}

fn main() {
    let b = qeval(3);
    println!("{}", b);
    
    let equation: std::vec::Vec<(u32, u32)> = vec![(1,3),(1,1),(5,0)];
    let flattened_res = flatten(equation.to_vec());
    let evaluation = match flattened_res { // TODO is there another way to do this without cloning?
        Ok(value) => {
            value.print();
            let witness = value.witness(3);
            let r1cs = r1cs_to_qap(value);
            evaluate_r1cs(r1cs, witness)
        },
        Err(error) => {
            println!("{}", error);
            false
        }
    };
    println!("Evaluation: {}", evaluation);
    
}
