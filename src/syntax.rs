use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    ops::{Bound, Range, RangeBounds},
    str::FromStr,
};
use syntect::{
    easy::{self, ScopeRegionIterator},
    highlighting::{HighlightIterator, HighlightState, Highlighter, RangedHighlightIterator, Style, Theme, ThemeSet},
    parsing::{ParseState, Scope, ScopeStack, ScopeStackOp, SyntaxReference, SyntaxSet, SCOPE_REPO},
    util,
};

use crate::{
    text_buffer::buffer::Buffer,
    text_buffer::{position::Line, EditStack},
    theme::{self, THEME},
};

pub static SYNTAXSET: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());

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
        dbg!(start<<4, &states.1.path);

        for i in start << 4..(end << 4).min(buffer.len_lines()) {
            if i & 0xF == 0 {
                self.states.push(states.clone());
            }
            dbg!(i, &states.1.path);
            let str = buffer.line(i).to_string(&buffer);
            let ops = states.0.parse_line(&str, &SYNTAXSET);
            let h: Vec<(Style, Range<usize>)> =
                RangedHighlightIterator::new(&mut states.1, &ops, &str, &self.highlighter)
                    .map(|h| (h.0, h.2))
                    .collect();
            

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
        if self.is_cached(line) {
            &self.highlighted_line[line]
        } else {
            self.update_range(syntax, buffer, line, line + 10);
            &self.highlighted_line[line]
        }
    }
}

pub fn stats(input: String, syntax: &SyntaxReference) {
    let mut scope_stack = ScopeStack::new();
    let mut state = ParseState::new(syntax);
    //let highlighter = Highlighter::new(&THEME.style);

    //let theme = &ThemeSet::load_defaults().themes["base16-ocean.dark"];
    let theme = &THEME.style;
    //dbg!(serde_json::to_string(&THEME.style));
    let highlighter = Highlighter::new(theme);
    let mut highlight_state = HighlightState::new(&highlighter, scope_stack);
    for line in input.lines() {
        let ops = state.parse_line(&line, &SYNTAXSET);

        // for s in  RangedHighlightIterator::new(&mut highlight_state,&ops[..],line,&highlighter) {
        //     //dbg!(s);

        // }

        println!(
            "{}",
            util::as_24_bit_terminal_escaped(
                &HighlightIterator::new(&mut highlight_state, &ops[..], line, &highlighter)
                    .collect::<Vec<(syntect::highlighting::Style, &str)>>(),
                false
            )
        );

        // let ops = state.parse_line(&line, &SYNTAXSET);
        // for (s, op) in ScopeRegionIterator::new(&ops, line) {
        //     scope_stack.apply(op);

        //     if !scope_stack.is_empty() && scope_stack.does_match(ScopeStack::from_str("punctuation.definition.comment").unwrap().as_slice()).is_some() {
        //         dbg!(s);
        //         scope_stack.debug_print(&SCOPE_REPO.lock().unwrap());
        //     }
        // }
    }
}
