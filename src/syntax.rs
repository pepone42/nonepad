use once_cell::sync::Lazy;
use std::ops::Range;
use syntect::{
    highlighting::{HighlightState, Highlighter, RangedHighlightIterator, Style},
    parsing::{ParseState, ScopeStack, SyntaxReference, SyntaxSet},
};

use crate::{text_buffer::buffer::Buffer, theme::THEME};

pub static SYNTAXSET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);

#[derive(Debug)]
pub struct StateCache {
    states: Vec<(ParseState, HighlightState)>,
    highlighter: Highlighter<'static>,
    highlighted_line: Vec<Vec<(Style, Range<usize>)>>,
}

impl StateCache {
    pub fn new() -> Self {
        StateCache {
            states: Vec::new(),
            highlighter: Highlighter::new(&THEME.style),
            highlighted_line: Vec::new(),
        }
    }

    pub fn update_range(&mut self, syntax: &SyntaxReference, buffer: &Buffer, start: usize, end: usize) {
        // states are cached every 16 lines
        let start = (start >> 4).min(self.states.len());
        let end = (end.min(buffer.len_lines()) >> 4) + 1;

        self.states.truncate(start);
        self.highlighted_line.truncate(start << 4);
        let mut states = self.states.last().cloned().unwrap_or_else(|| {
            (
                ParseState::new(syntax),
                HighlightState::new(&self.highlighter, ScopeStack::new()),
            )
        });

        for i in start << 4..(end << 4).min(buffer.len_lines()) {
            let str = buffer.line(i).to_string(&buffer);
            let ops = states.0.parse_line(&str, &SYNTAXSET);
            let h: Vec<(Style, Range<usize>)> =
                RangedHighlightIterator::new(&mut states.1, &ops, &str, &self.highlighter)
                    .map(|h| (h.0, h.2))
                    .collect();
            if i & 0xF == 0xF {
                self.states.push(states.clone());
            }

            self.highlighted_line.push(h);
        }
    }

    pub fn invalidate(&mut self, line: usize) {
        self.states.truncate(line >> 4);
        self.highlighted_line.truncate(line);
    }

    fn is_cached(&self, line: usize) -> bool {
        line < self.highlighted_line.len()
    }

    pub fn get_highlighted_line(
        &mut self,
        syntax: &SyntaxReference,
        buffer: &Buffer,
        line: usize,
    ) -> &[(Style, Range<usize>)] {
        if !self.is_cached(line) {
            self.update_range(syntax, buffer, line, line + 10);
        }
        &self.highlighted_line[line]
    }
}
