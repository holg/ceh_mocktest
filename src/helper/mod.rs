use std::cmp::PartialEq;
use std::fs;
use std::io::{self, Read, Write};
use std::process;
use std::thread::sleep;
use std::time::Duration;
// use rand::seq::SliceRandom;
use colored::*;
// use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::net::{TcpStream, ToSocketAddrs};
use url;
use rand::prelude::IndexedRandom;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::marker::Tuple;
use std::process::Command;
const USE_LOCAL: bool = true;


#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
struct Question {
    question: String,
    answer: String,
    hint: Option<String>,
    options: Vec<String>,
}
    // Enum to handle different types of duplicate checks
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum QuestionType {
    DuplicateQuestions, // Based on question field
    DuplicateNodes,     // Based on all fields
    DefaultItem,        // Placeholder for default case
}
struct TypedQuestion {
    qtype: QuestionType,
    question: Question,
}
#[derive(Debug)]
struct HttpResponse {
    status_line: String,
    headers: HashMap<String, String>,
    body: String,
}

fn http_request(url: &str, method: &str, include_body: bool, timeout: Option<Duration>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    // Parse the URL
    let url = url::Url::parse(url)?;
    let host = url.host_str().ok_or("Invalid host")?;
    let path = url.path();
    let port = url.port().unwrap_or(80);

    // Connect to the server with timeout
    let addr = (host, port).to_socket_addrs()?.next().ok_or("Invalid address")?;
    let mut stream = match timeout {
        Some(duration) => TcpStream::connect_timeout(&addr, duration)?,
        None => TcpStream::connect(addr)?,
    };

    // Set read and write timeouts
    if let Some(duration) = timeout {
        stream.set_read_timeout(Some(duration))?;
        stream.set_write_timeout(Some(duration))?;
    }

    // Send the HTTP request
    let request = format!(
        "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        method, path, host
    );
    stream.write_all(request.as_bytes())?;

    // Read the response
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Parse the response
    let mut parts = response.splitn(2, "\r\n\r\n");
    let header_section = parts.next().ok_or("No headers found")?;
    let body = parts.next().unwrap_or("").to_string();

    let mut headers = header_section.lines();
    let status_line = headers.next().ok_or("No status line found")?.to_string();

    let mut header_map = HashMap::new();
    for line in headers {
        if let Some((key, value)) = line.split_once(": ") {
            header_map.insert(key.to_string(), value.to_string());
        }
    }

    Ok(HttpResponse {
        status_line,
        headers: header_map,
        body: if include_body { body } else { String::new() },
    })
}

// Wrapper functions for specific use cases
fn http_get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = http_request(url, "GET", true, None/* :Option<Duration> */)?;
    Ok(response.body)
}

fn http_get_head(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = http_request(url, "HEAD", false, None)?;
    Ok(response.status_line)
}

fn http_get_full(url: &str) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    http_request(url, "GET", true, None/* Option<Duration> */)
}

// Call osascript to say something in Chinese
fn apple_say_using(text: &str, voice_using: Option<&str>) {
    let script = format!(r#"osascript -e 'say "{}" using "{})"'"#, text, voice_using.unwrap_or("Tingting"));
    dbg!(&script);
    if let Err(e) = Command::new("sh").arg("-c").arg(&script).status() {
        eprintln!("Failed to execute osascript: {}", e);
    }
}

fn load_question_pool() -> Vec<Question> {
    if USE_LOCAL {
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
            _ => {
                eprintln!("Error: Unknown error");
                vec![]
            }
        }
    } else {
        // Keep the existing code for the non-local case
        vec![]
    }
}

fn check_internet_connection() {
    print!("Checking internet connection... ");
    io::stdout().flush().unwrap();

    let url = "http://www.google.com";
    let timeout = Some(Duration::from_secs(5));

    match http_request(url, "HEAD", false, timeout) {
        Ok(response) => {
            if response.status_line.starts_with("HTTP/1.1 200") {
                println!("Connected!");
            } else {
                println!("\x1b[31mFailed to connect. Unexpected response: {}\x1b[0m", response.status_line);
                process::exit(1);
            }
        }
        Err(e) => {
            println!("\x1b[31mFailed to connect. No internet connection detected. Please check your connection.\x1b[0m");
            println!("Error details: {}", e);
            process::exit(1);
        }
    }
}

