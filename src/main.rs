//TO DO 
//Option for preserving block ids

use std::fs;
use flate2;

use mc_classic;

mod convert;

pub fn main () {

    let mut buf: String = "".to_string();
    let mut words: Vec<String> = Vec::new();

    while &buf == "" {
        println!("Press [w] to write a 4k file, press [r] to read a 4k file");
        std::io::stdin().read_line(&mut buf).expect("And this somehow broke...");

        match buf.trim() {
            "w" => words.push("write".to_string()),
            "r" => words.push("read".to_string()),
            _ => {
                println!("No you silly, you wrote the wrong thing");
                buf = "".to_string();
                continue
            }
        }

        if &buf.as_str() != &"w" || &buf.as_str() != &"r" {
            
        }

        let mut path: String = "".to_string();
        println!("Please paste the path to your file");
        std::io::stdin().read_line(&mut path).expect("And this somehow broke...");
        path = path.trim().to_string();

        let mut version: u8 = 3;
        while version > 2 {
            buf = "".to_string();
            println!("And which 4k version would you like to {} to",words[0]);
            println!("[4k-011742] Type 0\n[4k-040144] Type 1\n[4k-javascript] Type 2");
            std::io::stdin().read_line(&mut buf).expect("And this somehow broke...");
            version = buf.trim().parse().unwrap();

            if version > 2 {println!("You absolute brainless idiot, all I asked you to do was to type one god forsaken number. You literally only had to count to 2 you moron. An infant would perform better than you. Go die in a corner. Enjoy suffering in Hell.")}
        }

        buf = "".to_string();
        let mut con: bool = true;
        while buf == "" {
            println!("\nWould you like to convert ids or keep raw ids?\n[c] for convert\n[k] for keep");
            println!("If you don't know what this means, press [c]");
            std::io::stdin().read_line(&mut buf).expect("And this somehow broke...");

            match  buf.trim() {
                "c" => con = true,
                "k" => con = false,
                _ => {
                    println!("Just c or k my friend.");
                    buf = "".to_string();
                    continue
                }
            }
        } 

        match words[0].as_str() {

            "write" => convert::write_4k(&path, version, con),
            "read" => {
                let mut brck: bool = false;
                if con {
                    buf = "".to_string();
                    while buf == "" {

                        println!("\nWould you like to preserve bricks? [yes] or [no]?");
                        std::io::stdin().read_line(&mut buf).expect("And this somehow broke...");

                        match buf.trim() {
                            "yes" => brck = true,
                            "no" => brck = false,
                            _ => {
                                println!("Bro. My Guy. Just write [yes] or [no]. Not. That. Hard.");
                                buf = "".to_string();
                            }
                        }
                    }
                }
                convert::read_4k(&path, version, con, brck).expect("And, something broke somewhere...")
            },
            _ => ()
        }
    }

    println!("Conversion is complete!");

    println!("Press Enter to Exit");
    let mut s: String = String::from("");
    std::io::stdin().read_line(&mut s).expect("");
    return;

}
