use crate::{carray, carray_i, GateMatrixType};
use ndarray::prelude::*;

use super::gate::QuantumGate;

macro_rules! gen_gates {
    ($mat: ident) => {
        #[allow(non_snake_case)]
        fn $mat(&mut self, qubit: usize) {
            self.add_single($mat(qubit))
        }
    };

    // 横展開用
    ($($ms: ident),*) => {
        $(gen_gates!($ms);)*
    };
}

pub trait SingleGateApplicator {
    fn add_single(&mut self, gate: QuantumGate);
    gen_gates!(H, X, Y, Z, ID, SQNOT);
}

#[allow(non_snake_case)]
pub fn X(target_qubit: usize) -> QuantumGate {
    QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[0., 1.], [1., 0.]],
        target_qubit_index: vec![target_qubit],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    }
}

#[allow(non_snake_case)]
pub fn H(target_qubit: usize) -> QuantumGate {
    QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[1., 1.], [1., -1.]] / (2f64).sqrt(),
        target_qubit_index: vec![target_qubit],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    }
}

#[allow(non_snake_case)]
pub fn Y(target_qubit: usize) -> QuantumGate {
    QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray_i![[0., 1.], [-1., 0.]],
        target_qubit_index: vec![target_qubit],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    }
}

#[allow(non_snake_case)]
pub fn Z(target_qubit: usize) -> QuantumGate {
    QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[1., 0.], [0., -1.]],
        target_qubit_index: vec![target_qubit],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    }
}

#[allow(non_snake_case)]
pub fn ID(target_qubit: usize) -> QuantumGate {
    QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[1., 0.], [0., 1.]],
        target_qubit_index: vec![target_qubit],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    }
}

#[allow(non_snake_case)]
pub fn SQNOT(target_qubit: usize) -> QuantumGate {
    QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[1., 1.], [1., 1.]] / 2. + carray_i![[1., -1.], [-1., 1.]] / 2.,
        target_qubit_index: vec![target_qubit],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    }
}
