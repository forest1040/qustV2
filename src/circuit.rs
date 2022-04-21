use ndarray::Array2;
use num::Complex;

use crate::{gates::single::SingleGateApplicator, state::QuantumState};

#[derive(Debug)]
pub struct QuantumCircuit {
    state: QuantumState,
}

impl QuantumCircuit {
    pub fn new(n: usize) -> QuantumCircuit {
        let state = QuantumState::new(n);
        QuantumCircuit { state }
    }
}

impl SingleGateApplicator for QuantumCircuit {
    fn apply_single(&mut self, matrix: &Array2<Complex<f64>>, target_qubit: usize) {
        //self.states.apply(&[target_qubit], matrix);
    }
}
