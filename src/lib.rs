//pub mod gate;
pub mod state;

struct Qubit {
    pub index: usize,
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
