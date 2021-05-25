use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

use env_logger;
use indexmap::IndexMap;
use log::{debug, error, log_enabled, info, Level};
use regex::Regex;

enum LineMap {
    LineNumber(usize),
    LineString(String)
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename = &args[1];
    let indent = &args[2].parse::<usize>().unwrap();
    let ignore = &args[3];

    debug!("File to flatten: {}", filename);

    let re = Regex::new(r"^(\s+)(\S.+)$|^(\S.+)$").unwrap();
    let ignore_re = Regex::new(ignore).unwrap();
    let mut flat_config = String::from("");
    let mut previous_spaces: usize = 0;
    let mut line_map: IndexMap<usize, String> = IndexMap::new();

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let string_line = String::from(line);
                match re.captures(&string_line) {
                    None => {
                    }
                    Some(caps) => {
                        info!("Processing line: {}", &string_line);
                        debug!("Caps: {:?}", &caps);
                        let text1 = caps.get(1).map_or("", |m| m.as_str());
                        let text2 = caps.get(2).map_or("", |m| m.as_str());
                        let text3 = caps.get(3).map_or("", |m| m.as_str());
                        let current_spaces: usize = *&text1.chars().count();
                        if current_spaces < *indent {
                            continue;
                        }
                        match ignore_re.captures(&string_line){
                            None => {
                            }
                            Some(ignore_caps) => {
                                debug!("Ignoring line due to regex match: {}", &string_line);
                                continue;
                            }
                        }
                        debug!("Current spaces: {}", current_spaces);
                        debug!("Previous spaces: {}", previous_spaces);
                        if current_spaces <= previous_spaces {
                            debug!("current_spaces <= previous_spaces");
                            let mut new_line = "".to_string();
                            for (_spaces, string) in &line_map {
                                &new_line.push_str(&string);
                                &new_line.push_str(" ");
                            }
                            debug!("new_line: {}", &new_line);
                            if &new_line != "" {
                                flat_config.push_str(&new_line);
                                flat_config.push_str("\n");
                            }
                            if *&text2.chars().count() > 0 {
                                &line_map.insert(current_spaces, text2.to_string());
                            } else {
                                &line_map.insert(current_spaces, text3.to_string());
                            }
                            let mut to_remove = vec![];
                            for (spaces, _string) in &line_map {
                                if spaces > &current_spaces {
                                    to_remove.push(spaces.clone());
                                }
                            }
                            for spaces_to_remove in to_remove {
                                debug!("spaces_to_remove: {:?}", spaces_to_remove);
                                &line_map.remove(&spaces_to_remove);
                            }
                            debug!("line_map updated to: {:?}", &line_map);
                        }
                        if current_spaces > previous_spaces {
                            debug!("current_spaces > previous_spaces");
                            // Continue growing the line map.
                            &line_map.insert(current_spaces, String::from(text2));
                            debug!("line_map updated to: {:?}", &line_map);
                        }
                        previous_spaces = *&text1.chars().count();
                    }
                }
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