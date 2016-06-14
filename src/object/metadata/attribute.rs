use std::collections::HashMap;

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct PkContent {
	data: Option<String>,
	// type: Option<String>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ContentBody {
	pub formOf: Option<String>,
	pub expression: Option<String>,
	// type: Option<String>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct DisplayForms {
	pub content: Option<ContentBody>,
	pub links: Option<HashMap<String, String>>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct AttributeContent {
    pub direction: Option<String>,
    pub sort: Option<String>,
    pub pk: Option<PkContent>,
    pub dimension: Option<String>,
    // pub type: Option<String>,
    pub displayForms: Option<Vec<DisplayForms>>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct AttributeBody {
    pub content: AttributeContent,
    pub meta: super::MetadataMeta,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Attribute {
    pub attribute: AttributeBody,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsAttributeBody {
    pub paging: super::MetadataPaging,
    pub items: Vec<Attribute>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsAttribute {
    pub objects: ObjectsAttributeBody,
}
