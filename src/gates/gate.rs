use ndarray::Array2;
use num::Complex;

use crate::{state::QuantumState, GateMatrixType};

#[derive(Debug)]
pub struct QuantumGate {
    pub matrix_type: GateMatrixType,
    pub pauli_id: Vec<usize>,
    pub rotation_angle: f64,
    pub matrix: Array2<Complex<f64>>,
    pub target_qubit_index: Vec<usize>,
    pub control_qubit_index: Vec<usize>,
}

pub trait GateOp {
    //fn set_target_qubit(&mut self, target_qubit: usize);
    fn update_quantum_state(&self, state: &mut QuantumState);
    fn add_control_qubit(&mut self, control_qubit: usize);
}

impl GateOp for QuantumGate {
    // fn set_target_qubit(&mut self, target_qubit: usize) {
    //     self.target_qubit_index.push(target_qubit);
    // }
    fn update_quantum_state(&self, state: &mut QuantumState) {
        state.apply(
            &self.control_qubit_index,
            &self.target_qubit_index,
            &self.matrix,
        );
    }
    fn add_control_qubit(&mut self, control_qubit: usize) {
        self.control_qubit_index.push(control_qubit);
    }
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
