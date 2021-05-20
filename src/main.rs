//! # 🐱 `pcat`
//!
//! `pcat` is an alternative version of [`cat`](https://linux.die.net/man/1/cat)
//! that will not fail (to read inputs) when results of computation to concatenate
//! are not available in the right order!
//!
//! ```shell
//! mkfifo A B
//! cat A B > output &
//! echo "bar" > B & # the requirement of `&` here should not be...
//! echo "foo" > A
//! ```
//!
//! So, glad to know that with `tac` you can remove it:
//!
//! ```shell
//! mkfifo A B
//! pcat A B > output &
//! echo "bar" > B # \o/
//! echo "foo" > A
//! ```
//!
//! Of course, `pcat` and `cat` give the same final result:
//!
//! ```raw
//! foo
//! bar
//! ```
//!
//! This could be particularly useful when you want to prototype parallel code
//! that's rely on concatenation at some point, using classic `cat` would create IO
//! lock which is not really funny... this project was made at origin for the
//! purpose of [YeAST](https://github.com/yvan-sraka/YeAST/) implementation!

use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let mut children = vec![];
    let args: Vec<String> = std::env::args().collect();
    for arg in &args[1..] {
        let path = arg.clone();
        children.push(std::thread::spawn(move || -> String {
            let file = std::fs::File::open(path.clone())
                .unwrap_or_else(|_| panic!("error: couldn't open file `{}`", path));
            let mut reader = std::io::BufReader::new(file);
            let mut buffer = String::new();
            loop {
                let len = reader
                    .read_line(&mut buffer)
                    .unwrap_or_else(|_| panic!("error: couldn't read file `{}`", path));
                if len == 0 {
                    return buffer;
                }
            }
        }));
    }
    for child in children {
        print!("{}", &child.join().unwrap());
    }
    Ok(())
}
