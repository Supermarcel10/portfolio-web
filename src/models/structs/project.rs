use chrono::{DateTime, Utc};
use crate::models::base::content::Element;
use crate::models::base::entity::Base;
use crate::models::base::object_store_ref::ObjRef;
use crate::models::traits::{Attachable, AttachmentType, Describable, DurationDateable, Featurable};

pub struct Project {
	pub base: Base,
	pub status: ProjectStatus,
	pub challenges: Vec<String>,
	pub outcomes: Vec<String>,
	pub category: Option<String>,
	pub github_url: Option<String>,
	pub deployment_url: Option<String>
}

pub enum ProjectStatus {
	Planning,
	Designing,
	Developing,
	Released { version: String },
	Updating { current_version: String, target_version: String }
}

impl DurationDateable for Project {
	fn start_date() -> DateTime<Utc> {
		todo!()
	}

	fn end_date() -> Option<DateTime<Utc>> {
		todo!()
	}
}

impl Describable for Project {
	fn short_description() -> String {
		todo!()
	}

	fn long_description() -> Vec<Element> {
		todo!()
	}
}

impl Featurable for Project {
	fn is_featured() -> bool {
		todo!()
	}
}

impl Attachable for Project {
	fn attachment() -> Option<ObjRef> {
		todo!()
	}

	fn attachment_type() -> AttachmentType {
		todo!()
	}
}
