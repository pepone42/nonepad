use once_cell::sync::Lazy;
use syntect::parsing::SyntaxSet;

pub static SYNTAXSET: Lazy<SyntaxSet> = Lazy::new(|| {
    SyntaxSet::load_defaults_newlines()
});
 