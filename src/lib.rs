static ORTHODB_BASE_URL: &str = "https://www.orthodb.org";

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
