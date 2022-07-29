use clap::{App, Arg};
use csv::WriterBuilder;
use orthodb_request::GenerateUrl;
use orthodb_request::{get_data, GoUrl, OrthoDbUrl, Url};
use reqwest;
use serde_json;
use std::fs::OpenOptions;
use std::path::Path;

fn main() -> () {
    let args = App::new("orthodb_request")
        .version("0.1")
        .author("Kevin Amses")
        .about("Send requests to the OrthoDB API.")
        .arg(
            Arg::new("api")
                .short('v')
                .long("value")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("command")
                .short('c')
                .long("cmd")
                .takes_value(true)
                .required(false),
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
                .required(false),
        )
        .get_matches();

    let api = args.value_of("api").unwrap();
    let cmd = args.value_of("command").unwrap();
    let term = args.value_of("term").unwrap();
    let value = args.value_of("value").unwrap();

    let url: Box<dyn GenerateUrl>;
    if api == "go" {
        url = Box::new(GoUrl::new(String::from(term)));
    } else if api == "odb" {
        url = Box::new(OrthoDbUrl::new(
            String::from(cmd),
            String::from(term),
            String::from(value),
        ));
    } else {
        panic!()
    }

    let record: serde_json::Value =
        serde_json::from_str(&reqwest::get(&url.generate()).unwrap().text().unwrap()).unwrap();

    let outpath = format!("{}_orthodb_results.tsv", value);
    let handle = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(&outpath))
        .unwrap();
    let mut wtr = WriterBuilder::new().delimiter(b'\t').from_writer(handle);

    for hm in get_data(&record, "name").unwrap() {
        wtr.write_record(&[
            &format!("{}", &value),
            "OrthoDB",
            &format!("{}", &value),
            &format!("{}", hm["name"].as_str().unwrap()),
        ])
        .unwrap()
    }

    if let Some(res) = get_data(&record, "KEGGpathway") {
        for hm in res {
            wtr.write_record(&[
                &format!("{}", &value),
                "KEGGpathway",
                &format!("{}", hm["id"].as_str().unwrap()),
                &format!("{}", hm["description"].as_str().unwrap()),
            ])
            .unwrap();
        }
    }
    if let Some(res) = get_data(&record, "interpro_domains") {
        for hm in res {
            wtr.write_record(&[
                &format!("{}", &value),
                "interpro_domains",
                &format!("{}", hm["id"].as_str().unwrap()),
                &format!("{}", hm["description"].as_str().unwrap()),
            ])
            .unwrap();
        }
    }
    if let Some(res) = get_data(&record, "functional_category") {
        for hm in res {
            wtr.write_record(&[
                &format!("{}", &value),
                "functional_category",
                &format!("{}", hm["id"].as_str().unwrap()),
                &format!("{}", hm["description"].as_str().unwrap()),
            ])
            .unwrap();
        }
    }
}
