use std::{
    env,
    io::{self, Read},
};

fn main() {
    if isatty() {
        let args: Vec<String> = env::args().collect();
        for arg in args[1..].iter() {
            println!("{}", arg);
        }
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap();
        println!("{}", buf);
    }
}

fn isatty() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
}
