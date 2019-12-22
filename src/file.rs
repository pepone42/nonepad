use chardetng::EncodingDetector;
use encoding_rs::{Encoding, UTF_8};
use std::fs;
use std::io::{Read, Result};
use std::path::Path;

#[derive(Debug)]
pub struct TextFile {
    pub buffer: String,
    pub encoding: &'static Encoding,
    pub bom: Option<Vec<u8>>,
    pub linefeed: LineFeed,
}

impl Default for TextFile {
    fn default() -> Self {
        TextFile {
            buffer: String::new(),
            encoding: UTF_8,
            bom: None,
            linefeed: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LineFeed {
    CR,
    LF,
    CRLF,
}

impl Default for LineFeed {
    fn default() -> Self { 
        #[cfg(target_os = "windows")]
        return LineFeed::CRLF;
        #[cfg(not(target_os = "windows"))]
        return LineFeed::LF;
    }
    
}

impl LineFeed {
    pub fn to_str(&self) -> &'static str {
        match self {
            LineFeed::CR => "\r",
            LineFeed::LF => "\n",
            LineFeed::CRLF => "\r\n",
        }
    }
}

pub fn load<'a, P: AsRef<Path>>(path: P) -> Result<TextFile> {
    let mut file = fs::File::open(path)?;
    let mut detector = EncodingDetector::new();
    let mut vec = Vec::new();
    file.read_to_end(&mut vec)?;

    detector.feed(&vec, true);
    let encoding = Encoding::for_bom(&vec);

    match encoding {
        None => {
            let encoding = detector.guess(None, true);
            let buffer = encoding.decode_with_bom_removal(&vec).0.into_owned();
            let linefeed = detect_linefeed(&buffer);
            Ok(TextFile {
                buffer,
                encoding,
                bom: None,
                linefeed,
            })
        }
        Some((encoding, bom_size)) => {
            let bom = {
                let mut v = Vec::new();
                v.extend_from_slice(&vec[0..bom_size]);
                v
            };
            let buffer = encoding.decode_with_bom_removal(&vec).0.into_owned();
            let linefeed = detect_linefeed(&buffer);
            Ok(TextFile {
                buffer: encoding.decode_with_bom_removal(&vec).0.into_owned(),
                encoding,
                bom: Some(bom),
                linefeed,
            })
        }
    }
}

/// Detect the carriage return type of the buffer
pub fn detect_linefeed(input: &str) -> LineFeed {
    let linefeed = Default::default();

    if input.len() == 0 {
        return linefeed;
    }

    let mut cr = 0;
    let mut lf = 0;
    let mut crlf = 0;

    let mut chars = input.chars().take(1000);
    while let Some(c) = chars.next() {
        if c == '\r' {
            if let Some(c2) = chars.next() {
                if c2 == '\n' {
                    crlf += 1;
                } else {
                    cr += 1;
                }
            }
        } else if c == '\n' {
            lf += 1;
        }
    }

    let linefeed = if cr > crlf && cr > lf {
        LineFeed::CR
    } else if lf > crlf && lf > cr {
        LineFeed::LF
    } else {
        LineFeed::CRLF
    };
    return linefeed;
}
