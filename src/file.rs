use chardetng::EncodingDetector;
use encoding_rs::{Encoding, UTF_8};
use ropey::{Rope, RopeSlice};
use std::borrow::Cow;
use std::fs;
use std::io::{Read, Result, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct TextFileInfo {
    pub encoding: &'static Encoding,
    pub bom: Option<Vec<u8>>,
    pub linefeed: LineFeed,
    pub indentation: Indentation,
    pub path: Option<PathBuf>,
}

impl Default for TextFileInfo {
    fn default() -> Self {
        TextFileInfo {
            encoding: UTF_8,
            bom: None,
            linefeed: Default::default(),
            indentation: Default::default(),
            path: None,
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
#[derive(Debug, Clone, Copy)]
pub enum Indentation {
    Tab,
    Space(usize),
}
impl Default for Indentation {
    fn default() -> Self {
        // Todo, get from settings
        Indentation::Space(4)
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

impl TextFileInfo {
    pub fn load<'a, P: AsRef<Path>>(path: P) -> Result<(TextFileInfo, Rope)> {
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
                Ok((
                    TextFileInfo {
                        encoding,
                        bom: None,
                        linefeed,
                        indentation,
                        path: Some(path.as_ref().to_path_buf()),
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
                Ok((
                    TextFileInfo {
                        encoding,
                        bom: Some(bom),
                        linefeed,
                        indentation,
                        path: Some(path.as_ref().to_path_buf()),
                    },
                    buffer,
                ))
            }
        }
    }

    pub fn save_as<P: AsRef<Path>>(&mut self, buffer: &Rope, path: P) -> Result<()> {
        self.path = Some(path.as_ref().to_path_buf());
        self.save(buffer)?;
        Ok(())
    }

    pub fn save(&self, buffer: &Rope) -> Result<()> {
        println!("save to {:?}", &self.path);
        assert_ne!(self.path, None);
        let mut file = fs::File::create(self.path.as_ref().unwrap())?;
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
            file.write(bom)?;
        }
        file.write(&encoded_output)?;
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

    let linefeed = if cr > crlf && cr > lf {
        LineFeed::CR
    } else if lf > crlf && lf > cr {
        LineFeed::LF
    } else {
        LineFeed::CRLF
    };
    return linefeed;
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
        return Indentation::Tab;
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
        println!("largest {}", i.0);
        Indentation::Space(*i.0)
    } else {
        Indentation::default()
    }
}
