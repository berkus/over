//! Functions for loading/writing Objs.

pub mod error;

mod char_stream;
mod misc;
mod parser;
use self::error::ParseError;

use Obj;

type ParseResult<T> = Result<T, ParseError>;

const MAX_DEPTH: usize = 128;

/// Load an `Obj` from a file.
pub fn load_from_file(path: &str) -> ParseResult<Obj> {
    parser::parse_obj_file(path)
}

/// Load an `Obj` from a &str.
pub fn load_from_str(contents: &str) -> ParseResult<Obj> {
    parser::parse_obj_str(contents)
}

/// Write `obj` to a file at `path`.
pub fn write_to_file(_obj: &Obj, _path: &str) -> ParseResult<()> {
    // use std::fs::File;
    // let file = File::open(path).map_err(ParseError::from)?;

    unimplemented!()
}
