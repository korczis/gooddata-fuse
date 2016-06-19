use rustc_serialize;

pub trait IntoJsonString<T: rustc_serialize::Encodable> {
    fn into(self) -> String;
}
