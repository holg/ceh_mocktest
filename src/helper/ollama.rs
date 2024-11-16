//helper/ollama.rs
#![allow(unused_imports, dead_code)]
use anyhow::Result;
use serde_json;
use serde_json::json;
#[cfg(feature = "tokio")]
use tokio; // Make sure to add `tokio` in your dependencies in Cargo.toml
#[cfg(feature = "use_ki")]
mod use_ki {
    use ollama_rs;
    pub use ollama_rs::generation::completion::GenerationResponse;
    pub use ollama_rs::{
        generation::{completion::request::GenerationRequest, options::GenerationOptions},
        Ollama,
    };
}

#[cfg(feature = "use_ki")]
use use_ki::*;
pub fn ollama_get_options_str_from_param(
    temperature: Option<f32>,
    repeat_penalty: Option<f32>,top_k: Option<u32>, top_p: Option<f32>,
    num_predict: Option<u32>, stop: Option<&str>
) -> String {
    let temperature = Some(temperature.unwrap_or(0.7_f32));
    let repeat_penalty = Some(repeat_penalty.unwrap_or(1.2_f32));
    let top_k = Some(top_k.unwrap_or(40_u32));
    let top_p = Some(top_p.unwrap_or(0.9_f32));
    let num_predict = Some(num_predict.unwrap_or(100_u32));
    let stop = Some(stop.unwrap_or("3."));
    json!({
        "temperature":temperature,
        "repeat_penalty": repeat_penalty,
        "top_k": top_k,
        "top_p": top_p,
        "num_predict": num_predict,
        "stop": ["\n\n", stop]
    }).to_string()
}

// Synchronous version of the function

#[cfg(feature = "use_ki")]
pub fn ask_ollama_model(
    model: String,
    prompt: String,
    temperature: Option<f32>,
    repeat_penalty: Option<f32>,top_k: Option<u32>, top_p: Option<f32>,
    num_predict: Option<u32>, stop: Option<&str>
) -> anyhow::Result<GenerationResponse> {
    let ollama = Ollama::default();
    let options: GenerationOptions =
        serde_json::from_str(&ollama_get_options_str_from_param(
            temperature, repeat_penalty, top_k, top_p, num_predict, stop
        )).expect("JSON was not well-formatted");
    let res = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(ollama.generate(GenerationRequest::new(model, prompt).options(options)));
    // dbg!(&res);
    if let Ok(res) = res {
        // println!("{}", res.response);
        Ok(res)
    } else {
        anyhow::bail!("Failed to generate response");
    }
}

#[cfg(feature = "use_ki")]
pub async fn test_tokio_ollama_model(
    model: String,
    prompt: String,
    options_str: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();
    let options: GenerationOptions =
        serde_json::from_str(&options_str).expect("JSON was not well-formatted");
    let res = ollama
        .generate(GenerationRequest::new(model, prompt).options(options))
        .await;

    if let Ok(res) = res {
        println!("{}", res.response);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(feature = "use_ki")]
    async fn test_ollama_model_success() {
        // Fill in with valid example data
        let model = String::from("mistral:latest");
        let prompt = String::from("Some prompt text");
        let options_str = String::from("{}"); // Assuming an empty JSON object for options

        let result = test_tokio_ollama_model(model, prompt, options_str).await;
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(feature = "use_ki")]
    fn test_quiz_ollama_model_success() {
        // Fill in with valid example data
        let model = String::from("mistral");
        let prompt = String::from("Some prompt text");
        let options_str = String::from("{}"); // Assuming an empty JSON object for options
        let result = ask_ollama_model(model, prompt, None, None, None, None, None, None);
        assert!(result.is_ok());
    }
}
