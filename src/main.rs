use crackme_homework_generator::{is_valid_wrapper, generate_number};

extern crate clap;
use clap::{Arg, App};
use std::str::FromStr;


fn main() {
    let matches = App::new("Crackme-homework serial number generator")
        .version("1.0")
        .arg(Arg::with_name("keys_number")
            .short("n")
            .value_name("NUMBER")
            .help("Sets a number of keys to generate for output")
            .required(true)
            .takes_value(true))
        .get_matches();

    match u8::from_str(matches.value_of("keys_number").unwrap()) {
        Ok(number) => {
            for _ in 0..number  {
                let generated_serial = generate_number();
                if is_valid_wrapper(&generated_serial) {
                    println!("{}", generated_serial);
                } else {
                    panic!("Generated key is not valid. Please, contact the programmer.")
                }
            }
        }
        Err(_) => {}
    }
}
