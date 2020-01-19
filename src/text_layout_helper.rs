
use std::ops::Range;
use druid_shell::piet::TextLayout;

#[derive(Debug)]
enum InvisibleChar {
    Space(usize),
    Tab(usize, Range<usize>),
}

// pub fn index(index: usize, invisibles: &[InvisibleChar]) -> usize {
//     let mut j:usize = 0;
//     for i in 0..index {
//         if invisibles.filter(|a| match a { InvisibleChar::Tab(r) if r.contain(inde){ } })
//     }

// }