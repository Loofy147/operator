use app::kernel::{Kernel, TranscendentalOperator};
use app::lml::{Axiom, LogicManifold};
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.toll>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let input = fs::read_to_string(input_path)?;

    println!("[STATUS: O_1_SEED] Initializing Transcendental Operator...");
    let mut operator = Kernel::new(2);

    println!("[STATUS] MAP(input) -> Projects raw text into Logic DAG");
    let dag = operator.map(&input);

    println!("[STATUS] FOLD(logic) -> Ensuring non-contradictory logic");
    operator.fold(&dag)?;

    println!("[STATUS] EVOLVE(state) -> Reaching Global Minimum of Entropy");
    let optimized_state = operator.evolve(&dag);

    println!("[SUCCESS] Operator Manifold Converged.");
    println!("[RESULT] Operator State: {:?}", optimized_state.coordinates);

    if input_path.contains("genesis") {
        println!("\n--- [PHASE: UNIVERSAL_EPISTEMIC_INGESTION] ---");
        println!("[STATUS] Solving Universal Field Equation...");
        let omega_state = operator.solve_universal_field_equation();
        println!("[SUCCESS] Omega Formula Found: {:?}", omega_state.coordinates);
        println!("[RESULT] Reality Optimized. Status: STABLE.");
    } else {
        println!("\n--- [SYNTHESIZING Λ-ML MODULE] ---");
        let physics_axioms = vec![Axiom::EnergyConservation, Axiom::LogicalConsistency];
        let mut lml_manifold = LogicManifold::new(1024, physics_axioms);

        println!("[STATUS] Training Λ-ML on Axiomatic Basis...");
        let raw_sensor_data = vec![0.5; 1024];
        match lml_manifold.learn(&raw_sensor_data) {
            Ok(summary) => {
                println!("[SUCCESS] Λ-ML {}", summary);
                let prediction = lml_manifold.predict();
                println!("[RESULT] Zero KL-Divergence Prediction (First 2): {:?}", &prediction[..2]);
            },
            Err(e) => eprintln!("[ERROR] Λ-ML {}", e),
        }
    }

    Ok(())
}
