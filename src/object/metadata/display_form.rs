// use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
#[allow(non_snake_case)]
pub struct DisplayFormContent {
    pub formOf: Option<String>,
    pub expression: Option<Vec<String>>,
    pub type_: Option<String>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
#[allow(non_snake_case)]
pub struct DisplayForm {
    pub displayForm: super::MetadataObjectBody<DisplayFormContent>,
}
