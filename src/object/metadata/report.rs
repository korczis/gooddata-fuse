use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ReportContent {
    pub domains: Option<Vec<String>>,
    pub definitions: Option<Vec<String>>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ReportBody {
    pub content: ReportContent,
    pub meta: super::MetadataMeta,
}

impl ReportBody {
    pub fn meta(&self) -> &super::MetadataMeta {
        &self.meta
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Report {
    pub report: ReportBody,
}

impl Report {
    pub fn report(&self) -> &ReportBody {
        &self.report
    }
}

impl Into<String> for Report {
    fn into(self) -> String {
        format!("{}\n", json::as_pretty_json(&self).to_string())
    }
}

impl super::MetadataObjectsBody<Report> {
    pub fn items(&self) -> &Vec<Report> {
        &self.items
    }
}

impl super::MetadataObjects<super::MetadataObjectsBody<Report>> {
    pub fn objects(&self) -> &super::MetadataObjectsBody<Report> {
        &self.objects
    }

    pub fn find_by_identifier(&self, identifier: &String) -> (u32, Option<Report>) {
        let mut i: u32 = 0;
        for item in self.objects().items().into_iter() {
            if item.report().meta().identifier().as_ref().unwrap() == identifier {
                return (i, Some(item.clone()));
            }

            i += 1;
        }

        (0, None)
    }
}
