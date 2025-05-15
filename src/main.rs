// Import project modules
mod tokenizer;
mod pratt;
mod parser;
mod ast;

// Import standard IO for reading user input and flushing output
use std::io::{self, Write};

// Import the tokenizer components
use tokenizer::{Tokenizer, Token};

// Import the SQLParser to parse the tokens into SQL AST
use parser::SQLParser;

/// Entry point for the Mini SQL Parser CLI application.
fn main() {
    // Greeting message
    println!("🔷Welcome to the Mini SQL Parser command-line tool");
    println!("Enter your SQL query below, or type 'exit' to leave.\n");

    // Begin a REPL-style input loop
    loop {
        // Prompt the user for input
        print!("sql> ");
        io::stdout().flush().unwrap(); // Flush to ensure prompt is displayed

        // Read the input query from the user
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("❌Couldn't read the input."); // Handle read error
            continue;
        }

        // Trim whitespace from input and check for exit command
        let input = input.trim();
        if input.is_empty() {
            continue; // Ignore empty input
        }
        if input.eq_ignore_ascii_case("exit") {
            println!("👋 closing the SQL parser. Bye!");
            break; // Exit the loop and end the program
        }

        // Tokenize the user input into a list of SQL tokens
        let mut tokenizer = Tokenizer::new(input);
        let mut tokens = Vec::new();

        loop {
            let token = tokenizer.next_token();
            if token == Token::Eof {
                tokens.push(token); // Push EOF token and break
                break;
            }
            tokens.push(token); // Push valid token to token list
        }

        // Optional: Uncomment to debug tokens
        // println!("🔹 Tokens: {:?}", tokens);

        // Parse the tokens into a SQL AST (Abstract Syntax Tree)
        let mut parser = SQLParser::new(&tokens);
        match parser.parse_statement() {
            Ok(statement) => {
                // Successfully parsed SQL statement
                println!("✅ Processed Statement:\n{:#?}\n", statement);
            }
            Err(e) => {
                // Error while parsing SQL
                eprintln!("❌ Parse Error: {}\n", e);
            }
        }
    }
}

