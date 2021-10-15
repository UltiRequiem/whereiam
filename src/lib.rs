use std::env;

pub fn where_i_am() -> String {
    return format!("{:?}", env::current_dir().unwrap()); 
}
