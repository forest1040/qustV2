use crate::{carray, GateMatrixType};
use ndarray::prelude::*;
use num::complex::Complex;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct TripleGate {
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
        fn $mat(&mut self, qubit1: usize, qubit2: usize, qubit3: usize) {
            self.apply_triple(&$mat.matrix, qubit1, qubit2, qubit3);
        }
    };

    ($($ms: ident),*) => {
        $(gen_gates!($ms);)*
    };
}

pub trait TripleGateApplicator {
    fn apply_triple(
        &mut self,
        matrix: &Array2<Complex<f64>>,
        qubit1: usize,
        qubit2: usize,
        qubit3: usize,
    );

    gen_gates!(CCNOT, CSWAP);
}

pub static CCNOT: Lazy<TripleGate> = {
    Lazy::new(|| TripleGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![
            [1., 0., 0., 0., 0., 0., 0., 0.],
            [0., 1., 0., 0., 0., 0., 0., 0.],
            [0., 0., 1., 0., 0., 0., 0., 0.],
            [0., 0., 0., 1., 0., 0., 0., 0.],
            [0., 0., 0., 0., 1., 0., 0., 0.],
            [0., 0., 0., 0., 0., 1., 0., 0.],
            [0., 0., 0., 0., 0., 0., 0., 1.],
            [0., 0., 0., 0., 0., 0., 1., 0.]
        ],
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
pub static CSWAP: Lazy<TripleGate> = {
    Lazy::new(|| TripleGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![
            [1., 0., 0., 0., 0., 0., 0., 0.],
            [0., 1., 0., 0., 0., 0., 0., 0.],
            [0., 0., 1., 0., 0., 0., 0., 0.],
            [0., 0., 0., 1., 0., 0., 0., 0.],
            [0., 0., 0., 0., 1., 0., 0., 0.],
            [0., 0., 0., 0., 0., 0., 1., 0.],
            [0., 0., 0., 0., 0., 1., 0., 0.],
            [0., 0., 0., 0., 0., 0., 0., 1.]
        ],
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
