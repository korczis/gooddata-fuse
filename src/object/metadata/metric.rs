use rustc_serialize::json;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetricTreePosition {
    pub column: Option<u16>,
    pub line: Option<u16>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetricTreeNode {
    pub content: Option<Vec<MetricTreeNode>>,
    pub position: MetricTreePosition, // pub type: Option<String>,
}


#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetricContent {
    pub folders: Option<Vec<String>>,
    pub format: Option<String>,
    pub tree: MetricTreeNode,
    pub expression: Option<String>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Metric {
    pub metric: super::MetadataObjectBody<MetricContent>,
}

impl Metric {
    pub fn object(&self) -> &super::MetadataObjectBody<MetricContent> {
        &self.metric
    }
}

impl super::MetadataObjectGetter<MetricContent> for Metric {
    fn object(&self) -> &super::MetadataObjectBody<MetricContent> {
        &self.object()
    }
}

impl Into<String> for Metric {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}

pub const NAME: &'static str = "metric";

impl super::MetadataObjectRootKey for Metric {
    fn root_key() -> String {
        NAME.to_string()
    }
}

impl super::MetadataQuery<super::MetadataQueryBody<Metric>> {
    pub fn find_by_identifier(&self, identifier: &String) -> (u32, Option<Metric>) {
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
