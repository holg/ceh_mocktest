// src/questions.rs
//// src/helper/mod.rs
//// src/main.rs
#![allow(unused_imports)]

use serde_json::json;
use serde::{Deserialize, Serialize};
use colored::*;
use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::io::{self, Write};
use std::process;
use std::thread::sleep;
use std::time::Duration;
use super::helper::{banner}; // we are one deeper than the helper module, as executed from e.g. main.rs /lib.rs
use rand::prelude::{IndexedRandom}; //, SliceRandom};
use clipboard::{ClipboardContext, ClipboardProvider};
use anyhow;
use crate::{helper, questions};
#[cfg(feature = "use_ki")]
use super::ollama;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct OllamaJson {
    pub hint: String,
    pub option_number: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct Question {
    pub question: String,
    pub answer: String,
    pub hint: Option<String>,
    pub options: Vec<String>,
}
impl Question {
    fn new() -> Self {
        Self {
            question: "".to_string(),
            answer: "".to_string(),
            hint: None,
            options: vec!["".to_string(); 4],
        }
    }
}

// Enum to handle different types of duplicate checks
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum QuestionType {
    DuplicateQuestions, // Based on question field
    DuplicateNodes,     // Based on all fields
    DefaultItem,        // Placeholder for default case
}

#[derive(Debug, Clone,  Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct TypedQuestion {
    pub qtype: QuestionType,
    pub question: Question,
}
impl TypedQuestion {
    pub fn new() -> Self {
        Self {
            qtype: QuestionType::DefaultItem,
            question: Question::new(),
        }
    }
}

pub fn load_question_pool() -> Vec<Question> {
    if helper::is_use_local() {
        let data = match fs::read_to_string("questions.json") {
            Ok(content) => content,
            Err(_) => {
                eprintln!("Error: Unable to read file");
                return vec![];
            }
        };

        match serde_json::from_str::<Vec<Question>>(&data) {
            Ok(questions) => questions,
            Err(e) => {
                eprintln!("Error: JSON parsing failed:");
                eprintln!("  {}", e);
                dbg!(e);
                let preview = data.chars().take(100).collect::<String>();
                eprintln!("Data preview: {}", preview);
                vec![]
            },
        }
    } else {
        // Keep the existing code for the non-local case
        vec![]
    }
}
pub fn choose_questions(pool: &Vec<Question>, num_questions: usize) -> Vec<Question> {
    let mut rng = rand::thread_rng();
    pool.choose_multiple(&mut rng, num_questions).cloned().collect()
}

pub fn check_for_duplicates(question_pool: &Vec<Question>) -> Vec<TypedQuestion> {
    let mut typed_questions: Vec<TypedQuestion> = Vec::new();
    let mut question_map: HashMap<String, Vec<&Question>> = HashMap::new();

    // Group questions by their text
    for question in question_pool {
        question_map
            .entry(question.question.clone())
            .or_insert_with(Vec::new)
            .push(question);
    }

    // Check within each group for exact duplicates
    for questions in question_map.values() {
        if questions.len() > 1 {
            for (i, question) in questions.iter().enumerate() {
                let is_duplicate_node = questions.iter().skip(i + 1).any(|other| {
                    question.answer == other.answer &&
                        question.hint == other.hint &&
                        question.options == other.options
                });

                if is_duplicate_node {
                    typed_questions.push(TypedQuestion {
                        qtype: QuestionType::DuplicateNodes,
                        question: (*question).clone()
                    });
                } else {
                    typed_questions.push(TypedQuestion {
                        qtype: QuestionType::DuplicateQuestions,
                        question: (*question).clone()
                    });
                }
            }
        } else {
            typed_questions.push(TypedQuestion {
                qtype: QuestionType::DefaultItem,
                question: (*questions.first().unwrap()).clone()
            });
        }
    }

    typed_questions
}

