use rustc_serialize::json;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ExprItem {
    pub data: Option<String>, // pub type: Option<String>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FactContent {
    pub expr: Option<Vec<ExprItem>>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Fact {
    pub fact: super::MetadataObjectBody<FactContent>,
}

impl Fact {
    pub fn object(&self) -> &super::MetadataObjectBody<FactContent> {
        &self.fact
    }
}

impl super::MetadataObjectGetter<FactContent> for Fact {
    fn object(&self) -> &super::MetadataObjectBody<FactContent> {
        &self.fact
    }
}

impl Into<String> for Fact {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}

pub const NAME: &'static str = "fact";

impl super::MetadataObjectRootKey for Fact {
    fn root_key() -> String {
        NAME.to_string()
    }
}

impl super::MetadataObjects<super::MetadataObjectsBody<Fact>> {
    pub fn find_by_identifier(&self, identifier: &String) -> (u32, Option<Fact>) {
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
