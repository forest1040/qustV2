use num::complex::Complex;
use rand::distributions::{Standard, WeightedIndex};
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
        let dim = 1usize << n;
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

    pub fn get_vector(&self) -> &Vec<Complex<f64>> {
        &self.states
    }

    pub fn get_dim(&self) -> usize {
        self.dim
    }

    pub fn load(&mut self, states: Vec<Complex<f64>>) {
        // TODO: validate
        // if (_state.size() != _dim) {
        //     throw std::invalid_argument("inconsistent dim");
        // }
        self.states = states
    }

    pub fn get_zero_probability(&self, target_qubit_index: usize) -> f64 {
        // TODO: validate
        // if (target_qubit_index >= this->qubit_count) {
        //     throw std::invalid_argument("qubit index >= num_qubit");
        // }
        m0_prob(target_qubit_index, &self.get_vector(), self.dim)
        // let mask = 1usize << target_qubit_index;
        // let mut sum = 0.;
        // for i in 0..self.dim << 1 {
        //     let temp_basis = (i >> target_qubit_index) << (target_qubit_index + 1);
        //     let basis_1 = (temp_basis + i % mask) ^ mask;
        //     sum += self.states[basis_1].powi(2).re;
        // }
        // sum
    }

    pub fn sampling(&self, sampling_count: usize) -> Vec<usize> {
        let mut sum = 0.;
        let mut weights = Vec::with_capacity(self.dim + 1);
        weights.push(0.);
        for i in 0..self.dim {
            sum += self.states[i].norm();
            weights.push(sum);
        }
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = thread_rng();
        let mut result: Vec<usize> = Vec::with_capacity(sampling_count);
        for _ in 0..sampling_count {
            result.push(dist.sample(&mut rng) - 1);
        }
        result
    }

    pub fn inner_product(state_bra: QuantumState, state_ket: QuantumState) -> Complex<f64> {
        // double real_sum = 0.;
        // double imag_sum = 0.;
        // ITYPE index;
        // for (index = 0; index < dim; ++index) {
        //     CTYPE value;
        //     value += conj(state_bra[index]) * state_ket[index];
        //     real_sum += _creal(value);
        //     imag_sum += _cimag(value);
        // }
        // return real_sum + 1.i * imag_sum;
        let mut real_sum = 0.;
        let mut imag_sum = 0.;
        for i in 0..state_bra.dim {
            let value = state_bra.states[i].conj() * state_ket.states[i];
            real_sum += value.re;
            imag_sum += value.im;
        }
        Complex::new(real_sum, imag_sum)
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

fn m0_prob(target_qubit_index: usize, state: &Vec<Complex<f64>>, dim: usize) -> f64 {
    let mask = 1usize << target_qubit_index;
    let mut sum = 0.;
    for i in 0..dim >> 1 {
        let basis_1 = insert_zero_to_basis_index(i, mask, target_qubit_index) ^ mask;
        //sum += pow(_cabs(state[basis_1]), 2);
        sum += state[basis_1].l1_norm().powi(2);
    }
    sum
}

#[inline]
fn insert_zero_to_basis_index(basis_index: usize, basis_mask: usize, qubit_index: usize) -> usize {
    // ITYPE temp_basis = (basis_index >> qubit_index) << (qubit_index + 1);
    // return temp_basis + basis_index % basis_mask;
    let temp_basis = (basis_index >> qubit_index) << (qubit_index + 1);
    temp_basis + basis_index % basis_mask
}
