// src/main.rs
#![allow(unused_imports, dead_code)]
mod helper;
mod questions;

use ollama_rs::generation::completion::GenerationResponse;
use helper::{htr_low_level_http, apple_say_using, ollama, quiz};
// use colored::*;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::options::GenerationOptions;
use ollama_rs::Ollama;
use questions::{load_question_pool, TypedQuestion, QuestionType};
use crate::questions::Question;

// use quiz::{TypedQuestion};
// no internet load from local questions.json
#[cfg(not(feature = "use_local"))]
pub const USE_LOCAL: bool = false;
#[cfg(feature = "use_local")]
pub const USE_LOCAL: bool = true;
#[cfg(not(feature = "use_ki"))]
pub const USE_KI: bool = false;
#[cfg(feature = "use_ki")]
pub const USE_KI: bool = true;
fn test_ask_ollama() -> Result<GenerationResponse, Box<dyn std::error::Error>> {
    if USE_KI {
        let ollama_result = ollama::ask_ollama_model(
            "mistral".to_string(), "Why is the sky blue?".to_string(), None, None, None, None, None, None);
        Ok(ollama_result?)
    }else{
        Err("Not using KI".into())
    }
}
#[allow(dead_code)]
fn test_typed_question(){
    let typed_question = TypedQuestion {
        qtype: QuestionType::DuplicateQuestions,
        question: questions::Question {
            question: "What is the capital of France?".to_string(),
            options: vec!["Paris".to_string(), "London".to_string(), "Berlin".to_string(), "Madrid".to_string()],
            answer: "Paris".to_string(),
            hint: Some("It's a city in France".to_string())
        }
    };
    questions::ask_question(1, &typed_question);
}

#[allow(dead_code)]
fn test_components(){
    let _ = test_ask_ollama();
    test_typed_question();
}
#[allow(dead_code)]
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
    // test_components();
    // htr_low_level_http::check_internet_connection();
    // apple_say_using("欢迎来到黑客测验！", Some("Tingting"));
    // let question_pool = load_question_pool();
    // let num_questions = quiz::get_num_questions(); // in short the :: denominates the functional approach
    // let typed_questions = questions::check_for_duplicates(&question_pool);
    if USE_KI {
        //test_ask_ollama().expect("Error asking ollama");
    }
    let clip_question = questions::get_question_from_clipboard();
    match clip_question {
        Ok(question) => {
            // check if we already have this question in the json
            let found_question=  questions::get_filled_question(&question);
            if found_question.is_ok(){
                eprintln!("Question already in json");
                dbg!(&found_question);
                return;
            }
            // if Question is created from clipboard, we ask ollama about it
            let ollama_question = questions::fill_question_from_ollama(question);
            let mut i = 0;
            match ollama_question{
                Ok(question) => {
                    // if ollama filled in the gaps in clip_question, we add the question to the json
                    questions::add_question_to_json(question.clone()).expect("Error adding question to json");
                    i += 1;
                    questions::ask_question(i, &TypedQuestion { qtype: QuestionType::DefaultItem, question });
                },
                Err(e) => {
                    eprintln!("Error getting question from ollama: {}", e);
                }
            }

        },
        Err(e) => {
            eprintln!("Error getting question from clipboard: {}", e);
        }
    };
}