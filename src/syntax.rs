use once_cell::sync::Lazy;
use syntect::{easy::ScopeRegionIterator, highlighting::{HighlightState, Highlighter, RangedHighlightIterator, Theme, ThemeSet}, parsing::{ParseState, SCOPE_REPO, Scope, ScopeStack, ScopeStackOp, SyntaxReference, SyntaxSet}};
use std::str::FromStr;

use crate::theme::{self, THEME};

pub static SYNTAXSET: Lazy<SyntaxSet> = Lazy::new(|| {
    SyntaxSet::load_defaults_newlines()
});
 
pub fn stats(input: String,syntax: &SyntaxReference) {
    let mut scope_stack = ScopeStack::new();
    let mut state = ParseState::new(syntax);
    //let highlighter = Highlighter::new(&THEME.style);

    let theme = &ThemeSet::load_defaults().themes["base16-ocean.dark"];
    //let theme = &THEME.style;
    dbg!(serde_json::to_string(&THEME.style));
    let highlighter = Highlighter::new(theme);
    let mut highlight_state = HighlightState::new(&highlighter, scope_stack);
    for line in input.lines() {
        let ops = state.parse_line(&line, &SYNTAXSET);

        for s in  RangedHighlightIterator::new(&mut highlight_state,&ops[..],line,&highlighter) {
            dbg!(s);
        }

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