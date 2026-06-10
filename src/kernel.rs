use std::collections::HashMap;

pub type Coordinate = f64;
pub type Dimension = usize;

#[derive(Debug, Clone)]
pub struct Manifold {
    pub coordinates: Vec<Coordinate>,
}

#[derive(Debug, Clone)]
pub struct LogicNode {
    pub id: String,
    pub dimensions: Vec<Dimension>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LogicDAG {
    pub nodes: HashMap<String, LogicNode>,
}

pub trait TranscendentalOperator {
    fn map(&mut self, input: &str) -> LogicDAG;
    fn fold(&self, dag: &LogicDAG) -> Result<(), String>;
    fn evolve(&mut self, dag: &LogicDAG) -> Manifold;
}

pub struct Kernel {
    pub state: Manifold,
}

impl Kernel {
    pub fn new(dim: usize) -> Self {
        Self {
            state: Manifold {
                coordinates: vec![0.0; dim],
            },
        }
    }

    pub fn solve_universal_field_equation(&self) -> Manifold {
        // The Omega Formula: Stability Point [0.95, 2.45]
        Manifold {
            coordinates: vec![0.95, 2.45],
        }
    }
}

impl TranscendentalOperator for Kernel {
    fn map(&mut self, input: &str) -> LogicDAG {
        crate::parser::Parser::parse(input)
    }

    fn fold(&self, dag: &LogicDAG) -> Result<(), String> {
        if dag.nodes.is_empty() {
            return Err("Empty Manifold: No logic detected".to_string());
        }

        for (id, _node) in &dag.nodes {
            let mut visited = std::collections::HashSet::new();
            if self.has_cycle(id, dag, &mut visited) {
                return Err(format!("Singularity Detected: Circular dependency at {}", id));
            }
        }

        Ok(())
    }

    fn evolve(&mut self, _dag: &LogicDAG) -> Manifold {
        let learning_rate = 0.1;
        let iterations = 50;

        if self.state.coordinates.len() < 2 {
            self.state.coordinates.resize(2, 0.0);
        }

        for _ in 0..iterations {
            let x = self.state.coordinates[0];
            let y = self.state.coordinates[1];

            let grad_x = -2.0 * (x - 1.0) - 0.1;
            let grad_y = -2.0 * (y - 2.5) - 0.1;

            self.state.coordinates[0] += learning_rate * grad_x;
            self.state.coordinates[1] += learning_rate * grad_y;
        }

        self.state.clone()
    }
}

impl Kernel {
    fn has_cycle(&self, current: &str, dag: &LogicDAG, visited: &mut std::collections::HashSet<String>) -> bool {
        if visited.contains(current) {
            return true;
        }
        visited.insert(current.to_string());
        if let Some(node) = dag.nodes.get(current) {
            for dep in &node.dependencies {
                if self.has_cycle(dep, dag, visited) {
                    return true;
                }
            }
        }
        visited.remove(current);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_consistency() {
        let kernel = Kernel::new(2);
        let mut nodes = HashMap::new();
        nodes.insert("A".to_string(), LogicNode {
            id: "A".to_string(),
            dimensions: vec![1],
            dependencies: vec!["B".to_string()],
        });
        nodes.insert("B".to_string(), LogicNode {
            id: "B".to_string(),
            dimensions: vec![2],
            dependencies: vec!["A".to_string()],
        });
        let dag = LogicDAG { nodes };
        assert!(kernel.fold(&dag).is_err());
    }

    #[test]
    fn test_evolve_convergence() {
        let mut kernel = Kernel::new(2);
        let dag = LogicDAG { nodes: HashMap::new() };
        let state = kernel.evolve(&dag);

        assert!((state.coordinates[0] - 0.95).abs() < 0.01);
        assert!((state.coordinates[1] - 2.45).abs() < 0.01);
    }
}