pub fn ask_question(question_number: usize, typed_question: &TypedQuestion) -> bool {
    let question = &typed_question.question;
    let ca = &question.answer;
    let options = &question.options;
    let hint = question.hint.as_deref();

    sleep(Duration::from_micros(300));
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear screen
    banner();
    println!("Question {}\n\n{}", question_number, question.question);
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option.cyan());
    }

    loop {
        print!("\nYour answer here [1-4] (X to exit): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_uppercase();

        if input == "X" {
            println!("Exiting the quiz...");
            process::exit(0);
        }

        match input.parse::<usize>() {
            Ok(5) => {
                if let Some(hint_text) = hint {
                    println!("Hint: {}", hint_text.yellow());
                } else {
                    println!("No hint available for this question.");
                }
            }
            Ok(cor) if (1..=4).contains(&cor) => {
                if options[cor - 1] == *ca {
                    println!("{}", "Correct!".green());
                    return true;
                } else {
                    println!(
                        "{}",
                        format!("Wrong! The correct answer was: {}", ca).red()
                    );
                    return false;
                }
            }
            _ => {
                println!("{}", "Invalid input. Please enter a number from 1 to 4 or 'X' to exit.".red());
            }
        }
    }
}

pub fn add_question_to_json(new_question: Question) -> anyhow::Result<()> {
    // Read existing questions
    let mut questions = load_question_pool();

    // Add the new question
    questions.push(new_question);

    // Write the updated questions back to the file
    let json = serde_json::to_string_pretty(&questions)?;
    let mut file = fs::File::create("questions.json")?;
    file.write_all(json.as_bytes())?;

    println!("Question added successfully!");
    Ok(())
}
#[cfg(feature = "use_clipboard")]
pub fn get_question_from_clipboard() -> anyhow::Result<Question> {
    // Retrieve text content from clipboard
    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Error creating clipboard context");
    let clipboard_content = ctx.get_contents().expect("Error getting clipboard content");
    Ok(create_question_from_text(&clipboard_content)?)
}

#[cfg(not(feature = "use_clipboard"))]
pub fn get_question_from_clipboard() -> anyhow::Result<Question> {Ok(Question::new())}


#[cfg(feature = "use_ki")]
pub fn fill_question_from_ollama(mut question: Question) -> anyhow::Result<Question> {
    // Retrieve text content from clipboard
    dbg!(&question);
    let mut question_text = "answer in correct JSON Format (so no comments) only two fields \"option_number  exact (1-4)\": int Number of the option\n\"hint\": some single line hint\n\n} Question:\n\n".to_string();
    question_text += &question.question;
    question_text += "\nOptions:\n";
    let mut i = 0;
    for option in &question.options {
        i += 1;
        question_text += &format!("{}:{}\n", i, option);
    }
    dbg!(&question_text);
    let test = ollama::ask_ollama_model("mistral-large".to_string(), question_text,
        Some(0.7), Some(1.2), Some(40), Some(0.9), Some(100), Some("3."))?;

    dbg!(&test.response);
    // Parse the JSON-like response
    let response = test.response.trim();
    // Clean and parse the JSON-like response
    let cleaned_response = test.response
        .trim()
        .trim_start_matches("```json\n")
        .trim_end_matches("\n```")
        .replace("\\n", "\n")
        .replace("\\_", "_");
    let json_response: OllamaJson = match serde_json::from_str(&cleaned_response) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error parsing JSON response: {}", e);
            eprintln!("Raw response: {}", response);

            // Attempt to create a default OllamaJson if parsing fails
            OllamaJson {
                hint: "Unable to parse hint".to_string(),
                option_number: 1, // Default to the first option
            }
        }
    };
    dbg!(&json_response);
    let mut correct_option = json_response.option_number;
    let mut hint = json_response.hint;
    if correct_option == 0 {
        correct_option = 1;
        hint = "No ollama hint available".to_string();
    }else{

    }

    question.answer = question.options[correct_option - 1].clone();
    question.hint = Some(hint);

    Ok(question)
}

pub fn get_filled_question(question:&Question) -> anyhow::Result<Question> {
    // get question with same question text
    let questions = load_question_pool();
    let found_question = questions.iter().find(|q| q.question == question.question);
    if let Some(q) = found_question {
        return Ok(q.clone());
    }else{
        return Err(anyhow::Error::msg("Question not found"));
    }
}

