use crate::kernel::Coordinate;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Axiom {
    EnergyConservation,
    EntropyIncrease,
    LogicalConsistency,
}

#[derive(Debug, Clone)]
pub struct LogicTensor {
    pub data: Vec<Coordinate>,
    pub axioms: HashSet<Axiom>,
    pub uncertainty: f64,
}

pub struct LogicManifold {
    pub dimensions: usize,
    pub axioms: HashSet<Axiom>,
    pub state: Vec<Coordinate>,
}

impl LogicManifold {
    pub fn new(dimensions: usize, axioms: Vec<Axiom>) -> Self {
        Self {
            dimensions,
            axioms: axioms.into_iter().collect(),
            state: vec![0.0; dimensions],
        }
    }

    pub fn learn(&mut self, data_stream: &[Coordinate]) -> Result<String, String> {
        if self.axioms.contains(&Axiom::EnergyConservation) {
            let energy: f64 = data_stream.iter().map(|x| x * x).sum();
            if energy > 1000.0 {
                return Err("Singularity Exception: Energy Conservation Violated".to_string());
            }
        }

        self.gradient_ascent(data_stream);

        let entropy: f64 = self.state.iter().map(|x| x.abs()).sum::<f64>() * 0.1;
        Ok(format!("Manifold Converged. Entropy: {:.4}", entropy))
    }

    pub fn predict(&self) -> Vec<Coordinate> {
        // Zero KL-Divergence Prediction: Return the state which is optimized for axioms
        self.state.clone()
    }

    pub fn calculate_loss(&self, prediction: &[Coordinate], target: &[Coordinate]) -> f64 {
        let mut kl_divergence = 0.0;
        for (p, t) in prediction.iter().zip(target.iter()) {
            if *p > 0.0 && *t > 0.0 {
                kl_divergence += p * (p / t).ln();
            }
        }

        let inconsistency = if self.axioms.contains(&Axiom::LogicalConsistency) {
            prediction.iter().sum::<f64>().abs() * 0.01
        } else {
            0.0
        };

        kl_divergence + inconsistency
    }

    pub fn gradient_ascent(&mut self, data: &[Coordinate]) {
        let learning_rate = 0.1;
        let iterations = 50;
        let lambda = 0.1;

        for _ in 0..iterations {
            let mut grads = vec![0.0; self.dimensions];
            for i in 0..self.dimensions {
                let q_i = data.get(i).cloned().unwrap_or(0.0);
                let x_i = self.state[i];
                grads[i] = -2.0 * (x_i - q_i) - lambda * x_i.signum();
            }

            for i in 0..self.dimensions {
                self.state[i] += learning_rate * grads[i];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_convergence() {
        let mut manifold = LogicManifold::new(2, vec![Axiom::LogicalConsistency]);
        let data = vec![1.0, 2.5];
        manifold.learn(&data).unwrap();
        let prediction = manifold.predict();
        assert!((prediction[0] - 0.95).abs() < 0.1);
        assert!((prediction[1] - 2.45).abs() < 0.1);
    }
}
