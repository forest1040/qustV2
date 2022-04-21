pub mod circuit;
pub mod gates;
pub mod state;

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
