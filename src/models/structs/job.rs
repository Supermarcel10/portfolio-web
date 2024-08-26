use chrono::{DateTime, Utc};
use crate::models::base::content::Element;
use crate::models::base::object_store_ref::ObjRef;
use crate::models::traits::{Achievable, Attachable, AttachmentType, Describable, DurationDateable};
use super::company::Company;

pub struct Job {
	pub company: Company,
	pub responsibilities: Vec<String>
}

impl DurationDateable for Job {
	fn start_date() -> DateTime<Utc> {
		todo!()
	}

	fn end_date() -> Option<DateTime<Utc>> {
		todo!()
	}
}

impl Describable for Job {
	fn short_description() -> String {
		todo!()
	}

	fn long_description() -> Vec<Element> {
		todo!()
	}
}

impl Achievable for Job	{
	fn achievements() -> Vec<String> {
		todo!()
	}
}

impl Attachable for Job {
	fn attachment() -> Option<ObjRef> {
		todo!()
	}

	fn attachment_type() -> AttachmentType {
		todo!()
	}
}
