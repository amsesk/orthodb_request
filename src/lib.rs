use csv;
use serde_json;
use std::collections::HashMap;
use std::convert::From;
use std::fmt;

static ORTHODB_BASE_URL: &str = "https://www.orthodb.org";
static GO_BASE_URL: &str = "http://api.geneontology.org/api/ontology/term";
static DATALABEL: &str = "data";

pub trait Url {
    fn generate(&self) -> String;
}

pub struct GoUrl {
    base: &'static str,
    term: String,
}

impl GoUrl {
    pub fn new(term: String) -> GoUrl {
        GoUrl {
            base: GO_BASE_URL,
            term: term,
        }
    }
}

impl Url for GoUrl {
    fn generate(&self) -> String {
        let mut url: String = String::new();
        url.push_str(&self.base);
        url.push('/');
        url.push_str(&self.term);

        url
    }
}

pub struct OrthoDbUrl {
    base: &'static str,
    cmd: String,
    term: String,
    value: String,
}

impl OrthoDbUrl {
    pub fn new(cmd: String, term: String, value: String) -> OrthoDbUrl {
        OrthoDbUrl {
            base: ORTHODB_BASE_URL,
            cmd,
            term,
            value,
        }
    }
}

impl Url for OrthoDbUrl {
    fn generate(&self) -> String {
        let mut url: String = String::new();
        url.push_str(&self.base);
        url.push('/');
        url.push_str(&self.cmd);
        url.push('?');
        url.push_str(&self.term);
        url.push('=');
        url.push_str(&self.value);

        url
    }
}

pub struct GoOntologyJson {
    marker: String,
    goid: String,
    label: String,
    definition: String,
}

impl GoOntologyJson {
    pub fn to_csv(&self, file_handle: &std::fs::File) {
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .from_writer(file_handle);
        wtr.write_record(&[
            &format!("{}", &self.marker),
            &format!("{}", &self.goid),
            &format!("{}", &self.label),
            &format!("{}", &self.definition),
        ])
        .unwrap()
    }
    pub fn set_marker(&mut self, marker: &str) -> () {
        self.marker = marker.to_owned();
    }
}

impl From<serde_json::Value> for GoOntologyJson {
    fn from(record: serde_json::Value) -> Self {
        GoOntologyJson {
            marker: "".to_owned(),
            goid: unpack_str_value(&record["goid"]),
            label: unpack_str_value(&record["label"]),
            definition: unpack_str_value(&record["definition"]),
        }
    }
}

impl fmt::Display for GoOntologyJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "goid: {:?}\nlabel: {:?}\ndefinition: {:?}",
            self.goid, self.label, self.definition
        )
    }
}

pub fn unpack_str_value(v: &serde_json::Value) -> String {
    match v.as_str() {
        Some(x) => x.to_owned(),
        None => "NA".to_owned(),
    }
}

pub fn args_to_url(api: String, cmd: String, term: String, value: String) -> Box<dyn Url> {
    if api == "go" {
        Box::new(GoUrl::new(String::from(term)))
    } else if api == "odb" {
        Box::new(OrthoDbUrl::new(
            String::from(cmd),
            String::from(term),
            String::from(value),
        ))
    } else {
        panic!("Unrecognized API: {}", &api)
    }
}
pub fn get_data(
    value: &serde_json::Value,
    label: &str,
) -> Option<Vec<HashMap<String, serde_json::Value>>> {
    let search = &value[DATALABEL][label];

    if search.is_null() {
        return None;
    }

    let mut retvec: Vec<HashMap<String, serde_json::Value>> = vec![];

    if search.is_array() {
        match search.as_array() {
            Some(result) => {
                for entry in result {
                    let mut map = HashMap::new();
                    if let Some(elements) = entry.as_object() {
                        for e in elements {
                            let (key, val) = e;
                            map.insert(key.to_owned(), val.to_owned());
                        }
                        retvec.push(map);
                    }
                }
            }
            None => (),
        }

        return Some(retvec);
    } else {
        if search.is_string() {
            let mut map = HashMap::new();
            map.insert(label.to_owned(), search.to_owned());
            retvec.push(map);
            return Some(retvec);
        }
        return None;
    }
}
