# Define project path
project_path := "/Volumes/tb3ssd/develeop/ceh/rust/CEH-mocktest"
# Default recipe, will run if no recipe name is provided

default:
    echo "Available commands: build, test, clean, run, fmt, clippy"
# Recipe to build the project
build:
    echo "Building project..."
    cd {{project_path}} && cargo build --release

# Recipe to run tests
test:
    echo "Running tests..."
    cd {{project_path}} && cargo test

# Recipe to clean the project
clean:
    echo "Cleaning project..."
    cd {{project_path}} && cargo clean

# Recipe to run the project
run:
    echo "Running project..."
    cd {{project_path}} && cargo run

# Recipe to format the code
fmt:
    echo "Formatting the code..."
    cd {{project_path}} && cargo fmt

# Recipe to run clippy for linting
clippy:
    echo "Running clippy..."
    cd {{project_path}} && cargo clippy