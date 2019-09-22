pub trait R1CS<T> {
    fn a(&self) -> &T;
    fn b(&self) -> &T;
    fn c(&self) -> &T;
}
