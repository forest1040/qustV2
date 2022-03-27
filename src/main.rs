//use num::complex::Complex;
use qustV2::state::QuantumState;

// fn set(sim: &mut QuantumSimulator, qubit: &Qubit, r: MeasuredResult) {
//     if sim.measure(qubit) != r {
//         sim.X(qubit);
//     }
// }

fn main() {
    // let mut sim = QuantumSimulator::new(2);
    // let qubits = sim.get_qubits();
    // let measure_count = 10000;

    // for _ in 0..measure_count {
    //     set(&mut sim, &qubits[0], MeasuredResult::Zero);
    //     set(&mut sim, &qubits[1], MeasuredResult::Zero);

    //     sim.H(&qubits[0]);
    //     sim.CNOT(&qubits[0], &qubits[1]);

    //     assert_eq!(sim.measure(&qubits[0]), sim.measure(&qubits[1]));
    // }

    // ============= 量子状態 =================
    println!("QuantumState");
    let n = 2;
    let mut state = QuantumState::new(n);
    //println!("{:?}", state);
    println!("{}", state);

    // |00>に初期化
    state.set_zero_state();
    // 基底を二進数と見た時の整数値を入れて、その状態に初期化
    state.set_computational_basis(1);
    println!("{}", state);

    // シードを指定してランダムな初期状態を生成
    // (シードを省略した場合は時刻を用いてシードを決定します。)
    let seed = 0;
    state.set_haar_random_state(seed);
    println!("{}", state);
}
