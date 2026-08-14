//! Enumerator: turns an AST into linear sequences
//! (flash order, quiz order, checklist order, etc.).

use crate::ast::{Document, FlashCard, ChecklistItem};

pub fn flash_sequence(doc: &Document) -> Vec<FlashCard> {
    match doc {
        Document::FlashDeck(d) => d.cards.clone(),
        _ => vec![],
    }
}

pub fn critical_items(doc: &Document) -> Vec<ChecklistItem> {
    match doc {
        Document::Checklist(c) => c
            .groups
            .iter()
            .flat_map(|g| g.items.iter())
            .filter(|i| i.critical)
            .cloned()
            .collect(),
        _ => vec![],
    }
}
