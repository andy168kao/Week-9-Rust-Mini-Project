Generating Parentheses using Rust

This program implements an algorithm to generate all combinations of well-formed parentheses using Rust programming language. The algorithm uses backtracking by adding left and right parentheses to generate all possible combinations and then filters out the well-formed ones.

How to Use

Requirements
Rust programming language compiler
Running the Program
The program can be compiled using the Rust compiler to generate an executable file that can be run from the command line.

Here are the steps to compile and run the program using the Rust compiler:

Download the source code for the program.
Open the command line and navigate to the directory where the source code is located.
Run the following command to compile the program:
arduino
Copy code
cargo build --release
Once the compilation is complete, navigate to the target/release directory to find the executable file named generate-parentheses.
Navigate to the target/release directory in the command line.
Run the following command to execute the program:
bash
Copy code
./generate-parentheses
The program will prompt you to enter the number of parentheses pairs to generate. Enter the value and press Enter. The program will output all combinations of well-formed parentheses.
Program Structure
This program consists of the following files:

src/main.rs: Main entry point of the program containing the main function.
src/generate_parenthesis.rs: Implementation of the algorithm to generate all combinations of well-formed parentheses.
Program Arguments
This program does not accept any arguments.

Output Format
The program outputs all combinations of well-formed parentheses, with each combination on a new line.