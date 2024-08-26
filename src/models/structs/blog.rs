use crate::models::base::content::Element;
use crate::models::base::object_store_ref::ObjRef;
use crate::models::traits::{Attachable, AttachmentType, Describable, Featurable};

pub struct Blog {
	pub slug: String,
	pub tags: Vec<String>,
	pub content: Vec<Element>,
	pub is_draft: bool
}

impl Describable for Blog {
	fn short_description() -> String {
		todo!()
	}

	fn long_description() -> Vec<Element> {
		todo!()
	}
}

impl Featurable for Blog {
	fn is_featured() -> bool {
		todo!()
	}
}

impl Attachable for Blog {
	fn attachment() -> Option<ObjRef> {
		todo!()
	}

	fn attachment_type() -> AttachmentType {
		todo!()
	}
}
