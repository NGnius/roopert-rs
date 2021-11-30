use std::fmt::Display;

use syn::{Path, parse::ParseStream, Ident, Result};

pub fn single_path_segment<D: Display, E: Fn(&Path) -> D>(path: &Path, input: ParseStream, err_fn: E) -> Result<Ident> {
    if path.segments.len() != 1 {
        Err(input.error(err_fn(path)))
    } else {
        Ok(path.segments.first().unwrap().clone().ident)
    }
}
