//helpers/quiz.rs
// ./helpers/mod.rs
#![allow(unused_imports, dead_code)]
use crate::questions;
use crate::questions::TypedQuestion;
use std::io::{self, Write};
use rand::prelude::{IndexedRandom};
use super::{banner, quiz}; // The mod.rs is the best explanation for super IMHO
// Import the USE_LOCAL constant from main module
pub fn get_num_questions() -> Option<usize> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear screen
    banner();
    loop {
        print!("Do you want to set a custom number of questions? [Default: 125] (yes/no): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "yes" => {
                print!("Enter the number of questions you want: ");
                io::stdout().flush().unwrap();
                let mut num_input = String::new();
                io::stdin().read_line(&mut num_input).unwrap();
                match num_input.trim().parse::<usize>() {
                    Ok(num) => return Some(num),
                    Err(_) => println!("Invalid number. Please enter a valid number."),
                }
            }
            "no" => return None,
            _ => return None,
        }
    }
}

pub fn run_quiz(question_pool: Vec<TypedQuestion>, num_questions: Option<usize>) {
    let num_questions = num_questions.unwrap_or(125);
    let mut rng = rand::thread_rng();
    let mut selected_questions: Vec<TypedQuestion> = question_pool
        .choose_multiple(&mut rng, num_questions)
        .cloned()
        .collect();

    let mut question_number = 1;
    while !selected_questions.is_empty() {
        let typed_question = selected_questions.pop().unwrap();
        questions::ask_question(question_number, &typed_question);
        question_number += 1;
    }

    println!("Congratulations! You have completed the quiz.");
    println!("Press Enter to exit...");
    io::stdin().read_line(&mut String::new()).unwrap();
}