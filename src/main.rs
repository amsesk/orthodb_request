use clap::{App, Arg};
use json;
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

    let ret: String = reqwest::get(&url).unwrap().text().unwrap();

    let parsed = json::parse(&ret).unwrap();

    //let ret_stringed = json::stringify(ret);

    let v: Vec<serde_json::Value> =
        serde_json::from_str(&parsed["data"]["interpro_domains"].dump()).unwrap();
    for ipracc in v.iter() {
        println!("{}", ipracc["id"]);
    }
    //let v: serde_json::Value = serde_json::from_str(.dump()).unwrap();

    //println!("{}", data["data"]);

    //println!("{:#}", parsed["data"]["interpro_domains"]);

    /*
    for ipacc in parsed["data"]["interpro_domains"].iter() {
        println!("{}", ipacc["id"])
    }
    */
}