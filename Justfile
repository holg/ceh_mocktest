# Define project path
project_path := "/Volumes/tb3ssd/develeop/ceh/rust/CEH-mocktest"

# Change to project directory
@cd {{project_path}}

# Default recipe, will run if no recipe name is provided
default:
echo "Available commands: build, test, clean, run, fmt, clippy"

# Recipe to build the project
build:
echo "Building project..."
cargo build --release

# Recipe to run tests
test:
echo "Running tests..."
cargo test

# Recipe to clean the project
clean:
echo "Cleaning project..."
cargo clean

# Recipe to run the project
run:
echo "Running project..."
cargo run

# Recipe to format the code
fmt:
echo "Formatting the code..."
cargo fmt

# Recipe to run clippy for linting
clippy:
echo "Running clippy..."
cargo clippy
