use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Key {
    pub data: Option<String>,
    pub type_: Option<String>,
}

impl Into<String> for Key {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}
