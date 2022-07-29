use clap::{App, Arg};
use orthodb_request::{args_to_url, get_data, GoOntologyJson, Url};
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
                .short('a')
                .long("api")
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
        .arg(
            Arg::new("marker")
                .short('m')
                .long("marker")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let api = args.value_of("api").unwrap();
    let cmd = match args.value_of("command") {
        Some(x) => x,
        None => "",
    };
    let term = args.value_of("term").unwrap();
    let value = match args.value_of("value") {
        Some(x) => x,
        None => "",
    };
    let marker = match args.value_of("marker") {
        Some(x) => x,
        None => "",
    };

    let url: Box<dyn Url> = args_to_url(
        api.to_owned(),
        cmd.to_owned(),
        term.to_owned(),
        value.to_owned(),
    );
    let record_str: String = {
        match reqwest::get(&url.generate()) {
            Ok(res) => res,
            Err(e) => panic!("{}::{}, {}", marker, term, e),
        }
    }
    .text()
    .unwrap();
    let record: serde_json::Value = serde_json::from_str(&record_str).unwrap();

    match api {
        "go" => {
            let outpath = format!("{}_{}_orthodb_results.tsv", marker, term);
            let handle = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(Path::new(&outpath))
                .unwrap();

            let mut json = GoOntologyJson::from(record);
            json.set_marker(&marker);
            json.to_csv(&handle);
        }
        "odb" => {
            let outpath = format!("{}_orthodb_results.tsv", marker);
            let handle = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(Path::new(&outpath))
                .unwrap();
            let mut wtr = csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_writer(handle);
            for hm in get_data(&record, "name").unwrap() {
                wtr.write_record(&[
                    &format!("{}", &marker),
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
        _ => (),
    }
}
