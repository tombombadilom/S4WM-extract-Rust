# S4WM-extract-Rust
This code snippet demonstrates a Rust program that extracts questions from a PDF file, parses them, validates them,
and saves them to a JSON file.

The program uses the following external crates:
- `indicatif`: for displaying progress bars
- `pdf_extract`: for extracting text from a PDF file
- `regex`: for regular expression matching
- `serde`: for serializing and deserializing JSON
- `reqwest`: for making HTTP requests

The program defines the following structs:
- `Question`: represents a question with its number, text, choices, and correct answers
- `OutputError`: represents an error that can occur during the output process

The program also defines the following functions:
- `save_to_json`: saves the questions to a JSON file
- `download_pdf`: downloads a PDF file from a given URL
- `parse_questions`: parses the questions from the extracted text
- `clean_text`: cleans the text by replacing "<br>" tags with spaces
- `validate_questions`: validates the questions
- `async_main`: the main asynchronous function that orchestrates the program flow
- `main`: the main entry point that runs the asynchronous main function

To use the program, provide the path to the PDF file or the URL of the PDF file. The program will download the PDF
file if it doesn't exist locally. It will then extract the text from the PDF file, parse the questions, validate
them, and save them to a JSON file.
