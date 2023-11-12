use std::process::Command;

pub async fn transpile_onnx_to_orion(file_path: &str, output_path: &str) -> Result<(), String> {
    // Execute the giza CLI command
    let output = Command::new(".venv/bin/giza")
        .arg("transpile")
        .arg(file_path)
        .arg("--output-path")
        .arg(output_path)
        .output()
        .map_err(|err| format!("Failed to execute giza: {:?}", err))?;

    // Check if the command was successful
    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Transpile failed: {:?}", output))
    }
}

pub async fn generate_proof_from_casm(casm_path: &str, output_path: &str) -> Result<(), String> {
    // Execute the giza CLI command
    let output = Command::new(".venv/bin/giza")
        .arg("prove")
        .arg(casm_path)
        .arg("-s")
        .arg("M")
        .arg("-o")
        .arg(output_path)
        .output()
        .map_err(|err| format!("Failed to execute giza: {:?}", err))?;

    // Check if the command was successful
    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Transpile failed: {:?}", output))
    }
}
