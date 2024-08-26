use chrono::{DateTime, Utc};
use crate::models::base::content::Element;
use crate::models::base::object_store_ref::ObjRef;
use crate::models::traits::{Achievable, Attachable, AttachmentType, DurationDateable, Describable, Gradable};
use super::company::Company;

pub struct Education {
	institution: Company,
	degree: String,
	field_of_study: String,
	modules: Vec<Module>
}

pub struct Module {
	code: Option<String>
}

impl DurationDateable for Education {
	fn start_date() -> DateTime<Utc> {
		todo!()
	}

	fn end_date() -> Option<DateTime<Utc>> {
		todo!()
	}
}

impl Describable for Education {
	fn short_description() -> String {
		todo!()
	}

	fn long_description() -> Vec<Element> {
		todo!()
	}
}

impl Achievable for Education {
	fn achievements() -> Vec<String> {
		todo!()
	}
}

impl Attachable for Education {
	fn attachment() -> Option<ObjRef> {
		todo!()
	}

	fn attachment_type() -> AttachmentType {
		todo!()
	}
}

impl Gradable for Education {
	fn grade(&self) -> String {
		todo!()
	}

	fn is_passing(&self) -> bool {
		todo!()
	}
}

impl DurationDateable for Module {
	fn start_date() -> DateTime<Utc> {
		todo!()
	}

	fn end_date() -> Option<DateTime<Utc>> {
		todo!()
	}
}

impl Describable for Module {
	fn short_description() -> String {
		todo!()
	}

	fn long_description() -> Vec<Element> {
		todo!()
	}
}

impl Gradable for Module {
	fn grade(&self) -> String {
		todo!()
	}

	fn is_passing(&self) -> bool {
		todo!()
	}
}
