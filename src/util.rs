extern crate peroxide;

pub fn reverse_poly(poly: peroxide::structure::polynomial::Polynomial) -> peroxide::structure::polynomial::Polynomial {
    let mut reversed: std::vec::Vec<f64> = vec![0 as f64; poly.coef.len()];
    for i in 0..(poly.coef.len()) {
        reversed[poly.coef.len() - 1 - i] = poly.coef[i];
    }
    return peroxide::poly(reversed);
}
