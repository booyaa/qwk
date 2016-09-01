extern crate quackngo;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;
use clap::{Arg, App}; //, SubCommand};

//     // &t=user-agent
//
//     // -type A/C/N/E - return 1 if nothing
//     // -count N - return how many records, default is all
//     // Abstracts and categories
//     // - -rt = show related records instead (default for categories)
//     // Abstracts only
//     // - return abstract_text (default)
//     // Exclusive only
//     // - return redirect

#[allow(unused_imports)]
use quackngo::Quack;

const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const APP_ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    env_logger::init().unwrap();

    let matches = App::new(APP_NAME)
                      .version(APP_VERSION)
                      .author(APP_AUTHOR)
                      .about(APP_ABOUT)
                      .arg(Arg::with_name("query")
                               .help("Something to search")
                               .required(true)
                               .index(1))
                      .arg(Arg::with_name("type")
                               .help("Type of record to return, will error if no results")
                               .short("t")
                               .long("type")
                               .value_name("A|C|N|E")
                               .possible_values(&["A", "C", "N", "E"])
                               .takes_value(true))
                      .arg(Arg::with_name("count")
                               .help("Number of records to return")
                               .short("c")
                               .long("count")
                               .value_name("RECORDS")
                               .takes_value(true))
                      .get_matches();

    let query = matches.value_of("query").unwrap();
    info!("query: {}", query);

    let results = Quack::new(&query);
    debug!("{:#?}", results);
    if results.response_type == "E" {
        println!("{}", results.redirect);
    } else if !results.abstract_text.is_empty() {

        println!("{}", results.abstract_text);
    } else {
        println!("Nothing found :(");
    }

    if matches.is_present("count") {
        let count = matches.value_of("count").unwrap();
        info!("count: {}", count);

    }

    if matches.is_present("type") {
        let answer_type = matches.value_of("type").unwrap();
        info!("type: {}", answer_type);
    }


    // println!("{:#?}", foo);
}
