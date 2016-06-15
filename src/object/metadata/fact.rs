use rustc_serialize::json;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ExprItem {
    pub data: Option<String>, // pub type: Option<String>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FactcContent {
    pub expr: Option<Vec<ExprItem>>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FactBody {
    pub content: FactcContent,
    pub meta: super::MetadataMeta,
}

impl FactBody {
    pub fn meta(&self) -> &super::MetadataMeta {
        &self.meta
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Fact {
    pub fact: FactBody,
}

impl Fact {
    pub fn fact(&self) -> &FactBody {
        &self.fact
    }
}

impl Into<String> for Fact {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsFactBody {
    pub paging: super::MetadataPaging,
    pub items: Vec<Fact>,
}

impl ObjectsFactBody {
    pub fn items(&self) -> &Vec<Fact> {
        &self.items
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsFact {
    pub objects: ObjectsFactBody,
}

impl ObjectsFact {
    pub fn objects(&self) -> &ObjectsFactBody {
        &self.objects
    }

    pub fn find_by_identifier(&self, identifier: &String) -> (u32, Option<Fact>) {
        let mut i: u32 = 0;
        for item in self.objects().items().into_iter() {
            if item.fact().meta().identifier().as_ref().unwrap() == identifier {
                return (i, Some(item.clone()));
            }

            i += 1;
        }

        (0, None)
    }
}
