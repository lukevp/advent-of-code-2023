// use std::env;
// use std::fs;

// TODO: why can't I reference read_lines for this?


mod read_lines {
    use std::{
        fs::File,
        io::{self, prelude::*},
        rc::Rc,
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
        buf: Rc<String>,
    }
    
    fn new_buf() -> Rc<String> {
        Rc::new(String::with_capacity(1024)) // Tweakable capacity
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);
            let buf = new_buf();

            Ok(Self { reader, buf })
        }
    }

    impl Iterator for BufReader {
        type Item = io::Result<Rc<String>>;

        fn next(&mut self) -> Option<Self::Item> {
            let buf = match Rc::get_mut(&mut self.buf) {
                Some(buf) => {
                    buf.clear();
                    buf
                }
                None => {
                    self.buf = new_buf();
                    Rc::make_mut(&mut self.buf)
                }
            };

            self.reader
                .read_line(buf)
                .map(|u| if u == 0 { None } else { Some(Rc::clone(&self.buf)) })
                .transpose()
        }
    }
}



// #[path = "../read_lines.rs"]
// mod read_lines;

use read_lines::BufReader;

fn main() -> std::io::Result<()>  {
    // let file_path = "input.txt";
    // println!("In file {}", file_path);

    // let contents = fs::read_to_string(file_path)
    //     .expect("Should have been able to read the file");

    // for
    // println!("With text:\n{contents}");
    let mut sum = 0;
    for line in read_lines::BufReader::open("input.txt")? {
        let mut first_digit:char = '\0';
        let mut last_digit = '\0';
        for char in line?.chars() {
            //print!("{}", char);
            if (char.is_digit(10)) {
                if (first_digit == '\0') {
                    first_digit = char;
                }
                last_digit = char;
            }
        }
        let last_sum = sum;
        
        sum += first_digit.to_digit(10).unwrap() * 10;
        sum += last_digit.to_digit(10).unwrap();

        print!("{} + {}{} = {}\n", last_sum, first_digit, last_digit, sum);
        // Convert first_digit and last_digit to a single integer
        
        //println!("{}", line?.trim());
    }

    print!("Final sum: {}\n", sum);

    Ok(())
}



