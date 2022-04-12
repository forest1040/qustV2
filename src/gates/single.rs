use crate::{carray, carray_i, state::QuantumState, GateMatrixType, Qubit};
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
        fn $mat(&mut self, qubit: &Qubit)-> SingleGate {
            self.apply_single(&$mat.matrix, qubit)
        }
        // fn update_quantum_state() {

        // }
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

impl SingleGate {
    fn apply_single(&mut self, matrix: &Array2<Complex<f64>>, qubit: &Qubit) -> &mut SingleGate {
        //apply(&[qubit], matrix);
        self.target_qubit_index.push(qubit.index);
        self
    }
}

pub trait SingleGateApplicator {
    fn apply_single(&mut self, matrix: &Array2<Complex<f64>>, qubit: &Qubit) -> SingleGate;

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

pub fn mask_vec(qubits: &[&Qubit]) -> Vec<usize> {
    let min_qubit_index = qubits.iter().map(|q| q.index).min().unwrap();
    let max_qubit_index = qubits.iter().map(|q| q.index).max().unwrap();
    let min_qubit_mask = 1usize << min_qubit_index;
    let max_qubit_mask = 1usize
        << if qubits.len() > 1 {
            max_qubit_index - 1
        } else {
            max_qubit_index
        };
    let mask_low = min_qubit_mask - 1;
    let mask_high = !(max_qubit_mask - 1);
    let mut res = Vec::with_capacity(3);
    res.push(max_qubit_mask);
    res.push(mask_low);
    res.push(mask_high);
    res
}

pub fn indices_vec(index: usize, qubits: &[&Qubit], masks: &[usize]) -> Vec<usize> {
    let mut qubits = qubits.to_owned();
    qubits.sort_by(|a, b| a.index.cmp(&b.index));
    let mut res = Vec::with_capacity(qubits.len());
    let mask = masks[0];
    let mask_low = masks[1];
    let mask_high = masks[2];
    let basis_0 = (index & mask_low) + ((index & mask_high) << qubits.len());
    res.push(basis_0);
    // for i in 1..qubits.len() << 1 {
    //     let basis = basis_0 + mask;
    //     res.push(basis);
    // }
    if qubits.len() == 1 {
        let basis_1 = basis_0 + mask;
        res.push(basis_1);
    } else if qubits.len() == 2 {
        let target_mask1 = 1usize << qubits[1].index;
        let target_mask2 = 1usize << qubits[0].index;
        let basis_1 = basis_0 + target_mask1;
        let basis_2 = basis_0 + target_mask2;
        let basis_3 = basis_1 + target_mask2;
        res.push(basis_1);
        res.push(basis_2);
        res.push(basis_3);
    } else {
        // TODO
        unimplemented!();
    }

    res
}

pub fn apply(state: &mut QuantumState, qubits: &[&Qubit], matrix: &Array2<Complex<f64>>) {
    let masks = mask_vec(qubits);
    println!("masks: {:?}", masks);
    for i in 0..state.get_dim() >> qubits.len() {
        let indices = indices_vec(i, qubits, &masks);
        println!("indices_vec: {:?}", indices);
        let values = indices
            .iter()
            .map(|&i| state.get_vector()[i])
            .collect::<Vec<_>>();
        println!("matrix: {}", matrix);
        let new_values = matrix.dot(&arr1(&values));
        println!("new_values: {}", new_values);
        for (&i, nv) in indices.iter().zip(new_values.to_vec()) {
            // TODO: set_vector()
            //state.get_vector()[i] = nv;
        }
    }
}
