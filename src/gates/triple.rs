use crate::{carray, GateMatrixType};
use ndarray::prelude::*;

use super::gate::QuantumGate;

macro_rules! gen_gates {
    ($mat: ident) => {
        #[allow(non_snake_case)]
        fn $mat(&mut self, qubit1: usize, qubit2: usize, qubit3: usize) {
            self.add_triple($mat(qubit1, qubit2, qubit3));
        }
    };

    ($($ms: ident),*) => {
        $(gen_gates!($ms);)*
    };
}

pub trait TripleGateApplicator {
    fn add_triple(&mut self, gate: QuantumGate);
    gen_gates!(CCNOT, CSWAP);
}

#[allow(non_snake_case)]
pub fn CCNOT(qubit1: usize, qubit2: usize, qubit3: usize) -> QuantumGate {
    QuantumGate {
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
        target_qubit_index: vec![qubit1, qubit2, qubit3],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    }
}

#[allow(non_snake_case)]
pub fn CSWAP(qubit1: usize, qubit2: usize, qubit3: usize) -> QuantumGate {
    QuantumGate {
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
        target_qubit_index: vec![qubit1, qubit2, qubit3],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    }
}
