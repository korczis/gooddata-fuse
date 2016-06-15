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

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ReportPaging {
    pub next: Option<String>,
    pub count: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsReportBody {
    pub paging: super::MetadataPaging,
    pub items: Vec<Report>,
}

impl ObjectsReportBody {
    pub fn items(&self) -> &Vec<Report> {
        &self.items
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsReport {
    pub objects: ObjectsReportBody,
}

impl ObjectsReport {
    pub fn objects(&self) -> &ObjectsReportBody {
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
