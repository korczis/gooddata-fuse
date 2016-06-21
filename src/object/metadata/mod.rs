// use rustc_serialize;
// use rustc_serialize::json;

pub mod attribute;
pub mod fact;
pub mod metric;
pub mod report;
pub mod report_definition;

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

pub trait MetadataMetaGetters {
    fn identifier(&self) -> &Option<String>;
}

impl MetadataMetaGetters for MetadataMeta {
    fn identifier(&self) -> &Option<String> {
        &self.identifier
    }
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

pub trait MetadataPagingGetters {
    fn next(&self) -> &Option<String>;
    fn count(&self) -> &Option<u32>;
    fn offset(&self) -> &Option<u32>;
}

impl MetadataPagingGetters for MetadataPaging {
    fn next(&self) -> &Option<String> {
        &self.next
    }

    fn count(&self) -> &Option<u32> {
        &self.count
    }
    fn offset(&self) -> &Option<u32> {
        &self.offset
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetadataObjectBody<T> {
    pub content: T,
    pub meta: super::MetadataMeta,
}

pub trait MetadataObjectBodyGetters<T> {
    fn content(&self) -> &T;
    fn meta(&self) -> &super::MetadataMeta;
}

impl<T> MetadataObjectBodyGetters<T> for MetadataObjectBody<T> {
    fn content(&self) -> &T {
        &self.content
    }

    fn meta(&self) -> &super::MetadataMeta {
        &self.meta
    }
}

impl<T> MetadataObjectBody<T> {
    pub fn content(&self) -> &T {
        &self.content
    }

    pub fn meta(&self) -> &super::MetadataMeta {
        &self.meta
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetadataObjectsBody<T> {
    pub paging: super::MetadataPaging,
    pub items: Vec<T>,
}

pub trait MetadataObjectsBodyGetters<T> {
    fn paging(&self) -> &super::MetadataPaging;
    fn items(&self) -> &Vec<T>;
}

impl<T> MetadataObjectsBodyGetters<T> for MetadataObjectsBody<T> {
    fn paging(&self) -> &super::MetadataPaging {
        &self.paging
    }

    fn items(&self) -> &Vec<T> {
        &self.items
    }
}

impl<T> super::MetadataObjectsBody<T> {
    pub fn items(&self) -> &Vec<T> {
        &self.items
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct MetadataObjects<T> {
    pub objects: T,
}

pub trait MetadataObjectsGetter<T> {
    fn objects(&self) -> &T;
}

impl<T> MetadataObjectsGetter<T> for MetadataObjects<T> {
    fn objects(&self) -> &T {
        &self.objects
    }
}

impl<T> super::MetadataObjects<T> {
    pub fn objects(&self) -> &T {
        &self.objects
    }

    // pub fn find_by_identifier2(&self, identifier: &String) -> (u32, Option<T>) {
    //     let mut i: u32 = 0;
    //     for item in self.objects().items().into_iter() {
    //         if item.object().meta().identifier().as_ref().unwrap() == identifier {
    //             return (i, Some(item.clone()));
    //         }
    //
    //         i += 1;
    //     }
    //
    //     (0, None)
    // }
}

trait MetadataObjectGetter<T> {
    fn object(&self) -> &MetadataObjectBody<T>;
}

pub trait MetadataObjectRootKey {
    fn root_key() -> String;
}
