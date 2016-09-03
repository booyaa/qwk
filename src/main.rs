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
                      .arg(Arg::with_name("count")
                               .help("Number of records to return")
                               .short("c")
                               .long("count")
                               .value_name("RECORDS")
                               .takes_value(true))
                      .get_matches();

    let query = matches.value_of("query").unwrap();
    debug!("query: {}", query);

    let results = Quack::new(&query).expect("Nothing found :(");
    debug!("{:#?}", results);

    if results.response_type == "E" {
        println!("{}", results.redirect);
    } else if results.response_type == "C" {
        println!("{}",
                 results.related_topics
                        .iter()
                        .nth(0)
                        .expect("failed to find RelatedTopic tag")
                        .text);
    } else if results.response_type == "A" {
        println!("{}", results.abstract_text);
    } else {
        println!("response_type: {}", results.response_type);
    }

    if matches.is_present("count") {
        let count = matches.value_of("count").unwrap().parse::<usize>().unwrap();
        let actual_count = results.related_topics.len();
        if count > actual_count {
            error!("There are only {} records", actual_count);
            std::process::exit(1_i32);
        }
        for i in 1..count {
            println!("{},{:#?}",
                     i,
                     results.related_topics
                            .iter()
                            .nth(i)
                            .expect("failed to find RelatedTopic tag")
                            .text);
            info!("count: {}", count);
        }
    }
}
