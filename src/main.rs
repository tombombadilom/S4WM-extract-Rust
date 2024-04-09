
use indicatif::{ProgressBar, ProgressStyle};
use pdf_extract::extract_text;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::borrow::Cow;

/**
 * This code snippet demonstrates a Rust program that extracts questions from a PDF file, parses them, validates them,
 * and saves them to a JSON file.
 * 
 * The program uses the following external crates:
 * - `indicatif`: for displaying progress bars
 * - `pdf_extract`: for extracting text from a PDF file
 * - `regex`: for regular expression matching
 * - `serde`: for serializing and deserializing JSON
 * - `reqwest`: for making HTTP requests
 * 
 * The program defines the following structs:
 * - `Question`: represents a question with its number, text, choices, and correct answers
 * - `OutputError`: represents an error that can occur during the output process
 * 
 * The program also defines the following functions:
 * - `save_to_json`: saves the questions to a JSON file
 * - `download_pdf`: downloads a PDF file from a given URL
 * - `parse_questions`: parses the questions from the extracted text
 * - `clean_text`: cleans the text by replacing "<br>" tags with spaces
 * - `validate_questions`: validates the questions
 * - `async_main`: the main asynchronous function that orchestrates the program flow
 * - `main`: the main entry point that runs the asynchronous main function
 * 
 * To use the program, provide the path to the PDF file or the URL of the PDF file. The program will download the PDF
 * file if it doesn't exist locally. It will then extract the text from the PDF file, parse the questions, validate
 * them, and save them to a JSON file.
 */

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DIGIT_REGEX: Regex = Regex::new(r"^\d+\.").unwrap();
    static ref CHOICE_REGEX: Regex = Regex::new(r"^[A-D]\.").unwrap();
    static ref BR_REGEX: Regex = Regex::new(r"<br\s*/?>").unwrap();
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Question {
    number: String,
    text: String,
    choices: HashMap<String, String>,
    correct_answers: Option<usize>,
}
#[derive(Debug)]
pub struct OutputError {
    message: String,
    // Consider including the source error as well:
    // source: Option<Box<dyn Error>>,
}

impl Error for OutputError {}

impl std::fmt::Display for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<&str> for OutputError {
    fn from(msg: &str) -> Self {
        OutputError {
            message: msg.to_string(),
        }
    }
}

