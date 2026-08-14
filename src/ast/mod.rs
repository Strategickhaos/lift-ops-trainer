//! Abstract Syntax Tree for the training knowledge base.
//! Every renderer walks this tree.

use serde::{Deserialize, Serialize};

pub mod node;
pub mod visitor;

pub use node::*;
pub use visitor::*;

/// Top-level document that a content file deserializes into.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Document {
    Checklist(ChecklistDoc),
    FlashDeck(FlashDeck),
    Quiz(QuizDoc),
    CapacityRules(CapacityRules),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistDoc {
    pub machine: String,          // "boom" | "forklift"
    pub title: String,
    pub groups: Vec<ChecklistGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistGroup {
    pub name: String,
    pub items: Vec<ChecklistItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub text: String,
    #[serde(default)]
    pub critical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashDeck {
    pub title: String,
    pub cards: Vec<FlashCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashCard {
    pub front: String,
    pub back: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizDoc {
    pub title: String,
    pub questions: Vec<QuizItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizItem {
    pub prompt: String,
    pub options: Vec<String>,
    pub answer: usize,            // 0-based index
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityRules {
    pub default_load_center_in: f64,
    pub notes: Vec<String>,
}
