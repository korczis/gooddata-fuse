use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
#[allow(non_snake_case)]
pub struct AttributeContent {
    pub direction: Option<String>,
    pub sort: Option<String>,
    pub pk: Option<Vec<super::key::Key>>,
    pub dimension: Option<String>,
    pub type_: Option<String>,
    pub displayForms: Option<Vec<super::display_form::DisplayFormContent>>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Attribute {
    pub attribute: super::MetadataObjectBody<AttributeContent>,
}

impl Attribute {
    pub fn object(&self) -> &super::MetadataObjectBody<AttributeContent> {
        &self.attribute
    }
}

impl super::MetadataObjectGetter<AttributeContent> for Attribute {
    fn object(&self) -> &super::MetadataObjectBody<AttributeContent> {
        &self.object()
    }
}

impl Into<String> for Attribute {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}

pub const NAME: &'static str = "attribute";

impl super::MetadataObjectRootKey for Attribute {
    fn root_key() -> String {
        NAME.to_string()
    }
}

impl super::MetadataQuery<super::MetadataQueryBody<Attribute>> {
    pub fn find_by_identifier(&self, identifier: &String) -> (u32, Option<Attribute>) {
        let mut i: u32 = 0;
        for item in self.objects().items().into_iter() {
            if item.object().meta().identifier().as_ref().unwrap() == identifier {
                return (i, Some(item.clone()));
            }

            i += 1;
        }

        (0, None)
    }
}