/// Creates a `Question` struct from a given text input.
///
/// This function parses a string containing a question and its multiple-choice options,
/// and constructs a `Question` struct from it. The input text is expected to have the
/// question followed by four options, each starting with a letter (A, B, C, D).
///
/// # Parameters
///
/// * `text` - A string slice containing the question text and options.
///
/// # Returns
///
/// * `Result<Question, Box<dyn std::error::Error>>` - A Result containing either:
///   - `Ok(Question)`: A `Question` struct if parsing is successful.
///   - `Err(Box<dyn std::error::Error>)`: An error if the input format is invalid.
///
/// # Errors
///
/// This function will return an error if:
/// - The input text has fewer than two lines.
/// - The number of parsed options is not exactly 4.
pub fn create_question_from_text(text: &str) -> anyhow::Result<Question> {
    let parts: Vec<&str> = text.split("\n").collect();
    if parts.len() < 2 {
        dbg!(&parts);
        return Err(anyhow::Error::msg("Invalid question format"));
    }
    let mut question_text = parts[0].trim().to_string();
    let mut start_part_index = 1;
    // dbg!(&question_text);
    while start_part_index < parts.len() && parts[start_part_index] != "A" && parts[start_part_index] != "\"A,\"" {
        dbg!(parts[start_part_index]);
        question_text += " "; // Add a space between parts
        question_text += parts[start_part_index].trim();
        start_part_index += 1;
    }
    if start_part_index>4 {
        dbg!(&parts);
        dbg!(parts[start_part_index..].join("\n"));
        // return Err(anyhow::Error::msg("Invalid question format"));
    }
    let options_text = parts[start_part_index..].join("\n");
    // dbg!(&options_text);
    let mut options = Vec::new();
    let answer = String::new();

    let mut i = 1;
    for line in options_text.lines() {
        i += 1;
        if i % 2 == 0 {
            continue;
        } else {
            options.push(line.trim().to_string());
        }
    }
    // dbg!(&options.len());
    if options.len() != 4 {
        dbg!(&options);
        return Err(anyhow::Error::msg("Invalid number of options"));
    }

    Ok(Question {
        question: question_text,
        answer,
        hint: None,
        options,
    })
}





// TODO refactor

pub fn do_clipbboard_actions() -> anyhow::Result<Question> {
    let clipboard_question = match get_clipboard_question() {
        Ok(question) => question,
        Err(e) => {
            eprintln!("Error getting question from clipboard: {}", e);
            return Err(e);
        }
    };
    let found_question = check_question_exists(&clipboard_question);
    if found_question.is_some() {
        println!("Question already exists in the pool.");
        return Ok(found_question.unwrap());
    }
    let filled_question = do_clipboard_question(clipboard_question.clone())?;
    if &clipboard_question == &filled_question {
        println!("Question filled by Ollama:");
        println!("{:#?}", filled_question);
    }else {
        add_question_to_json(filled_question.clone())?;
    }
    Ok(filled_question)
}

#[cfg(feature = "use_clipboard")]
fn get_clipboard_question() -> anyhow::Result<Question> {
    questions::get_question_from_clipboard()
}
#[cfg(not(feature = "use_clipboard"))]
fn get_clipboard_question() -> anyhow::Result<Question> {
    Ok(questions::Question::new())
}
#[cfg(feature = "use_ki")]
fn do_clipboard_question(clip_question:Question) -> anyhow::Result<Question> {
    // If Question is created from the clipboard, we ask Ollama about it
    match questions::fill_question_from_ollama(clip_question.clone()) {
        Ok(filled_question) => Ok(filled_question),
        Err(_) => Ok(clip_question)
    }
}

fn check_question_exists(question: &Question) -> Option<Question> {
    let questions = load_question_pool();
    questions.into_iter().find(|q| q.question == question.question)
}

#[cfg(not(feature = "use_ki"))]
fn do_clipboard_question(clip_question:Question) -> anyhow::Result<Question> {
    Ok(clip_question)
}