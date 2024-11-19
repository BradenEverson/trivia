//! Main Driver for Trivia Object Structs and Interfacing

use rand::seq::SliceRandom;
use wasm_bindgen::prelude::wasm_bindgen;

/// The Main Driver for generating new questions for the day
#[wasm_bindgen]
pub struct TriviaGenerator {
    questions: Vec<Question>,
}

impl TriviaGenerator {
    /// Generates a new TriviaGenerator by  reading the existing files
    pub fn new() -> Self {
        let mut questions = vec![];

        Self { questions }
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
#[derive(Clone)]
pub struct Question {
    /// The question's name
    name: String,
    /// The possible answers
    answers: [String; 4],
    /// The correct answer index
    correct: String,
}

impl Question {
    /// Returns the question's name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns the question answers
    pub fn get_answers(&self) -> &[String] {
        &self.answers
    }

    /// Given an answer, checks if that is the correct one
    pub fn is_correct(&self, chosen: usize) -> bool {
        self.answers[chosen] == self.correct
    }
}
