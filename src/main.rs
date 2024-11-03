use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let content = &args[1];
    let file = &args[2];
    println!("content = {}, file = {}", content, file);
}
