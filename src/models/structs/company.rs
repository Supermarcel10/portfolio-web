use crate::models::base::object_store_ref::ObjRef;
use crate::models::traits::{Attachable, AttachmentType};

pub struct Company {
	name : String,
	location : Option<String>,
	industry : Option<String>,
	company_type : CompanyType
}

pub enum CompanyType {
	EducationalInstitution,
	Corporation,
	Startup,
	Government,
	NonProfit,
	Client
}

impl Attachable for Company {
	fn attachment() -> Option<ObjRef> {
		todo!()
	}

	fn attachment_type() -> AttachmentType {
		todo!()
	}
}
