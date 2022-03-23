use once_cell::sync::Lazy;
use ropey::Rope;
use std::{
    ops::{Deref, Range},
    sync::{Arc, Mutex},
};
use syntect::{
    highlighting::{HighlightState, Highlighter, RangedHighlightIterator, Style},
    parsing::{ParseState, ScopeStack, SyntaxReference, SyntaxSet},
};

use crate::theme::THEME;

pub static SYNTAXSET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);

#[derive(Debug)]
pub struct StateCache {
    states: Vec<(ParseState, HighlightState)>,
    highlighter: Highlighter<'static>,
}
#[derive(Debug)]
pub struct SpanStyle {
    pub style: Style,
    pub range: Range<usize>,
}

impl SpanStyle {
    pub fn new(style: Style, range: Range<usize>) -> Self {
        Self { style, range }
    }
}

#[derive(Debug)]
pub struct StyledLine {
    styles: Vec<SpanStyle>,
}

impl StyledLine {
    pub fn new(styles: Vec<SpanStyle>) -> Self {
        Self { styles }
    }
}

impl Deref for StyledLine {
    type Target = Vec<SpanStyle>;

    fn deref(&self) -> &Self::Target {
        &self.styles
    }
}

#[derive(Debug, Clone)]
pub struct StyledLinesCache {
    pub lines: Arc<Mutex<Vec<StyledLine>>>
}

impl StyledLinesCache {
    pub fn new() -> Self { Self { lines: Arc::new(Mutex::new(Vec::new())) } }
}


impl StateCache {
    pub fn new() -> Self {
        StateCache {
            states: Vec::new(),
            highlighter: Highlighter::new(&THEME.style),
        }
    }

    pub fn update_range(
        &mut self,
        highlighted_line: StyledLinesCache,
        syntax: &SyntaxReference,
        rope: &Rope,
        start: usize,
        end: usize,
    ) {
        // states are cached every 16 lines
        let start = (start >> 4).min(self.states.len());
        let end = (end.min(rope.len_lines()) >> 4) + 1;
        
        self.states.truncate(start);
        
        let mut states = self.states.last().cloned().unwrap_or_else(|| {
            (
                ParseState::new(syntax),
                HighlightState::new(&self.highlighter, ScopeStack::new()),
            )
        });

        for i in start << 4..(end << 4).min(rope.len_lines()) {
            let h = if let Some(str) = rope.line(i).as_str() {
                let ops = states.0.parse_line(str, &SYNTAXSET);
                let h: Vec<_> = RangedHighlightIterator::new(&mut states.1, &ops, str, &self.highlighter)
                    .map(|h| SpanStyle::new(h.0, h.2))
                    .collect();
                StyledLine::new(h)
            } else {
                let str = rope.line(i).to_string();
                let ops = states.0.parse_line(&str, &SYNTAXSET);
                let h: Vec<_> = RangedHighlightIterator::new(&mut states.1, &ops, &str, &self.highlighter)
                    .map(|h| SpanStyle::new(h.0, h.2))
                    .collect();
                StyledLine::new(h)
            };
            if i & 0xF == 0xF {
                self.states.push(states.clone());
            }
            let mut hl = highlighted_line.lines.lock().unwrap();
            if i>= hl.len() {
                hl.push(h);
            } else {
                hl[i] = h;
            }
        }
    }
}
