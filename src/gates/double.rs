use crate::{carray, carray_i, GateMatrixType};
use ndarray::prelude::*;
use num::complex::Complex;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct DoubleGate {
    matrix_type: GateMatrixType,
    pauli_id: Vec<usize>,
    rotation_angle: f64,
    matrix: Array2<Complex<f64>>,
    target_qubit_index: Vec<usize>,
    control_qubit_index: Vec<usize>,
}

macro_rules! gen_gates {
    ($mat: ident) => {
        #[allow(non_snake_case)]
        fn $mat(&mut self, qubit1: usize, qubit2: usize) {
            self.apply_double(&$mat.matrix, qubit1, qubit2);
        }
    };

    ($($ms: ident),*) => {
        $(gen_gates!($ms);)*
    };
}

pub trait DoubleGateApplicator {
    fn apply_double(&mut self, matrix: &Array2<Complex<f64>>, qubit1: usize, qubit2: usize);

    gen_gates!(CNOT, SWAP, SQSWAP);
}

pub static CNOT: Lazy<DoubleGate> = {
    Lazy::new(|| DoubleGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 0., 1.],
            [0., 0., 1., 0.]
        ],
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
pub static SWAP: Lazy<DoubleGate> = {
    Lazy::new(|| DoubleGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![
            [1., 0., 0., 0.],
            [0., 0., 1., 0.],
            [0., 1., 0., 0.],
            [0., 0., 0., 1.]
        ],
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
pub static SQSWAP: Lazy<DoubleGate> = {
    Lazy::new(|| DoubleGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![
            [1., 0., 0., 0.],
            [0., 0.5, 0.5, 0.],
            [0., 0.5, 0.5, 0.],
            [0., 0., 0., 1.]
        ] + carray_i![
            [0., 0., 0., 0.],
            [0., 0.5, -0.5, 0.],
            [0., -0.5, 0.5, 0.],
            [0., 0., 0., 0.]
        ],
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
