#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]tr
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use std::process::Command;

#[post("/api_upload_onnx", data = "<data>")]
async fn api_upload_onnx(data: rocket::Data) -> Result<Json<serde_json::Value>, (rocket::http::Status, String)> {
    let options = rocket_multipart_form_data::MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        rocket_multipart_form_data::MultipartFormDataField::file("onnx_file"),
    ]);

    let multipart_form_data = match rocket_multipart_form_data::MultipartFormData::parse(&options, &data).await {
        Ok(multipart) => multipart,
        Err(err) => {
            return Err((
                rocket::http::Status::INTERNAL_SERVER_ERROR,
                format!("Error parsing form data: {:?}", err),
            ));
        }
    };

    if let Some(file) = multipart_form_data.files.get("onnx_file") {
        if let Some(file_name) = &file.file_name {
            stream_to_file(file_name, file.data.clone())
                .await
                .map_err(|err| {
                    (
                        rocket::http::Status::INTERNAL_SERVER_ERROR,
                        format!("Error saving file: {:?}", err),
                    )
                })?;
        }
    } else {
        return Err((
            rocket::http::Status::BAD_REQUEST,
            "Missing 'onnx_file' field in the form data".to_string(),
        ));
    }

    // Now call the transpile function with the saved file
    let transpile_output = transpile_file("UPLOADS_DIRECTORY/onnx_file").await?;

    Ok(Json(json!({
        "result": {
            "success": transpile_output,
        }
    })))
}

async fn transpile_file(file_path: &str) -> Result<bool, String> {
    // Execute the giza CLI command
    let output = Command::new("giza")
        .arg("transpile")
        .arg(file_path)
        .arg("--output-path")
        .arg("cairo_model")
        .output()
        .map_err(|err| format!("Failed to execute giza: {:?}", err))?;

    // Check if the command was successful
    if output.status.success() {
        Ok(true)
    } else {
        Err(format!("Transpile failed: {:?}", output))
    }
}

async fn stream_to_file(path: &str, stream: rocket::Data) -> Result<(), String> {
    // ... (unchanged)
    Ok(())
}

fn main() {
    rocket::ignite().mount("/", routes![api_upload_onnx]).launch();
}
