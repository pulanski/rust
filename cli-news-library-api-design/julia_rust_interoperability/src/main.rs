use std::process::Command;

fn main() {
    println!("Hello from Rust");
    let mut cmd = Command::new("Julia");
    cmd.arg("src/main.jl");
    // cmd.args(&["main.jl", "arg1", "arg2"]);
    match cmd.output() {
        Ok(o) => unsafe {
            println!("Output: {}", String::from_utf8_unchecked(o.stdout));
        },
        Err(e) => {
            println!("There was an error {}", e);
        }
    }
}
