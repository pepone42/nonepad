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

const WORD_BOUNDARY_CHAR: [char; 6] = ['.', ':', ' ', '\n', '\r', '\t'];

pub fn next_word_boundary<U: Into<usize>>(slice: &RopeSlice, byte_idx: U) -> usize {
    let mut i: usize = byte_idx.into();
    if WORD_BOUNDARY_CHAR.contains(&slice.char(slice.byte_to_char(i))) {
        loop {
            if !WORD_BOUNDARY_CHAR.contains(&slice.char(slice.byte_to_char(i))) {
                break;
            }
            let newi = next_grapheme_boundary(slice, i);
            if newi == i {
                break;
            }
            i = newi;
        }
    }
    loop {
        if WORD_BOUNDARY_CHAR.contains(&slice.char(slice.byte_to_char(i))) {
            break;
        }
        let newi = next_grapheme_boundary(slice, i);
        if newi == i {
            break;
        }
        i = newi;
    }
    return i.into();
}

pub fn prev_word_boundary<U: Into<usize>>(slice: &RopeSlice, byte_idx: U) -> usize {
    let mut i: usize = byte_idx.into();
    if WORD_BOUNDARY_CHAR.contains(&slice.char(slice.byte_to_char(i))) {
        loop {
            if !WORD_BOUNDARY_CHAR.contains(&slice.char(slice.byte_to_char(i))) {
                break;
            }
            let newi = prev_grapheme_boundary(slice, i);
            if newi == i {
                break;
            }
            i = newi;
        }
    }
    loop {
        if WORD_BOUNDARY_CHAR.contains(&slice.char(slice.byte_to_char(i))) {
            break;
        }
        let newi = prev_grapheme_boundary(slice, i);
        if newi == i {
            break;
        }
        i = newi;
    }

    return i.into();
}
