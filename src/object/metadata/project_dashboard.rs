#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct IframeItem {
	positionX: Option<u8>,
	positionY: Option<u8>,
	sizeY: Option<u8>,
	sizeX: Option<u8>,
	url: Option<String>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct HeadlineItem {
	positionX: Option<u8>,
	positionY: Option<u8>,
	sizeY: Option<u8>,
	sizeX: Option<u8>,
	style: Option<String>,
	visualization: Option<String>,
	filters: Option<String>,
	obj: Option<String>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ReportItem {
	positionX: Option<u8>,
	positionY: Option<u8>,
	sizeY: Option<u8>,
	sizeX: Option<u8>,
	style: Option<String>,
	visualization: Option<String>,
	filters: Option<String>,
	metric: Option<String>,
	linkedWithExternalFilter: Option<String>,
	format: Option<String>,
	title: Option<String>,
	constraint: Option<String>,
	filterAttributeDF: Option<String>,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub enum ProjectDashboardItems {
	IframeItem { iframeItem: IframeItem },
	HeadlineItem { iframeItem: HeadlineItem },
	ReportItem { iframeItem: ReportItem },
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ProjectDashboardTabs {
    pub identifier: Option<String>,
    pub title: Option<String>,
    pub items: Option<Vec<ProjectDashboardItems>>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ProjectDashboardContent {
    pub tabs: Option<Vec<ProjectDashboardTabs>>,
    pub filters: super::MetadataMeta,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ProjectDashboardBody {
    pub content: Option<ProjectDashboardContent>,
    pub meta: super::MetadataMeta,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ProjectDashboard {
    pub ProjectDashboard: ProjectDashboardBody,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsProjectDashboardBody {
    pub paging: super::MetadataPaging,
    pub items: Vec<ProjectDashboard>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct ObjectsProjectDashboard {
    pub objects: ObjectsProjectDashboardBody,
}
