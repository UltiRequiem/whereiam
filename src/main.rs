use std::env;

fn main() {
    let dir = env::current_dir().unwrap();

    println!("{:?}", dir);
}
