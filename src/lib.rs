use std::collections::HashMap;

static ORTHODB_BASE_URL: &str = "https://www.orthodb.org";
static DATALABEL: &str = "data";

pub fn generate_url(cmd: &str, term: &str, value: &str) -> String {
    let mut url: String = String::new();
    url.push_str(ORTHODB_BASE_URL);
    url.push('/');
    url.push_str(&cmd);
    url.push('?');
    url.push_str(&term);
    url.push('=');
    url.push_str(&value);

    url
}

pub fn get_data(value: &serde_json::Value, label: &str) -> Option<Vec<HashMap<String, serde_json::Value>>> {
    let search = &value[DATALABEL][label];
    
    if search.is_null() {
        return None
    }
    if search.is_array() {
        let mut retvec: Vec<HashMap<String, serde_json::Value>> = vec![];
        match search.as_array() {
            Some(result) => {
                for entry in result {
                    let mut map = HashMap::new();
                    if let Some(elements) = entry.as_object() {
                        for e in elements {
                            let (key, val) = e;
                            map.insert(key.to_owned(), val.to_owned());
                        }
                        retvec.push(map.to_owned());
                    }
                }
            }
            None => (),
        }

        return Some(retvec)
    }
    None

}
