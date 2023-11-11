use clap::{Arg, Command};
use lambdaworks_math::field::fields::fft_friendly::stark_252_prime_field::Stark252PrimeField;
use lambdaworks_math::traits::{Deserializable, Serializable};
use lambdaworks_stark::cairo::air::{generate_cairo_proof, verify_cairo_proof, PublicInputs};
use lambdaworks_stark::cairo::runner::run::{generate_prover_args, CairoVersion};
use lambdaworks_stark::starks::proof::options::ProofOptions;
use lambdaworks_stark::starks::proof::stark::StarkProof;
use std::env;
use std::time::Instant;

fn generate_proof(
    input_path: &String,
    proof_options: &ProofOptions,
) -> Option<(StarkProof<Stark252PrimeField>, PublicInputs)> {
    let timer = Instant::now();

    let cairo_version = if input_path.contains(".casm") {
        println!("Running casm on CairoVM and generating trace ...");
        CairoVersion::V1
    } else {
        println!("Running program on CairoVM and generating trace ...");
        CairoVersion::V0
    };

    let Ok(program_content) = std::fs::read(input_path) else {
        println!("Error opening {input_path} file");
        return None;
    };

    let Ok((main_trace, pub_inputs)) =
        generate_prover_args(&program_content, &cairo_version, &None)
    else {
        println!("Error generating prover args");
        return None;
    };

    println!("  Time spent: {:?} \n", timer.elapsed());

    let timer = Instant::now();
    println!("Making proof ...");
    let proof = match generate_cairo_proof(&main_trace, &pub_inputs, proof_options) {
        Ok(p) => p,
        Err(e) => {
            println!("Error generating proof: {:?}", e);
            return None;
        }
    };

    println!("Time spent in proving: {:?} \n", timer.elapsed());

    Some((proof, pub_inputs))
}

fn verify_proof(
    proof: StarkProof<Stark252PrimeField>,
    pub_inputs: PublicInputs,
    proof_options: &ProofOptions,
) -> bool {
    let timer = Instant::now();

    println!("Verifying ...");
    let proof_verified = verify_cairo_proof(&proof, &pub_inputs, proof_options);
    println!("Time spent in verifying: {:?} \n", timer.elapsed());

    if proof_verified {
        println!("Verification succeded");
    } else {
        println!("Verification failed");
    }

    proof_verified
}

fn main() {
    let matches = Command::new("STARK Proof Generator")
        .version("1.0")
        .author("Your Name")
        .about("Generates and verifies STARK proofs")
        .subcommand(
            Command::new("verify").about("Verifies a proof").arg(
                Arg::new("input")
                    .help("Input file path")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("verify", sub_m)) => {
            println!("Verify subcommand called");
            let input_path = sub_m.get_one::<String>("input").unwrap().to_string();
            let proof_options = ProofOptions::default_test_options();

            // Read the proof and public inputs from the input file
            let program_content = std::fs::read(&input_path).expect("Error opening input file");
            let mut bytes = program_content.as_slice();

            // Deserialize the proof and public inputs
            let proof_len = usize::from_be_bytes(bytes[0..8].try_into().unwrap());
            bytes = &bytes[8..];
            let proof = StarkProof::<Stark252PrimeField>::deserialize(&bytes[0..proof_len])
                .expect("Error reading proof");
            bytes = &bytes[proof_len..];
            let pub_inputs = PublicInputs::deserialize(bytes).expect("Error reading public inputs");

            // Verify the proof
            if verify_proof(proof, pub_inputs, &proof_options) {
                println!("Proof verification succeeded");
            } else {
                println!("Proof verification failed");
            }
        }
        _ => eprintln!("Invalid command"),
    }
}
