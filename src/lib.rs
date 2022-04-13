pub mod gates;
pub mod state;

pub struct Qubit {
    pub index: usize,
}
#[derive(Debug)]
pub enum PauliID {
    I,
    X,
    Y,
    Z,
}
#[derive(Debug)]
pub enum GateMatrixType {
    DenseMatrix,
    PauliMatrix(PauliID),
}
