use chrono::Datelike;

use crate::meta::SOLUTIONS;

mod meta;
mod y2025;

fn main() {
    let mut args = std::env::args();
    let mut year = String::new();
    let mut day = String::new();
    let mut mode = meta::InputType::Example;

    // By default run today in example mode (if theres an active event)
    if args.len() == 1 {
        let now = chrono::Utc::now();
        if now.month() != 12 {
            panic!()
        }
        year = now.year().to_string();
        day = now.day().to_string();
    } else {
        let _ = args.next();
        year = args.next().unwrap();
        day = args.next().unwrap();
        mode = meta::InputType::from(args.next().unwrap().as_str());
    }

    if let Some(s) = SOLUTIONS.get(&year).and_then(|y| y.get(&day)) {
        let _ = (s.pretty_print_result)(&mode);
    }
}

fn pretty_print_all(input_type: &meta::InputType) {
    for (year, days) in crate::meta::SOLUTIONS.iter() {
        println!("");
        println!("***** {year} *****");
        println!("");
        for (_, sol) in days.iter() {
            let _ = (sol.pretty_print_result)(input_type);
        }
    }
}