fn banner() {
    let art = r#"
    _____ ______ _    _      __  __            _      _______        _   
  / ____|  ____| |  | |    |  \/  |          | |    |__   __|      | |  
 | |    | |__  | |__| |    | \  / | ___   ___| | __    | | ___  ___| |_ 
 | |    |  __| |  __  |    | |\/| |/ _ \ / __| |/ /    | |/ _ \/ __| __|
 | |____| |____| |  | |    | |  | | (_) | (__|   <     | |  __/\__ \ |_ 
  \_____|______|_|  |_|    |_|  |_|\___/ \___|_|\_\    |_|\___||___/\__|
                                                                   v1.5
    Quiz by H3LLKY4T                                                      
                          
 For Certified Ethical Hacker v12                    Last Updated April 2024                             
----------------------------------------------------------------------------
    "#;
    println!("{}", art);
}

fn choose_questions(pool: &Vec<Question>, num_questions: usize) -> Vec<Question> {
    let mut rng = rand::thread_rng();
    pool.choose_multiple(&mut rng, num_questions).cloned().collect()
}

fn ask_question(question_number: usize, question: &Vec<TypedQuestion>, ca: &str, options: &Vec<String>, hint:Option<&str>) -> bool {
    sleep(Duration::from_micros(300));
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear screen
    banner();
    println!("Question {}\n\n{}", question_number, question);
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
                if options[cor - 1] == ca {
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


fn check_for_duplicates(question_pool: &Vec<Question>) -> Vec<(QuestionType, Question)> {
    let mut typed_questions: Vec<(QuestionType, Question)> = Vec::new();
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
                    typed_questions.push((QuestionType::DuplicateNodes, (*question).clone()));
                } else {
                    typed_questions.push((QuestionType::DuplicateQuestions, (*question).clone()));
                }
            }
        } else {
            typed_questions.push((QuestionType::DefaultItem, (*questions.first().unwrap()).clone()));
        }
    }

    typed_questions
}



fn get_num_questions() -> Option<usize> {
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

fn run_quiz(question_pool: Vec<Question>, num_questions: Option<usize>) {

    let typed_questions = check_for_duplicates(&question_pool);

    // Filter and debug the results
    let duplicates: Vec<_> = typed_questions
        .iter()
        .filter(|(qtype, _)| matches!(qtype, QuestionType::DuplicateQuestions))
        .cloned()
        .collect();
    dbg!(&duplicates);

    let duplicate_nodes: Vec<_> = typed_questions
        .iter()
        .filter(|(qtype, _)| matches!(qtype, QuestionType::DuplicateNodes))
        .cloned()
        .collect();
    dbg!(&duplicate_nodes);

    let mut score = 0; // safely destroyed after the if statement
    println!("{} duplicate questions found in the question pool. Please remove them before starting the quiz.", duplicates.len());
    for (i, dup_question) in duplicates.iter().enumerate() {
        println!("{}. {}:{:?}", i + 1, dup_question.1.question, dup_question.0);


    }
    let mut score = 0;
    let sq = match num_questions {
        Some(n) => choose_questions(&question_pool, n),
        None => choose_questions(&question_pool, 125.min(question_pool.len())),
    };

    for (i, question) in typed_questions.iter().enumerate() {
        if ask_question(i + 1, &question) {
            score += 1;
        }
        println!();
    }

    println!(
        "Quiz completed! Your score is {} out of {}",
        score,
        sq.len()
    );
    println!(
        "Your percentage is {:.2}%",
        (score as f64 / sq.len() as f64) * 100.0
    );
    println!("Press Ctrl + C to Exit");
}

fn main() {
    check_internet_connection();
    // say_in_chinese("欢迎来到黑客测验！");
    let question_pool = load_question_pool();
    let num_questions = get_num_questions();
    run_quiz(question_pool, num_questions);
}