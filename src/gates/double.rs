use crate::{carray, carray_i, GateMatrixType};
use ndarray::prelude::*;

use super::gate::QuantumGate;

macro_rules! gen_gates {
    ($mat: ident) => {
        #[allow(non_snake_case)]
        fn $mat(&mut self, qubit1: usize, qubit2: usize) {
            self.add_double($mat(qubit1, qubit2));
        }
    };

    ($($ms: ident),*) => {
        $(gen_gates!($ms);)*
    };
}

pub trait DoubleGateApplicator {
    fn add_double(&mut self, gate: QuantumGate);
    gen_gates!(CNOT, SWAP, SQSWAP);
}

#[allow(non_snake_case)]
pub fn CNOT(qubit1: usize, qubit2: usize) -> QuantumGate {
    QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 0., 1.],
            [0., 0., 1., 0.]
        ],
        target_qubit_index: vec![qubit1, qubit2],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    }
}

#[allow(non_snake_case)]
pub fn SWAP(qubit1: usize, qubit2: usize) -> QuantumGate {
    QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![
            [1., 0., 0., 0.],
            [0., 0., 1., 0.],
            [0., 1., 0., 0.],
            [0., 0., 0., 1.]
        ],
        target_qubit_index: vec![qubit1, qubit2],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    }
}

#[allow(non_snake_case)]
pub fn SQSWAP(qubit1: usize, qubit2: usize) -> QuantumGate {
    QuantumGate {
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
        target_qubit_index: vec![qubit1, qubit2],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    }
}
