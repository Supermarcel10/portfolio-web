use crate::base::entity::impl_base;

pub struct Skill {
	pub name: String,
	pub category: SkillCategory,
	pub proficiency: Option<u8>,
	pub description: Option<String>,
}

pub enum SkillCategory {
	SoftSkill {
		skill: String,
	},
	Technology {
		technology_name: String,
		version: String,
	},
	Tool {
		tool_name: String,
		version: Option<String>,
	},
}

impl_base!(Skill);
