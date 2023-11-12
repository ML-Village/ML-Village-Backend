use tokio::task::JoinHandle;

use crate::giza_utils::generate_proof_from_casm;

pub async fn run_prove_in_bg(
    casm_path: String,
    output_file: String,
) -> JoinHandle<Result<(), String>> {
    tokio::task::spawn(generate_proof_from_casm(casm_path, output_file))
}
