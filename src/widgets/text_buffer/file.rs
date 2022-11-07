use chardetng::EncodingDetector;
use encoding_rs::{Encoding, UTF_8};
use ropey::{Rope, RopeSlice};
use syntect::parsing::SyntaxReference;
use std::borrow::Cow;
use std::fs;
use std::io::{Read, Result, Write};
use std::{fmt::Display, path::Path};
use super::syntax::SYNTAXSET;

use super::buffer::Buffer;

#[derive(Debug, Clone)]
pub struct TextFileInfo {
    pub encoding: &'static Encoding,
    pub bom: Option<Vec<u8>>,
    pub linefeed: LineFeed,
    pub indentation: Indentation,
    pub syntax: &'static SyntaxReference
}

impl Default for TextFileInfo {
    fn default() -> Self {
        TextFileInfo {
            encoding: UTF_8,
            bom: None,
            linefeed: Default::default(),
            indentation: Default::default(),
            syntax: SYNTAXSET.find_syntax_plain_text(),
        }
    }
}

impl PartialEq for TextFileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding && self.bom == other.bom && self.linefeed == other.linefeed && self.indentation == other.indentation && self.syntax.name == other.syntax.name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineFeed {
    Cr,
    Lf,
    CrLf,
}

impl Display for LineFeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineFeed::Cr => write!(f, "CR"),
            LineFeed::Lf => write!(f, "LF"),
            LineFeed::CrLf => write!(f, "CRLF"),
        }
    }
}

impl Default for LineFeed {
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        return LineFeed::CrLf;
        #[cfg(not(target_os = "windows"))]
        return LineFeed::Lf;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Indentation {
    Tab(usize),
    Space(usize),
}

impl Display for Indentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Indentation::Tab(x) => write!(f, "Tab ({})", x),
            Indentation::Space(x) => write!(f, "Space ({})", x),
        }
    }
}

impl Indentation {
    pub fn visible_len(&self) -> usize {
        match *self {
            Indentation::Tab(l) => l,
            Indentation::Space(l) => l,
        }
    }

    pub fn len_as_byte(&self) -> usize {
        match *self {
            Indentation::Tab(_) => 1,
            Indentation::Space(l) => l,
        }
    }
}

impl Default for Indentation {
    fn default() -> Self {
        Indentation::Space(4)
    }
}

impl LineFeed {
    pub fn to_str(self) -> &'static str {
        match self {
            LineFeed::Cr => "\r",
            LineFeed::Lf => "\n",
            LineFeed::CrLf => "\r\n",
        }
    }
}

impl TextFileInfo {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<(TextFileInfo, Rope)> {
        let syntax = if let Ok(s ) = SYNTAXSET.find_syntax_for_file(&path) {
            s.unwrap_or_else(|| SYNTAXSET.find_syntax_plain_text())
        } else {
            SYNTAXSET.find_syntax_plain_text()
        };

        //let syntax = SYNTAXSET.find_syntax_by_extension(&std::path::Path::extension(path.as_ref()).unwrap_or(&OsString::from("")).to_string_lossy()).unwrap_or_else(|| SYNTAXSET.find_syntax_plain_text());
        let mut file = fs::File::open(&path)?;

        let mut detector = EncodingDetector::new();
        let mut vec = Vec::new();
        file.read_to_end(&mut vec)?;

        

        detector.feed(&vec, true);
        let encoding = Encoding::for_bom(&vec);
        
        

        match encoding {
            None => {
                let encoding = detector.guess(None, true);
                
                let buffer = Rope::from_str(&encoding.decode_with_bom_removal(&vec).0);
                let linefeed = detect_linefeed(&buffer.slice(..));
                let indentation = detect_indentation(&buffer.slice(..));

                //crate::syntax::stats(buffer.to_string(), syntax);

                Ok((
                    TextFileInfo {
                        encoding,
                        bom: None,
                        linefeed,
                        indentation,
                        syntax
                    },
                    buffer,
                ))
            }
            Some((encoding, bom_size)) => {
                let bom = {
                    let mut v = Vec::new();
                    v.extend_from_slice(&vec[0..bom_size]);
                    v
                };
                let buffer = Rope::from_str(&encoding.decode_with_bom_removal(&vec).0);
                let linefeed = detect_linefeed(&buffer.slice(..));
                let indentation = detect_indentation(&buffer.slice(..));

                //crate::syntax::stats(buffer.to_string(), syntax);

                Ok((
                    TextFileInfo {
                        encoding,
                        bom: Some(bom),
                        linefeed,
                        indentation,
                        syntax
                    },
                    buffer,
                ))
            }
        }
    }

    pub fn save_as<P: AsRef<Path>>(&mut self, buffer: &Buffer, path: P) -> Result<()> {
        let mut file = fs::File::create(path.as_ref())?;
        let input = buffer.to_string();
        let encoded_output = match self.encoding.name() {
            "UTF-16LE" => {
                let mut v = Vec::new();
                input.encode_utf16().for_each(|i| v.extend_from_slice(&i.to_le_bytes()));
                Cow::from(v)
            }
            "UTF-16BE" => {
                let mut v = Vec::new();
                input.encode_utf16().for_each(|i| v.extend_from_slice(&i.to_be_bytes()));
                Cow::from(v)
            }
            _ => self.encoding.encode(&input).0,
        };

        if let Some(bom) = &self.bom {
            file.write_all(bom)?;
        }
        file.write_all(&encoded_output)?;
        Ok(())
    }
}

/// Detect the carriage return type of the buffer
fn detect_linefeed(input: &RopeSlice) -> LineFeed {
    let linefeed = Default::default();

    if input.len_bytes() == 0 {
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

    if cr > crlf && cr > lf {
        LineFeed::Cr
    } else if lf > crlf && lf > cr {
        LineFeed::Lf
    } else {
        LineFeed::CrLf
    }
}

pub fn detect_indentation(input: &RopeSlice) -> Indentation {
    // detect Tabs first. If the first char of a line is more often a Tab
    // then we consider the indentation as tabulation.

    let mut tab = 0;
    let mut space = 0;
    for line in input.lines() {
        match line.chars().next() {
            Some(' ') => space += 1,
            Some('\t') => tab += 1,
            _ => (),
        }
    }
    if tab > space {
        // todo: get len from settings
        return Indentation::Tab(4);
    }

    // Algorythm from
    // https://medium.com/firefox-developer-tools/detecting-code-indentation-eff3ed0fb56b
    use std::collections::HashMap;
    let mut indents = HashMap::new();
    let mut last = 0;

    for line in input.lines() {
        let width = line.chars().take_while(|c| *c == ' ').count();
        let indent = if width < last { last - width } else { width - last };
        if indent > 1 {
            (*indents.entry(indent).or_insert(0)) += 1;
        }
        last = width;
    }
    if let Some(i) = indents.iter().max_by(|x, y| x.1.cmp(y.1)) {
        Indentation::Space(*i.0)
    } else {
        Indentation::default()
    }
}
