pub mod gates;
pub mod state;

pub struct Qubit {
    pub index: usize,
}
#[derive(Debug)]
pub enum PauliID {
    I,
    X,
    Y,
    Z,
}
#[derive(Debug)]
pub enum GateMatrixType {
    DenseMatrix,
    PauliMatrix(PauliID),
}

pub struct QuantumGate {}

#[macro_export]
macro_rules! carray {
    ( $([$($x: expr),*]),* ) => {{
        use num::complex::Complex;
        array![
            $([$(Complex::new($x, 0.)),*]),*
        ]
    }};
}

#[macro_export]
macro_rules! carray_i {
    ( $([$($x: expr),*]),* ) => {{
        use num::complex::Complex;
        array![
            $([$(Complex::new(0., $x)),*]),*
        ]
    }};
}
