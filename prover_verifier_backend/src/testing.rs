mod background_utils;
mod cairo_utils;
mod giza_utils;

#[tokio::main]
pub async fn main() -> Result<(), String> {
    cairo_utils::prepare_inference_environment("abc1".to_owned()).await?;
    cairo_utils::convert_ttt_input_to_cairo(
        vec![1, 1, 2, 2, 2, 2, 0, 0, 2],
        "inference_result/abc1/orion/src/test.cairo",
    )
    .await?;
    cairo_utils::convert_ttt_input_to_cairo(
        vec![1, 1, 2, 2, 2, 2, 0, 0, 2],
        "inference_result/abc1/orion/src/inference.cairo",
    )
    .await?;
    let outputs = cairo_utils::run_inference("inference_result/abc1/orion").await?;
    cairo_utils::generate_trace("inference_result/abc1/orion").await?;
    println!("{:?}", outputs);
    giza_utils::generate_proof_from_casm("inference_result/abc1/orion/target/dev/tic_tac_toe_orion_OrionRunner.compiled_contract_class.json".to_owned(), "inference_result/abc1/zk.proof".to_owned()).await?;
    Ok(())
}
