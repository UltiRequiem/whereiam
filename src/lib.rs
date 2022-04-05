use std::{env, io::Result};

/// Returns in string format the current directory.
pub fn where_i_am() -> Result<String> {
    let value = format!("{:?}", env::current_dir()?);

    Ok(value)
}
