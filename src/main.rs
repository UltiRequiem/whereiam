use whereiam::where_i_am;

fn main() {
    let path = match where_i_am() {
        Ok(p) => p,
        Err(e) => panic!("{}", e),
    };

    println!("{}", path.as_str());
}
