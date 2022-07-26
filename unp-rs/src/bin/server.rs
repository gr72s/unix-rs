use std::{net::TcpListener, process};

use lib::util::str_echo;
use unix_rs::unistd::fork;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("listen..");

    loop {
        let (mut s, _) = listener.accept()?;
        let pid = fork().expect("fork error");
        if pid == 0 {
            str_echo(&mut s)?;
            process::exit(0);
        }
    }
}
