use regex::Regex;
use clap::{Arg, App};

fn main() {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Arthur Melton")
        .about("use regex in the terminal")
        .arg(Arg::with_name("find")
            .short("f")
            .long("find")
            .value_name("regex")
            .help("search with regex")
            .takes_value(true))
        .arg(Arg::with_name("replace")
            .short("r")
            .long("replace")
            .value_name("regex")
            .help("replace with regex")
            .takes_value(true))
        .arg(Arg::with_name("replace_2")
            .help("what will be replaced")
            .required(false)
            .index(1))
        .get_matches();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_string();
    if matches.is_present("find") {
        let re = Regex::new(matches.value_of("find").unwrap()).unwrap();
        println!("{:?}", re.find_iter(&line).filter_map(|digits| digits.as_str().parse().ok()).collect::<Vec<String>>());
    }
    if matches.is_present("replace") {
        let re = Regex::new(matches.value_of("replace").unwrap()).unwrap();
        println!("{}", re.replace_all(&line, matches.value_of("replace_2").unwrap()));
    }
}
