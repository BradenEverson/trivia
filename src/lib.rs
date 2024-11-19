//! Main Driver for Trivia Object Structs and Interfacing

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rand::seq::SliceRandom;
use wasm_bindgen::prelude::wasm_bindgen;

/// The Main Driver for generating new questions for the day
#[wasm_bindgen]
pub struct TriviaGenerator {
    /// All possible questions
    questions: Vec<Question>,
}

impl TriviaGenerator {
    /// Generates a new TriviaGenerator by  reading the existing files
    pub fn new() -> Option<Self> {
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
                    Some(_) => {
                        geo_lines.next()?.ok()?[2..].to_string();
                        match geo_lines.next()?.ok()?.trim() {
                            "" => None,
                            other => Some(other[2..].to_string()),
                        }
                    }
                };

                let question = Question {
                    name: question,
                    answers: [Some(a), Some(b), c, d],
                    correct: answer,
                };

                questions.push(question)
            }
        }

        Some(Self { questions })
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
#[derive(Clone, Debug)]
pub struct Question {
    /// The question's name
    name: String,
    /// The possible answers
    answers: [Option<String>; 4],
    /// The correct answer index
    correct: String,
}

impl Question {
    /// Returns the question's name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the question answers
    pub fn get_answers(&self) -> &[Option<String>] {
        &self.answers
    }

    /// Given an answer, checks if that is the correct one
    pub fn is_correct(&self, chosen: usize) -> bool {
        if let Some(chosen) = &self.answers[chosen] {
            chosen == &self.correct
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TriviaGenerator;

    #[test]
    fn ensure_trivia_generation() {
        TriviaGenerator::new().expect("Failed to generate trivia");
    }
}
