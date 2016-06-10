mod attribute;
mod fact;
mod metric;
mod project_dashboard;
mod report;
mod report_definition;

pub use self::attribute::*;
pub use self::fact::*;
pub use self::metric::*;
pub use self::project_dashboard::*;
pub use self::report::*;
pub use self::report_definition::*;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetadataMeta {
    pub author: Option<String>,
    pub uri: Option<String>,
    pub tags: Option<String>,
    pub created: Option<String>,
    pub identifier: Option<String>,
    pub deprecated: Option<String>,
    pub summary: Option<String>,
    pub isProduction: Option<u8>,
    pub title: Option<String>,
    pub category: Option<String>,
    pub updated: Option<String>,
    pub contributor: Option<String>,
}

impl MetadataMeta {
    pub fn identifier(&self) -> &Option<String> {
        &self.identifier
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetadataPaging {
    pub next: Option<String>,
    pub count: Option<u32>,
    pub offset: Option<u32>,
}
