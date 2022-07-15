use clap::{App, Arg};
use orthodb_request::generate_url;
use reqwest;
use serde_json;

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

    match record["data"]["KEGGpathway"].as_array() {
        Some(x) => {
            for k in x {
                if let Some(keggs) = k.as_object() {
                    for rec in keggs {
                        let (key, val) = rec;
                        println!("{},{}", key, val);
                    }
                }
            }
        }
        None => (),
    }
}
