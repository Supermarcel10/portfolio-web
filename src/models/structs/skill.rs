use crate::models::base::entity::Base;

pub struct Skill {
	base: Base,
	name: String,
	category: SkillCategory,
	proficiency: Option<u8>,
	description: Option<String>,
}

pub enum SkillCategory {
	SoftSkill{ skill: String },
	Technology { technology_name: String, version: String },
	Tool { tool_name: String, version: Option<String> },
}
