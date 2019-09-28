mod gates;
mod r1cs;
mod qap;
extern crate peroxide;
#[macro_use] extern crate itertools;

#[cfg(test)]
mod tests {
    use crate::gates::*;
    use crate::qap::*;
    use crate::r1cs::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn polynomial_adds() {
        let a = Polynomial { value: vec![2.0, 4.0, 6.0] };
        let b = Polynomial { value: vec![3.0, 5.0, 7.0] };
        let sum = a + b;
        assert!(sum.value == vec![5.0, 9.0, 13.0]);
    }

}
