use chrono::{DateTime, Utc};
use crate::models::base::{object_store_ref::ObjRef, content::Element};

pub trait DurationDateable {
	fn start_date() -> DateTime<Utc>;
	fn end_date() -> Option<DateTime<Utc>>;
}

pub trait Describable {
	fn short_description() -> String;
	fn long_description() -> Vec<Element>;
}

pub trait Featurable {
	fn is_featured() -> bool;
}

pub trait Attachable {
	fn attachment() -> Option<ObjRef>;
	fn attachment_type() -> AttachmentType;
}

pub enum AttachmentType{
	Image,
	Document,
	Certificate,
	Audio,
	Video,
	Other { custom_type: String }
}

pub trait Achievable {
	fn achievements() -> Vec<String>;
}

pub trait Gradable {
	fn grade(&self) -> String;
	fn is_passing(&self) -> bool;
}
