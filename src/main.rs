mod helper;
mod questions;
use helper::{htr_low_level_http, apple_say_using, banner, ollama, quiz};
use std::io::{self, Read, Write};
use colored::*;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::options::GenerationOptions;
use ollama_rs::Ollama;
use questions::{load_question_pool, TypedQuestion, QuestionType};
// use quiz::{TypedQuestion};
const USE_LOCAL: bool = true;

#[tokio::main]
async fn tokia_main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();
    let model = "llama2:latest".to_string();
    let prompt = "Why is the sky blue?".to_string();

    // Fetch the configuration from a file or from user request
    // let options_str = fs::read_to_string("options.json").expect("The option file should be available")    ;
    let options_str = r#"{
      "temperature": 0.2,
      "repeat_penalty": 1.5,
      "top_k": 25,
      "top_p": 0.25
    }"#;
    let options: GenerationOptions =
        serde_json::from_str(options_str).expect("JSON was not well-formatted");
    let res = ollama
        .generate(GenerationRequest::new(model, prompt).options(options))
        .await;

    if let Ok(res) = res {

        println!("{}", res.response);

    }
    Ok(())
}


fn main() {
    htr_low_level_http::check_internet_connection(USE_LOCAL);
    apple_say_using("欢迎来到黑客测验！", Some("Tingting"));
    let question_pool = load_question_pool();
    let num_questions = quiz::get_num_questions(); // in short the :: denominates the functional approach
    // run_quiz(question_pool, num_questions);
}