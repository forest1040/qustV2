use crate::{carray, carray_i, state::QuantumState, GateMatrixType};
use ndarray::prelude::*;
use num::complex::Complex;
use once_cell::sync::Lazy;

use super::gate::QuantumGate;

macro_rules! gen_gates {
    ($mat: ident) => {
        #[allow(non_snake_case)]
        fn $mat(&mut self, qubit: usize) {
            //self.add_single(&$mat.matrix, qubit)
            //$mat.target_qubit_index = vec![qubit];
            self.add_single($mat(qubit))
        }
    }; // ($($ms: ident),*) => {
       //     $(gen_gates!($ms);)*
       // };
}

// pub trait SingleGate {
//     fn set_target_qubit(&mut self, target_qubit: usize);
//     fn update_quantum_state(&mut self, state: &mut QuantumState);
//     fn add_control_qubit(&mut self, control_qubit: usize);
// }

// impl SingleGate for QuantumGate {
//     fn set_target_qubit(&mut self, target_qubit: usize) {
//         self.target_qubit_index.push(target_qubit);
//     }
//     fn update_quantum_state(&mut self, state: &mut QuantumState) {
//         state.apply(
//             &self.control_qubit_index,
//             &self.target_qubit_index,
//             &self.matrix,
//         );
//     }
//     fn add_control_qubit(&mut self, control_qubit: usize) {
//         self.control_qubit_index.push(control_qubit);
//     }
// }

pub trait SingleGateApplicator {
    fn add_single(&mut self, gate: QuantumGate);
    fn update_quantum_state(&mut self);
    //gen_gates!(H, X, Y, Z, ID);
    gen_gates!(X);
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
//pub static X: Lazy<QuantumGate> = Lazy::new(|| Xgate(0));

pub static H: Lazy<QuantumGate> = {
    Lazy::new(|| QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[1., 1.], [1., -1.]] / (2f64).sqrt(),
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};

pub static Y: Lazy<QuantumGate> = {
    Lazy::new(|| QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray_i![[0., 1.], [-1., 0.]],
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
pub static Z: Lazy<QuantumGate> = {
    Lazy::new(|| QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[1., 0.], [0., -1.]],
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
pub static ID: Lazy<QuantumGate> = {
    Lazy::new(|| QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[1., 0.], [0., 1.]],
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
pub static SQNOT: Lazy<QuantumGate> = {
    Lazy::new(|| QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[1., 1.], [1., 1.]] / 2. + carray_i![[1., -1.], [-1., 1.]] / 2.,
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
