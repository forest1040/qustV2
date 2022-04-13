use ndarray::Array2;
use num::Complex;

use crate::GateMatrixType;

#[derive(Debug)]
pub struct QuantumGate {
    pub matrix_type: GateMatrixType,
    pub pauli_id: Vec<usize>,
    pub rotation_angle: f64,
    pub matrix: Array2<Complex<f64>>,
    pub target_qubit_index: Vec<usize>,
    pub control_qubit_index: Vec<usize>,
}

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
