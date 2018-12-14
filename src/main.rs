use std::io::prelude::*;

fn main() {
    let mut children = vec![];
    let args: Vec<String> = std::env::args().collect();
    for arg in &args[1..] {
        let path = arg.clone();
        children.push(std::thread::spawn(move || -> String {
            let file = std::fs::File::open(path.clone())
                .unwrap_or_else(|_| panic!("Error: couldn't open file `{}`", path));
            let mut reader = std::io::BufReader::new(file);
            let mut buffer = String::new();
            loop {
                let len = reader
                    .read_line(&mut buffer)
                    .unwrap_or_else(|_| panic!("Error: couldn't read file `{}`", path));
                if len == 0 {
                    return buffer;
                }
            }
        }));
    }
    for child in children {
        print!("{}", &child.join().unwrap());
    }
}
