use std::fmt;

use crate::{
    gates::{
        double::DoubleGateApplicator,
        gate::{GateOp, QuantumGate},
        single::SingleGateApplicator,
        triple::TripleGateApplicator,
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
    pub fn update_quantum_state(&mut self) {
        for gate in &self.gate_list {
            gate.update_quantum_state(&mut self.state);
        }
    }
}

impl SingleGateApplicator for QuantumCircuit {
    fn add_single(&mut self, gate: QuantumGate) {
        self.add_gate(gate);
    }
}

impl DoubleGateApplicator for QuantumCircuit {
    fn add_double(&mut self, gate: QuantumGate) {
        self.add_gate(gate);
    }
}

impl TripleGateApplicator for QuantumCircuit {
    fn add_triple(&mut self, gate: QuantumGate) {
        self.add_gate(gate);
    }
}

impl fmt::Display for QuantumCircuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.state.fmt(f)
    }
}
