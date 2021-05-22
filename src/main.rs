use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

use env_logger;
use log::{debug, error, log_enabled, info, Level};
use regex::Regex;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename = &args[1];
    debug!("File to flatten: {}", filename);

    let re = Regex::new(r"^(\s+)(\S.+)$|^(\S.+)$").unwrap();
    let mut new_line = String::from("");
    let mut flat_config = String::from("BEGIN FLAT CONFIG\n");
    let mut current_spaces: usize = 0;
    let mut previous_spaces: usize = 0;
    let mut line_map: HashMap<usize, String> = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let string_line = String::from(line);
                let caps = re.captures(&string_line).unwrap();
                info!("Processing line: {}", &string_line);
                debug!("Caps: {:?}", &caps);
                let text1 = caps.get(1).map_or("", |m| m.as_str());
                let text2 = caps.get(2).map_or("", |m| m.as_str());
                let text3 = caps.get(3).map_or("", |m| m.as_str());
                let current_spaces: usize = *&text1.chars().count();
                debug!("Current spaces: {}", current_spaces);
                debug!("Previous spaces: {}", previous_spaces);
                if current_spaces == previous_spaces {
                    debug!("Condition 1 found!");
                    new_line = "".to_string();
                    for (spaces, string) in &line_map {
                        if spaces < &current_spaces {
                            new_line.push_str(&string);
                        }
                        new_line.push_str(&string_line)
                    }
                    line_map.insert(current_spaces, text3.to_string());
                    debug!("line_map updated to: {:?}", &line_map);
                }
                if current_spaces > previous_spaces {
                    debug!("Condition 2 found!");
                    new_line.push_str(" ");
                    new_line.push_str(&text2);
                    debug!("new_line updated to: {}", &new_line);
                    line_map.insert(current_spaces, String::from(text2));
                    debug!("line_map updated to: {:?}", &line_map);
                }
                if current_spaces < previous_spaces {
                    debug!("Condition 3 found!");
                    flat_config.push_str(&new_line);
                    flat_config.push_str("\n");
                    new_line = "".to_string();
                }
                previous_spaces = *&text1.chars().count();
                debug!("Previous spaces updated: {}", previous_spaces);
            }
        }
    }
    println!("{}", flat_config);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}