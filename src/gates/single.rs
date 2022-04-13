use crate::{carray, carray_i, GateMatrixType, Qubit};
use ndarray::prelude::*;
use num::complex::Complex;
use once_cell::sync::Lazy;

use super::gate::QuantumGate;

macro_rules! gen_gates {
    ($mat: ident) => {
        #[allow(non_snake_case)]
        fn $mat(&mut self, qubit: &Qubit) {
            //self.apply_single(&$mat.matrix, qubit)
        }
        // fn update_quantum_state() {

        // }
    };

    ($($ms: ident),*) => {
        $(gen_gates!($ms);)*
    };
}

pub trait SingleGate {
    //fn apply_single(&mut self, matrix: &Array2<Complex<f64>>, qubit: &Qubit) -> &mut QuantumGate;
    fn set_target_qubit(&mut self, qubit: &Qubit);
}

impl SingleGate for QuantumGate {
    // fn apply_single(&mut self, matrix: &Array2<Complex<f64>>, qubit: &Qubit) -> &mut QuantumGate {
    //     //apply(&[qubit], matrix);
    //     self.target_qubit_index.push(qubit.index);
    //     self
    // }
    fn set_target_qubit(&mut self, qubit: &Qubit) {
        self.target_qubit_index.push(qubit.index);
    }
}

pub trait SingleGateApplicator {
    fn apply_single(&mut self, matrix: &Array2<Complex<f64>>, qubit: &Qubit);
    gen_gates!(H, X, Y, Z, ID);
}

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

pub fn X() -> QuantumGate {
    QuantumGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[0., 1.], [1., 0.]],
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    }
}

// pub static X: Lazy<QuantumGate> = {
//     Lazy::new(|| QuantumGate {
//         matrix_type: GateMatrixType::DenseMatrix,
//         matrix: carray![[0., 1.], [1., 0.]],
//         target_qubit_index: vec![],
//         control_qubit_index: vec![],
//         pauli_id: vec![],
//         rotation_angle: 0.,
//     })
// };
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