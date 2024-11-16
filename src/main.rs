// src/main.rs
#![allow(unused_imports, dead_code)]
mod helper;
mod questions;

#[cfg(feature = "use_ki")]
use ollama_rs::{generation::{completion::GenerationResponse, completion::request::GenerationRequest, options::GenerationOptions}, Ollama};
use crate::quiz::get_num_questions;
use questions::{load_question_pool, check_for_duplicates, TypedQuestion, QuestionType};
use helper::{htr_low_level_http, apple_say_using, ollama, quiz, banner, build_clap_app, AppConfig};
use crate::questions::Question;

// use quiz::{TypedQuestion};
// no internet load from local questions.json

#[cfg(feature = "use_ki")]
fn test_ask_ollama() -> Result<GenerationResponse, Box<dyn std::error::Error>> {
    if helper::is_use_ki() {
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
            question: "What is the capital of Burkina Faso?".to_string(),
            options: vec!["Paris".to_string(), "Ouagadougou".to_string(), "Berlin".to_string(), "Madrid".to_string()],
            answer: "Paris".to_string(),
            hint: Some("It's a city in France".to_string())
        }
    };
    questions::ask_question(1, &typed_question);
}

#[allow(dead_code)]
#[cfg(feature = "use_ki")]
#[tokio::main]
async fn wuhan_main() -> Result<(), Box<dyn std::error::Error>> {
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
    // let num_questions = quiz::get_num_questions(); // in short the :: denominates the functional approach
    // let typed_questions = questions::check_for_duplicates(&question_pool);
    let config = helper::get_app_config();

    if let Err(e) = banner(config) {
        eprintln!("Error displaying the banner: {}", e);
    }
    println!("{:?}", helper::banner(config));
    #[cfg(feature = "use_clipboard")]{
        let result = questions::do_clipbboard_actions();
        #[cfg(feature = "use_clipboard")]
        dbg!(&result);
    }
    #[cfg(feature = "use_sqlite")]{
        let result = helper::json_to_sqlite();
        #[cfg(feature = "use_sqlite")]
        dbg!(&result);
    }
    #[cfg(not(feature = "use_clipboard"))]{
        let question_pool = check_for_duplicates(&load_question_pool());
        quiz::run_quiz(question_pool, get_num_questions());
    }

}
