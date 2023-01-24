extern crate dotenv;

use dotenv::dotenv;
use std::env;
use openai_api_client::*;
use rand::seq::SliceRandom;


#[actix_rt::main]
async fn main() {


    println!("\r\nPlease enter a topic of the lesson:");
    println!("");

    let mut subject = String::new();
    std::io::stdin().read_line(&mut subject).unwrap();
    let subject = subject.trim();

    let question = format!("Give list of words linked with the subject \"{}\". Words should comma separated. Only words.", subject);
    println!("\r\nPlease wait 5 seconds...");
    let words_str = ask(question.as_str(), 800).await;
    let words_str_trim = words_str.replace(".", "").replace("\r", "").replace("\n", "");
    println!("\r\nWords of lesson:");

    let mut words: Vec<String> = words_str_trim.split(",").map(|s| s.to_string().trim().to_string()).collect();
    let mut i = 1;
    for word in words.iter() {
        println!("{}. {}", i, word);
        i = i + 1;
    }

    println!("\r\nDo you know all these words? Say: Yes or write unknown words comma separated:");
    let mut unknown_words_str = String::new();
    println!("");

    std::io::stdin().read_line(&mut unknown_words_str).unwrap();
    let unknown_words: Vec<String> = if unknown_words_str.starts_with("yes") || unknown_words_str.starts_with("Yes") {
        Vec::new()
    } else {
        unknown_words_str.split(",").map(|s| s.to_string().trim().to_string()).collect()
    };

    if unknown_words.len() > 0 {
        println!("\r\nPlease wait {} seconds...", unknown_words.len() * 5);
    }
    for word in unknown_words {
        print!("\r\n{}\r\n", word_capital(&word));
        let question = format!("Give answer what is \"{}\" from the field \"{}\"?", word, subject);
        let explanation = ask(question.as_str(), 200).await;
        print!("{}\r\n", print_with_line_break(explanation.replace("\r", "").replace("\n", "").as_str()));
        let example_1:String = ask(format!("Give sentence with word \"{}\" in field \"{}\"", word, subject).as_str(),200).await;
        print!("Example 1: {}\r\n", print_with_line_break(example_1.replace("\r", "").replace("\n", "").as_str()));
        let example_2:String = ask(format!("Give rude sentence with word \"{}\" in field \"{}\"", word, subject).as_str(), 200).await;
        print!("Example 2: {}\r\n", print_with_line_break(example_2.replace("\r", "").replace("\n", "").as_str()));

    }
    let mut rng = rand::thread_rng();
    words.shuffle(&mut rng);

    let mut answer_has_question = false;
    let mut corrected:String = "".to_string();
    while !words.is_empty() {
        if answer_has_question {
            answer_has_question = false;
            let random_sentence_type = random_sentence_type();
            let question = format!("Give {} answer on text: {}", random_sentence_type, corrected);
            let answer = ask(question.as_str(), 200).await;
            println!("\r\n{}", print_with_line_break(answer.replace("\r", "").replace("\n", "").as_str()));

        } else {
            let word = words.pop().unwrap().to_lowercase();
            let question_type = random_question_type();
            let generated_question = format!("Give {}. About  \"{}\". Use word \"{}\". Do not use word \"{}\"", question_type, subject, word, subject);
            let question = ask(generated_question.as_str(), 200).await;
            println!("\r\n{}", print_with_line_break(question.replace("\r", "").replace("\n", "").as_str()));
            let mut answer = String::new();
            
            println!("");
            std::io::stdin().read_line(&mut answer).unwrap();
            answer = answer.trim().to_string();
            corrected = correct(answer.as_str(), "Fix grammar and spelling").await;
            corrected = corrected.replace("\r", "").replace("\n", "");
            corrected = correct_dot(corrected.as_str());
            if !corrected.contains(&answer) || !is_end_dot_or_question_mark(&answer) {
                println!("{} < Grammar fixes", corrected);
            }
            if corrected.contains("?") {
                answer_has_question = true;
            }

        }
    }
}

fn word_capital(word: &str) -> String {
    let mut word_clone = word.to_string();
    let first_letter = word_clone.chars().next().unwrap();
    let first_letter_uppercase = first_letter.to_uppercase().to_string();
    word_clone.replace_range(0..1, &first_letter_uppercase);
    word_clone
}

fn correct_dot(input_string: &str) -> String {
    // println!("input_string: {}", input_string);
    let mut result = input_string.to_string();
    if !result.ends_with(".") && !result.ends_with("?") {
        result = format!("{}.", result);
    }
    result
}

fn is_end_dot_or_question_mark(input_string: &str) -> bool {
    input_string.ends_with(".") || input_string.ends_with("?")
}

fn random_question_type() -> String {
    let mut types: Vec<String> = question_types();
    let mut rng = rand::thread_rng();
    types.shuffle(&mut rng);
    types[0].clone()
}

fn question_types() -> Vec<String> {
    let types: Vec<String> = vec![
        "Yes/No question".to_string(),
        "Wh-question".to_string(),
        "Choice question".to_string(),
        "Open-ended question".to_string(),
        "Hypothetical question".to_string(),
        "Leading question".to_string(),
        "Rhetorical question".to_string(),
        "Funnel question".to_string(),
        "Probing question".to_string(),
        "Clarifying question".to_string(),
        "Follow-up question".to_string(),
        "Forced-choice question".to_string(),
        "Matching question".to_string(),
        "Categorical question".to_string(),
        "Comparison question".to_string(),
        "Rude question".to_string(),
        "Polite question".to_string(),
    ];
    types
}


fn random_sentence_type() -> String {
    let mut types: Vec<String> = sentence_types();
    let mut rng = rand::thread_rng();
    types.shuffle(&mut rng);
    types[0].clone()
}


fn sentence_types() -> Vec<String> {
    let types: Vec<String> = vec![
        "Declarative".to_string(),
        "Imperative".to_string(),
        "Assertive".to_string(),
        "Optative".to_string(),
        "Exhortative".to_string(),
        "Descriptive".to_string(),
        "Expository".to_string(),
        "Complex".to_string(),
        "Polite".to_string(),
        "Rude".to_string(),
    ];
    types
}

async fn ask(question: &str, words: u32) -> String {
    dotenv().ok();
    let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");

    let model = "text-davinci-003";
    completions_pretty(question, model, words, &api_key).await
}


async fn correct(input: &str, instruction: &str) -> String {
    dotenv().ok();
    let api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY must be set");

    let model = "text-davinci-edit-001";
    edits_pretty(input, instruction, model, &api_key).await
}

fn print_with_line_break(text: &str) ->String {
    let mut result = String::new();
    let mut current_line = String::new();
    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > 90 {
            if !result.is_empty() {
                result.push_str("\r\n");
            }
            result.push_str(&current_line);
            current_line = String::new();
        }

        if !current_line.is_empty() {
            current_line.push(' ');
        }

        current_line.push_str(word);
    }

    if !result.is_empty() {
        result.push_str("\r\n");
    }
    result.push_str(&current_line);
    result
}