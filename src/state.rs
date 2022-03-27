use num::complex::Complex;
use rand::distributions::Standard;
//use rand::distributions::StandardNormal;
use rand::prelude::*;
use std::fmt;

#[derive(Debug)]
pub struct QuantumState {
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

    fn create_init_state(dim: usize) -> Vec<Complex<f64>> {
        let mut state_vector = Vec::with_capacity(dim as usize);
        state_vector.push(Complex::new(1., 0.));
        for _ in 1..dim {
            state_vector.push(Complex::new(0., 0.));
        }
        state_vector
    }

    pub fn set_zero_state(&mut self) {
        self.states = QuantumState::create_init_state(self.dim);
    }

    pub fn set_computational_basis(&mut self, comp_basis: usize) {
        // TODO: validate
        // if (comp_basis >= (ITYPE)(1ULL << this->qubit_count)) {
        //     throw std::invalid_argument("basis index >= 2^n");
        // }
        self.set_zero_state();
        self.states[0] = Complex::new(0., 0.);
        self.states[comp_basis] = Complex::new(1., 0.);
    }

    pub fn set_haar_random_state(&mut self, seed: u8) {
        self.initialize_haar_random_state_with_seed(seed)
    }

    fn initialize_haar_random_state_with_seed(&mut self, seed: u8) {
        let seed: [u8; 32] = [seed; 32];
        let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
        let mut norm = 0.;
        for i in 0..self.dim {
            let r1 = rng.gen_range(-1.0..1.0);
            let r2 = rng.gen_range(-1.0..1.0);
            self.states[i] = Complex::new(r1, r2);
            norm += self.states[i].norm();
        }
        let norm = norm.sqrt();
        for i in 0..self.dim {
            self.states[i] /= norm;
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
