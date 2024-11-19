//! Main Driver for Trivia Object Structs and Interfacing

use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use cached_questions::CACHED_QUESTIONS;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

pub mod cached_questions;

/// The Main Driver for generating new questions for the day
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct TriviaGenerator {
    /// All possible questions
    questions: Vec<Question>,
}

#[wasm_bindgen]
impl TriviaGenerator {
    /// Creates a new Trivia set from the cached JSON
    pub fn new() -> Option<Self> {
        serde_json::from_str(CACHED_QUESTIONS).ok()?
    }
    /// Generates a new TriviaGenerator by  reading the existing files
    pub fn generate() -> Option<Self> {
        let mut questions = vec![];

        let geography_questions = File::open("data/geography").ok()?;

        let mut geo_lines = BufReader::new(geography_questions).lines();

        while let Some(Ok(line)) = geo_lines.next() {
            if line.starts_with("#Q") {
                let question = line[3..].to_string();
                let answer = geo_lines.next()?.ok()?[2..].to_string();
                let a = geo_lines.next()?.ok()?[2..].to_string();
                let b = geo_lines.next()?.ok()?[2..].to_string();
                let c = match geo_lines.next()?.ok()?.trim() {
                    "" => None,
                    other => Some(other[2..].to_string()),
                };
                let d = match &c {
                    None => None,
                    Some(_) => match geo_lines.next()?.ok()?.trim() {
                        "" => None,
                        other => Some(other[2..].to_string()),
                    },
                };

                let question = Question {
                    name: question,
                    answers: [Some(a), Some(b), c, d],
                    correct: answer,
                };

                questions.push(question)
            }
        }

        let res = Self { questions };
        let serialized = serde_json::to_string(&res).ok()?;
        let mut out = File::create_new("data/geography.json").ok()?;
        out.write_all(serialized.as_bytes()).ok()?;

        Some(res)
    }

    /// Get's today's question
    pub fn get_question(&mut self) -> Question {
        let mut rng = rand::thread_rng();
        self.questions
            .choose(&mut rng)
            .expect("Failed to get question")
            .clone()
    }
}

/// A question, it's answers and a correct answer
#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Question {
    /// The question's name
    name: String,
    /// The possible answers
    answers: [Option<String>; 4],
    /// The correct answer index
    correct: String,
}

#[wasm_bindgen]
impl Question {
    /// Returns the question's name
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Returns the question answers
    pub fn get_answers(&self) -> Vec<String> {
        self.answers
            .iter()
            .filter_map(|answer| answer.clone())
            .collect()
    }

    /// Given an answer, checks if that is the correct one
    pub fn is_correct(&self, chosen: String) -> bool {
        chosen == self.correct
    }

    /// Gets the correct answer
    pub fn get_correct_answer(&self) -> String {
        self.correct.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::TriviaGenerator;

    #[test]
    fn cached_loading_is_valid() {
        let _ = TriviaGenerator::generate();
        TriviaGenerator::new().expect("Failed to load generator");
    }
}
