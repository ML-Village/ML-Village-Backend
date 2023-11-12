mod cairo_utils;

#[tokio::main]
pub async fn main() -> Result<(), String> {
    cairo_utils::prepare_inference_environment("abc1".to_owned()).await?;
    cairo_utils::convert_ttt_input_to_cairo("1,1,2;2,2,2;0,0,2", "inference_result/abc1/orion/src/test.cairo").await?;
    cairo_utils::convert_ttt_input_to_cairo("1,1,2;2,2,2;0,0,2", "inference_result/abc1/orion/src/inference.cairo").await?;
    let outputs = cairo_utils::run_inference("inference_result/abc1/orion").await?;
    println!("{:?}", outputs);
    Ok(())    
}