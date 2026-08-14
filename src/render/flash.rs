//! Tachistoscope / RSVP flash renderer.
//! High-speed presentation of critical facts for pattern memory.

use crate::ast::FlashCard;

pub struct FlashConfig {
    pub ms_per_card: u64,
    pub repetitions: u32,
    pub shuffle: bool,
}

impl Default for FlashConfig {
    fn default() -> Self {
        Self {
            ms_per_card: 800,
            repetitions: 2,
            shuffle: true,
        }
    }
}

pub fn plan_sequence(cards: &[FlashCard], cfg: &FlashConfig) -> Vec<FlashCard> {
    let mut seq = cards.to_vec();
    if cfg.shuffle {
        // simple deterministic shuffle placeholder — replace with rand later
        seq.reverse();
    }
    let mut out = Vec::new();
    for _ in 0..cfg.repetitions {
        out.extend(seq.iter().cloned());
    }
    out
}
