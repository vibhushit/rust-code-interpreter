# Rust Code Interpreter

A simple command-line application that allows users to input, compile, and execute Rust code dynamically. This program provides an interactive experience, resembling an interpreter, by allowing users to write Rust code line by line and see the output immediately.

## Features

- **Interactive Input**: Enter Rust code line by line.
- **Immediate Compilation**: Compiles and executes code after each line of input.
- **Clear Command**: Clear the current input file and start fresh.
- **Exit Command**: Exit the program gracefully.
- **Error Handling**: Provides feedback for compilation errors.

## Prerequisites

To run this project, you need to have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (including `rustc` and `cargo`)
- A terminal or command prompt

## Getting Started

### Clone the Repository

```bash
git clone https://github.com/vibhushit/rust-code-interpreter.git
cd rust-code-interpreter
```

### Run the Application

1. Open your terminal and navigate to the project directory.
2. Run the application using the following command:

```bash
cargo run
```

### Usage

- After starting the application, you will be prompted to enter Rust code.
- Type your Rust code and press Enter. The program will compile and execute the code immediately.
- To clear the input file, type `clear` and press Enter.
- To exit the program, type `exit` and press Enter.

### Example Input

```rust
fn main() {
    println!("Hello, World!");
}
```

### Output

The output will be displayed in the terminal after the code is executed.

## Contributing

Contributions are welcome! If you have suggestions for improvements or features, feel free to open an issue or submit a pull request.

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/YourFeature`).
3. Commit your changes (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature/YourFeature`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Rust Programming Language](https://www.rust-lang.org/)