impl From<std::io::Error> for OutputError {
    fn from(error: std::io::Error) -> Self {
        OutputError {
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for OutputError {
    fn from(error: serde_json::Error) -> Self {
        OutputError {
            message: error.to_string(),
        }
    }
}

impl From<reqwest::Error> for OutputError {
    fn from(error: reqwest::Error) -> Self {
        OutputError {
            message: error.to_string(),
        }
    }
}

impl From<regex::Error> for OutputError {
    fn from(error: regex::Error) -> Self {
        OutputError {
            message: error.to_string(),
        }
    }
}

impl From<pdf_extract::OutputError> for OutputError {
    fn from(error: pdf_extract::OutputError) -> Self {
        OutputError {
            message: error.to_string(),
        }
    }
}

fn save_to_json(questions: &[Question], output_path: &str) -> Result<(), OutputError> {
    let file = File::create(output_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, questions)?;
    Ok(())
}

async fn download_pdf(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;
    Ok(content.to_vec())
}

fn parse_questions(full_text: &str) -> Result<Vec<Question>, regex::Error> {
    let mut questions = Vec::new();
    let mut current_question: Option<Question> = None;
    let mut question_number = 1;

    let lines = full_text.split('\n');
    let digit_regex = Regex::new(r"^\d+\.")?;
    let choice_regex = Regex::new(r"^[A-D]\.")?;

    for line in lines {
        let cleaned_line = clean_text(line);
        if cleaned_line.is_empty() {
            continue;
        }

        if digit_regex.is_match(&cleaned_line) {
            if let Some(q) = current_question.take() {
                questions.push(q);
            }
            current_question = Some(Question {
                number: question_number.to_string(),
                text: String::new(),
                choices: HashMap::new(),
                correct_answers: None,
            });
            question_number += 1;
        } else if let Some(ref mut question) = current_question {
            if choice_regex.is_match(&cleaned_line) {
                let (answer_letter, text_without_answer) = cleaned_line.split_at(2);
                question.choices.insert(answer_letter.trim().to_string(), text_without_answer.trim().to_string());
            } else {
                question.text.push_str(&cleaned_line);
            }
        }
    }

    if let Some(q) = current_question {
        questions.push(q);
    }

    Ok(questions)
}

fn clean_text(text: &str) -> String {
    BR_REGEX.replace_all(text, " ").trim().into()
}

// Function validate_questions is assumed to be implemented correctly
fn validate_questions(_questions: &[Question]) -> Result<(), OutputError> {
    // Assuming implementation here that checks questions and possibly modifies them
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    async_main().await
}

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let pdf_path = "./C_S4EWM_2020 - Extended Warehouse Management with SAP S4HANA.pdf";
    
    if !PathBuf::from(&pdf_path).exists() {
        let pdf_url = "https://cdn.filestackcontent.com/pTHCm0vSbiGJkwM74n1H";
        let pdf_data = download_pdf(pdf_url).await?;
        fs::write(&pdf_path, &pdf_data)?;
    }

    let pdf_pages = extract_text(&pdf_path)?; // Handle this Result as well
    let progress_bar = ProgressBar::new_spinner();

    // Correct way to set the style for the progress bar
   let style = ProgressStyle::default_spinner()
    .template("{spinner:.green} [{elapsed_precise}] {wide_msg}")?
    .tick_strings(&["-", "\\", "|", "/"]);
    
    progress_bar.set_style(style);
    

    let update_frequency = 5;
    let time_update_frequency = Duration::from_millis(500);
    let mut last_update = Instant::now(); // Assuming last_update should start at now

    let (all_questions, total_questions_parsed) = pdf_pages.lines().enumerate().try_fold(
        (Vec::new(), 0),
        |(mut all_questions_acc, mut total_questions_parsed_acc), (page_number, text)| -> Result<_, Box<dyn std::error::Error>> {
            let questions = parse_questions(text)?;
            total_questions_parsed_acc += questions.len();
            all_questions_acc.extend(questions);
    
            if page_number % update_frequency == 0 || last_update.elapsed() >= time_update_frequency {
                // Directly set the leaked message into the progress bar
                let msg = format!(
                    "Processing page {} (total questions: {})",
                    page_number + 1,
                    total_questions_parsed_acc
                );
                let static_str: &'static str = Box::leak(msg.into_boxed_str());
                let cow_message: Cow<'static, str> = Cow::Borrowed(static_str);
                progress_bar.set_message(cow_message);
                progress_bar.tick(); // Update the progress bar
                last_update = Instant::now(); // Reset the last update time
            }
    
            Ok((all_questions_acc, total_questions_parsed_acc)) // Continue folding
        },
    )?;
   
    let completion_message = format!("Processing complete: {} questions processed", total_questions_parsed).into_boxed_str();
    let cow_message: Cow<'static, str> = Cow::Borrowed(Box::leak(completion_message));
    progress_bar.finish_with_message(cow_message); // Use cow_message, which satisfies the trait bound
    
    validate_questions(&all_questions)?;

    // Save the validated questions to JSON
    let output_path = "json/questions.json";
    let output_path_buf = PathBuf::from(output_path);
    let output_dir = output_path_buf.parent().ok_or_else(|| OutputError::from("Failed to get parent directory"))?;
    
    // Check if the directory exists and create it if not
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    save_to_json(&all_questions, output_path)
        .map_err(|e| e.into()) // Convert OutputError into Box<dyn Error>

    // No need for Ok(()) since save_to_json already returns a Result<(), Box<dyn std::error::Error>>
}
