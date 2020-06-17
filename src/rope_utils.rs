use ropey::RopeSlice;
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};

/// Finds the previous grapheme boundary before the given char position.
pub fn prev_grapheme_boundary<U: Into<usize>>(slice: &RopeSlice, byte_idx: U) -> usize {
    let byte_idx = byte_idx.into();
    // Bounds check
    debug_assert!(byte_idx <= slice.len_bytes());

    // Get the chunk with our byte index in it.
    let (mut chunk, mut chunk_byte_idx, _, _) = slice.chunk_at_byte(byte_idx);

    // Set up the grapheme cursor.
    let mut gc = GraphemeCursor::new(byte_idx, slice.len_bytes(), true);

    // Find the previous grapheme cluster boundary.
    loop {
        match gc.prev_boundary(chunk, chunk_byte_idx) {
            Ok(None) => return 0,
            Ok(Some(n)) => {
                let tmp = n - chunk_byte_idx;
                return chunk_byte_idx + tmp;
            }
            Err(GraphemeIncomplete::PrevChunk) => {
                let (a, b, _, _) = slice.chunk_at_byte(chunk_byte_idx - 1);
                chunk = a;
                chunk_byte_idx = b;
            }
            Err(GraphemeIncomplete::PreContext(n)) => {
                let ctx_chunk = slice.chunk_at_byte(n - 1).0;
                gc.provide_context(ctx_chunk, n - ctx_chunk.len());
            }
            _ => unreachable!(),
        }
    }
}

/// Finds the next grapheme boundary after the given char position.
pub fn next_grapheme_boundary<U: Into<usize>>(slice: &RopeSlice, byte_idx: U) -> usize {
    let byte_idx = byte_idx.into();
    // Bounds check
    debug_assert!(byte_idx <= slice.len_bytes());

    // Get the chunk with our byte index in it.
    let (mut chunk, mut chunk_byte_idx, _, _) = slice.chunk_at_byte(byte_idx);

    // Set up the grapheme cursor.
    let mut gc = GraphemeCursor::new(byte_idx, slice.len_bytes(), true);

    // Find the next grapheme cluster boundary.
    loop {
        match gc.next_boundary(chunk, chunk_byte_idx) {
            Ok(None) => return slice.len_bytes(),
            Ok(Some(n)) => {
                let tmp = n - chunk_byte_idx;
                return chunk_byte_idx + tmp;
            }
            Err(GraphemeIncomplete::NextChunk) => {
                chunk_byte_idx += chunk.len();
                let (a, b, _, _) = slice.chunk_at_byte(chunk_byte_idx);
                chunk = a;
                chunk_byte_idx = b;
            }
            Err(GraphemeIncomplete::PreContext(n)) => {
                let ctx_chunk = slice.chunk_at_byte(n - 1).0;
                gc.provide_context(ctx_chunk, n - ctx_chunk.len());
            }
            _ => unreachable!(),
        }
    }
}

const WORD_BOUNDARY_PUCTUATION: [char; 31] = [
    '`', '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '=', '+', '[', '{', ']', '}', '\\', '|', ';', ':',
    '\'', '"', ',', '.', '<', '>', '/', '?',
];
const WORD_BOUNDARY_LINEFEED: [char; 2] = ['\n', '\r'];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharType {
    LINEFEED,
    SPACE,
    PUCTUATION,
    OTHER,
}

fn char_type(c: char) -> CharType {
    if WORD_BOUNDARY_PUCTUATION.contains(&c) {
        return CharType::PUCTUATION;
    }
    if WORD_BOUNDARY_LINEFEED.contains(&c) {
        return CharType::LINEFEED;
    }
    if c.is_whitespace() {
        return CharType::SPACE;
    }
    return CharType::OTHER;
}

fn is_boundary(a: char, b: char) -> bool {
    char_type(a) != char_type(b)
}

pub fn next_word_boundary<U: Into<usize>>(slice: &RopeSlice, byte_idx: U) -> usize {
    let mut i: usize = slice.byte_to_char(byte_idx.into());

    // discard all space
    i += slice.chars_at(i).take_while(|c| c.is_whitespace()).count();

    // if multi puctionation, skip to new non puctuation char
    let fp = slice
        .chars_at(i)
        .take_while(|c| WORD_BOUNDARY_PUCTUATION.contains(c))
        .count();
    i += fp;
    if i >= slice.len_chars() {
        return slice.len_bytes();
    }
    let current_char = slice.char(i);
    if fp > 1 || (fp == 1 && char_type(current_char) != CharType::OTHER) {
        return slice.char_to_byte(i);
    }

    i += slice.chars_at(i).take_while(|c| !is_boundary(*c, current_char)).count();

    return slice.char_to_byte(i);
}

pub fn prev_word_boundary<U: Into<usize>>(slice: &RopeSlice, byte_idx: U) -> usize {
    let mut i: usize = slice.byte_to_char(byte_idx.into());

    // discard all space
    let mut iter = slice.chars_at(i);
    let mut count = 0;
    i -= loop {
        match iter.prev() {
            Some(c) if c.is_whitespace() => count += 1,
            _ => break count,
        }
    };

    // if multi puctionation, skip to new non puctuation char
    let mut iter = slice.chars_at(i);
    let mut count = 0;
    let fp = loop {
        match iter.prev() {
            Some(c) if WORD_BOUNDARY_PUCTUATION.contains(&c) => count += 1,
            _ => break count,
        }
    };
    i -= fp;
    if i == 0 {
        return 0;
    }

    let current_char = slice.char(i - 1);
    if fp > 1 || (fp == 1 && char_type(current_char) != CharType::OTHER) {
        return slice.char_to_byte(i);
    }

    let mut iter = slice.chars_at(i);
    let mut count = 0;
    i -= loop {
        match iter.prev() {
            Some(c) if !is_boundary(c, current_char) => count += 1,
            _ => break count,
        }
    };

    return slice.char_to_byte(i);
}
