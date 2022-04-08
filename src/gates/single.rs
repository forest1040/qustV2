use crate::{carray, carray_i, GateMatrixType, Qubit};
use ndarray::prelude::*;
use num::complex::Complex;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct SingleGate {
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
        fn $mat(&mut self, qubit: &Qubit) {
            self.apply_single(&$mat.matrix, qubit);
        }
    };

    ($($ms: ident),*) => {
        $(gen_gates!($ms);)*
    };
}

// macro_rules! gen_gates_with_ctrl {
//     ($mat: ident) => {
//         #[allow(non_snake_case)]
//         fn $mat(&mut self, qubit_ctrl: &Qubit, qubit: &Qubit) {
//             self.apply_single_with_ctrl(&$mat.matrix, qubit_ctrl, qubit);
//         }
//     };

//     ($($ms: ident),*) => {
//         $(gen_gates_with_ctrl!($ms);)*
//     };
// }

pub trait SingleGateApplicator {
    fn apply_single(&mut self, matrix: &Array2<Complex<f64>>, qubit: &Qubit);

    fn apply_single_with_ctrl(
        &mut self,
        matrix: &Array2<Complex<f64>>,
        qubit_ctrl: &Qubit,
        qubit: &Qubit,
    );

    gen_gates!(H, X, Y, Z, ID);

    //gen_gates_with_ctrl!(X_C);
}

pub static H: Lazy<SingleGate> = {
    Lazy::new(|| SingleGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[1., 1.], [1., -1.]] / (2f64).sqrt(),
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
pub static X: Lazy<SingleGate> = {
    Lazy::new(|| SingleGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[0., 1.], [1., 0.]],
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
// pub static X_C: Lazy<SingleGate> = {
//     Lazy::new(|| SingleGate {
//         matrix: carray![[0., 1.], [1., 0.]],
//     })
// };
pub static Y: Lazy<SingleGate> = {
    Lazy::new(|| SingleGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray_i![[0., 1.], [-1., 0.]],
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
pub static Z: Lazy<SingleGate> = {
    Lazy::new(|| SingleGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[1., 0.], [0., -1.]],
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
pub static ID: Lazy<SingleGate> = {
    Lazy::new(|| SingleGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[1., 0.], [0., 1.]],
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
pub static SQNOT: Lazy<SingleGate> = {
    Lazy::new(|| SingleGate {
        matrix_type: GateMatrixType::DenseMatrix,
        matrix: carray![[1., 1.], [1., 1.]] / 2. + carray_i![[1., -1.], [-1., 1.]] / 2.,
        target_qubit_index: vec![],
        control_qubit_index: vec![],
        pauli_id: vec![],
        rotation_angle: 0.,
    })
};
