use num::complex::Complex;
use qustV2::{
    gates::single::{SingleGate, X},
    state::QuantumState,
};

// fn set(sim: &mut QuantumSimulator, qubit: &Qubit, r: MeasuredResult) {
//     if sim.measure(qubit) != r {
//         sim.X(qubit);
//     }
// }

fn main() {
    // // ============= 量子状態 =================
    // println!("QuantumState");
    // let n = 2;
    // let mut state = QuantumState::new(n);
    // //println!("{:?}", state);
    // println!("{}", state);

    // // |00>に初期化
    // state.set_zero_state();
    // // 基底を二進数と見た時の整数値を入れて、その状態に初期化
    // state.set_computational_basis(1);
    // println!("{}", state);

    // // シードを指定してランダムな初期状態を生成
    // // (シードを省略した場合は時刻を用いてシードを決定します。)
    // let seed = 0;
    // state.set_haar_random_state(seed);
    // println!("{}", state);

    // state.set_zero_state();
    // let vec = state.get_vector();
    // println!("{:?}", vec);

    // let mut new_state = Vec::new();
    // new_state.push(Complex { re: 0.5, im: 0. });
    // new_state.push(Complex { re: 0.5, im: 0. });
    // new_state.push(Complex { re: 0.5, im: 0. });
    // new_state.push(Complex { re: 0.5, im: 0. });

    // state.load(new_state);
    // println!("{}", state);

    // let n = 5;
    // let mut state = QuantumState::new(n);
    // state.set_haar_random_state(seed);
    // let index = 3;
    // let zero_probability = state.get_zero_probability(index);
    // println!("prob_meas_3rd : {}", zero_probability);

    // let n = 2;
    // let mut state = QuantumState::new(n);
    // let mut new_state = Vec::new();
    // new_state.push(Complex {
    //     re: 1. / 2.0_f64.sqrt(),
    //     im: 0.,
    // });
    // new_state.push(Complex { re: 0., im: 0. });
    // new_state.push(Complex { re: 0.5, im: 0. });
    // new_state.push(Complex { re: 0.5, im: 0. });
    // state.load(new_state);
    // let data = state.sampling(10);
    // println!("{:?}", data);

    // // 内積値の計算
    // let n = 5;
    // let mut state_bra = QuantumState::new(n);
    // let mut state_ket = QuantumState::new(n);
    // state_bra.set_haar_random_state(0);
    // state_ket.set_computational_basis(0);
    // let value = QuantumState::inner_product(state_bra, state_ket);
    // println!("{:?}", value);

    // ============= 量子ゲート =================
    let x_gate = X(1);
    println!("{:?}", x_gate);

    let n = 2;
    let mut state = QuantumState::new(n);
    println!("{}", state);

    let mut x_gate = X(1);
    x_gate.update_quantum_state(&mut state);
    println!("{}", state);
}
