use crate::{impl_attachable, impl_base};

pub struct Company {
	pub name: String,
	pub location: Option<String>,
	pub industry: Option<String>,
	pub company_type: CompanyType,
}

pub enum CompanyType {
	EducationalInstitution,
	Corporation,
	Startup,
	Government,
	NonProfit,
	Client,
}

impl_base!(Company);
impl_attachable!(Company);
