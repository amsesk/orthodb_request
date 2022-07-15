use clap::{App, Arg};
use orthodb_request::{generate_url, get_data};
use reqwest;
use serde_json;
use std::collections::HashMap;

fn main() -> () {
    let args = App::new("orthodb_request")
        .version("0.1")
        .author("Kevin Amses")
        .about("Send requests to the OrthoDB API.")
        .arg(
            Arg::new("command")
                .short('c')
                .long("cmd")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("term")
                .short('t')
                .long("term")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("value")
                .short('v')
                .long("value")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let cmd = args.value_of("command").unwrap();
    let term = args.value_of("term").unwrap();
    let value = args.value_of("value").unwrap();

    let url = generate_url(&cmd, &term, &value);

    let record: serde_json::Value =
        serde_json::from_str(&reqwest::get(&url).unwrap().text().unwrap()).unwrap();

    for hm in get_data(&record, "KEGGpathway").unwrap() {
        for (k,v) in hm {
            println!("{} -- {}", k, v);
        }
        println!("-----------------");
    }
    for hm in get_data(&record, "interpro_domains").unwrap() {
        for (k,v) in hm {
            println!("{} -- {}", k, v);
        }
        println!("-----------------");
    }
}
