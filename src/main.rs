use num::complex::Complex;
use std::fmt;

struct Qubit {
    pub index: usize,
}

#[derive(Debug)]
struct QuantumState {
    dim: usize,
    qubit_count: usize,
    states: Vec<Complex<f64>>,
}

impl QuantumState {
    pub fn new(n: usize) -> QuantumState {
        let dim = 1 << n;
        let mut states = vec![Complex::new(0., 0.); dim];
        states[0] = Complex::new(1., 0.);
        QuantumState {
            dim,
            qubit_count: n,
            states,
        }
    }
}

impl fmt::Display for QuantumState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sv: Vec<String> = self.states.iter().map(|x| x.to_string()).collect();
        let svs = sv.join("\n");
        let tmp = format!(
            r"* Qubit Count  : {}
* Dimension    : {}
* State vector :
{}
",
            self.qubit_count, self.dim, svs
        );
        write!(f, "{}", tmp)
    }
}

enum PauliID {
    I,
    X,
    Y,
    Z,
}
enum GateMatrixType {
    DenseMatrix,
    PauliMatrix(PauliID),
}

struct QuantumGate {}

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
    print!("hello world");

    // ============= 量子状態 =================
    println!("QuantumState");
    let n = 2;
    let state = QuantumState::new(n);
    //println!("{:?}", state);
    println!("{}", state);
}
