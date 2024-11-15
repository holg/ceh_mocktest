# CEH Mock Test

This project is a Certified Ethical Hacker (CEH) mock test application built with Rust. It provides a platform for users to practice and prepare for the CEH certification exam.

## Features

- Multiple-choice questions covering various CEH exam topics
- Randomized question selection
- Instant feedback on answers
- Scoring system
- Hint support for some questions

## Project Structure

The project is organized as follows:

- `src/`: Contains the main source code files
    - `main.rs`: The entry point of the application
    - `questions.rs`: Handles question-related functionality
    - `helper/`: Additional helper modules
        - `ceh_lab.rs`: CEH lab-related functionality
        - `htr_low_level_http.rs`: Low-level HTTP functionality
        - `mod.rs`: Module declarations
        - `ollama.rs`: Ollama-related functionality
        - `quiz.rs`: Quiz-related functionality
- `questions.json`: Database of CEH mock test questions

## Dependencies

The project uses several external crates, including:

- `rand`: For random number generation
- `colored`: For colorized console output
- `reqwest`: For making HTTP requests
- `serde_json`: For JSON serialization/deserialization
- `tokio`: For asynchronous runtime
- `anyhow`: For error handling

For a complete list of dependencies, refer to the `Cargo.toml` file.

## Building and Running

To build and run the project:

1. Ensure you have Rust and Cargo installed on your system.
2. Clone this repository.
3. Navigate to the project directory.
4. Run `cargo build` to compile the project.
5. Run `cargo run` to start the application.

## Contributing

Contributions to improve the mock test or add more questions are welcome. Please feel free to submit pull requests or open issues for any bugs or feature requests.

## License

[Insert your chosen license here]

## Disclaimer

This mock test is intended for practice purposes only and is not affiliated with or endorsed by EC-Council or the official CEH certification program.