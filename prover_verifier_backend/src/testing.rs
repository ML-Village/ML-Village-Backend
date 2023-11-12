mod cairo_utils;

#[tokio::main]
pub async fn main() -> Result<(), String> {
    let outputs = cairo_utils::run_inference("tic_tac_toe_orion").await?;
    println!("{:?}", outputs);
    Ok(())    
}