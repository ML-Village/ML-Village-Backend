use std::{
    future::Future,
    path::{Path, PathBuf},
    process::Command,
    str::from_utf8,
};

use async_recursion::async_recursion;
use regex::Regex;
use rocket::futures::TryFutureExt;
use tokio::{fs::{self, File}, io::{self, AsyncReadExt, AsyncWriteExt}};

#[async_recursion]
async fn copy_recursively(source: PathBuf, destination: PathBuf) -> io::Result<()> {
    fs::create_dir_all(&destination).await?;
    let mut dir = fs::read_dir(&source).await?;
    while let Some(entry) = dir.next_entry().await? {
        let filetype = entry.file_type().await?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.clone().join(entry.file_name())).await?;
        } else {
            fs::copy(entry.path(), destination.clone().join(entry.file_name())).await?;
        }
    }
    Ok(())
}
pub async fn prepare_inference_environment(proof_id: String) -> Result<(), String> {
    copy_recursively(
        PathBuf::from("tic_tac_toe_orion"),
        PathBuf::from(format!("inference_result/{}/orion", proof_id)),
    )
    .map_err(|err| err.to_string())
    .await?;
    Ok(())
}

pub async fn convert_ttt_input_to_cairo(input_str: &str, write_path: &str) -> Result<(), String> {
    // Hard coded to tic tac toe for now
    // TODO: MAKE IT GENERAL
    let rows: Vec<&str> = input_str.split(';').collect();

    fn num_str_to_str(num_str: &str) -> String {
        if num_str == "0" {
            return "zero".to_owned()
        }
        if num_str == "1"  {
            return "one".to_owned()
        }
        "two".to_owned()
    }

    let row1: Vec<String> = rows[0].split(',').map(num_str_to_str).collect();
    let row2: Vec<String> = rows[1].split(',').map(num_str_to_str).collect();
    let row3: Vec<String> = rows[2].split(',').map(num_str_to_str).collect();

    // Read from file now
    let mut file = File::open(&write_path).await.map_err(|err| err.to_string())?;
    let mut contents = String::new();
	file.read_to_string(&mut contents).await.map_err(|err| err.to_string())?;

    row1.iter().for_each(|entry| contents = contents.replacen("{%%}", entry, 1));
    row2.iter().for_each(|entry| contents = contents.replacen("{%%}", entry, 1));
    row3.iter().for_each(|entry| contents = contents.replacen("{%%}", entry, 1));
    fs::write(&write_path, contents).await.map_err(|err| err.to_string())?;
    Ok(())
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
