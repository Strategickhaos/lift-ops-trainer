//! Visitor pattern for walking the knowledge AST.
//! Future renderers implement this (or just match on Document).

use super::*;

pub trait AstVisitor {
    fn visit_document(&mut self, doc: &Document) {
        match doc {
            Document::Checklist(c) => self.visit_checklist(c),
            Document::FlashDeck(f) => self.visit_flash_deck(f),
            Document::Quiz(q) => self.visit_quiz(q),
            Document::CapacityRules(r) => self.visit_capacity_rules(r),
        }
    }

    fn visit_checklist(&mut self, _c: &ChecklistDoc) {}
    fn visit_flash_deck(&mut self, _f: &FlashDeck) {}
    fn visit_quiz(&mut self, _q: &QuizDoc) {}
    fn visit_capacity_rules(&mut self, _r: &CapacityRules) {}
}
