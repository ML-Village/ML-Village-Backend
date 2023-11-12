use std::{process::Command, str::from_utf8};

use regex::Regex;

pub async fn prepare_inference_environment(model_path: &str) {
    // TODO
}

pub async fn convert_input_to_cairo(input_str: &str, write_path: &str) {
    
}

pub async fn run_inference(program_path: &str) -> Result<Vec<String>, String> {
    let output = Command::new("scarb")
        .current_dir(program_path)
        .arg("test")
        .output()
        .map_err(|err| format!("Failed to execute scarb: {:?}", err))?;

    let regex = Regex::new(r"(raw: (?<val>0x\w+))").unwrap();
    let outputs: Vec<String> = regex
        .captures_iter(
            from_utf8(output.stdout.as_slice())
                .map_err(|err| format!("Failed to parse output: {:?}", err))?,
        )
        .map(|caps| caps.name("val").unwrap().as_str().to_owned())
        .collect();

    Ok(outputs)
}
