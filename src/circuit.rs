use std::fmt;

use crate::{
    gates::{
        gate::{GateOp, QuantumGate},
        single::SingleGateApplicator,
    },
    state::QuantumState,
};

#[derive(Debug)]
pub struct QuantumCircuit {
    state: QuantumState,
    gate_list: Vec<QuantumGate>,
}

impl QuantumCircuit {
    pub fn new(n: usize) -> QuantumCircuit {
        let state = QuantumState::new(n);
        QuantumCircuit {
            state,
            gate_list: vec![],
        }
    }
    pub fn add_gate(&mut self, gate: QuantumGate) {
        self.gate_list.push(gate);
    }
}

impl SingleGateApplicator for QuantumCircuit {
    fn add_single(&mut self, gate: QuantumGate) {
        self.add_gate(gate);
    }
    fn update_quantum_state(&mut self) {
        for gate in &self.gate_list {
            gate.update_quantum_state(&mut self.state);
        }
    }
}

impl fmt::Display for QuantumCircuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.state.fmt(f)
    }
}
