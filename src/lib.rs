use std::collections::HashMap;

static ORTHODB_BASE_URL: &str = "https://www.orthodb.org";
static GO_BASE_URL: &str = "http://api.geneontology.org/api/ontology/term/";
static DATALABEL: &str = "data";

pub trait GenerateUrl {
    fn generate(&self) -> String;
}

pub struct OrthoDbUrl {
    pub base: &'static str,
    pub cmd: String,
    pub term: String,
    pub value: String,
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
impl GenerateUrl for OrthoDbUrl {
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

pub struct GoUrl {
    pub base: &'static str,
    pub term: String,
}

impl GoUrl {
    pub fn new(term: String) -> GoUrl {
        GoUrl {
            base: GO_BASE_URL,
            term: term,
        }
    }
}

impl GenerateUrl for GoUrl {
    fn generate(&self) -> String {
        let mut url: String = String::new();
        url.push_str(ORTHODB_BASE_URL);
        url.push('/');
        url.push_str(&self.term);

        url
    }
}

pub struct Url<T: GenerateUrl> {
    url: T,
}

impl<T> Url<T>
where
    T: GenerateUrl,
{
    pub fn new(api: String, cmd: String, term: String, value: String) -> Self {
        if api == "go" {
            Url {
                url: GoUrl::new(String::from(term)),
            }
        } else if api == "odb" {
            Url {
                url: OrthoDbUrl::new(String::from(cmd), String::from(term), String::from(value)),
            }
        } else {
            panic!()
        }
    }
    pub fn generate(&self) -> String {
        self.url.generate()
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
