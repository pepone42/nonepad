use std::fs;
use std::io::{Read, Result};
use std::path::Path;
use chardetng::EncodingDetector;
use encoding_rs::Encoding;
use std::borrow::Cow;


#[derive(Debug)]
pub struct TextFile {
    pub buffer: String,
    pub encoding: &'static Encoding,
    pub bom: Option<Vec<u8>>,
}

pub fn load<'a, P: AsRef<Path>>(path: P) -> Result<TextFile> {
    let mut file = fs::File::open(path)?;
    let mut detector = EncodingDetector::new();
    let mut vec = Vec::new();
    file.read_to_end(&mut vec)?;

    detector.feed(&vec,true);
    let encoding = Encoding::for_bom(&vec);
    match encoding {
        None => {
            let encoding = detector.guess(None,true);
            Ok(TextFile{buffer: encoding.decode_with_bom_removal(&vec).0.into_owned(),encoding,bom: None})
        }
            ,
        Some((encoding,bom_size)) => {
            let bom = {let mut v = Vec::new(); v.extend_from_slice(&vec[0..bom_size]);v};
            Ok(TextFile{buffer: encoding.decode_with_bom_removal(&vec).0.into_owned(),encoding,bom: Some(bom)})
        }
    }
}