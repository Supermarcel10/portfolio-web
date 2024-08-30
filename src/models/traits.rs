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

#[macro_export]
macro_rules! impl_duration_dateable {
	($type:ty) => {
		impl $crate::models::traits::DurationDateable for $type {
			fn start_date() -> chrono::DateTime<chrono::Utc> {
				todo!()
			}

			fn end_date() -> Option<chrono::DateTime<chrono::Utc>> {
				todo!()
			}
		}
	}
}

#[macro_export]
macro_rules! impl_describable {
	($type:ty) => {
		impl $crate::models::traits::Describable for $type {
			fn short_description() -> String {
				todo!()
			}

			fn long_description() -> Vec<$crate::models::base::content::Element> {
				todo!()
			}
		}
	}
}

#[macro_export]
macro_rules! impl_featurable {
	($type:ty) => {
		impl $crate::models::traits::Featurable for $type {
			fn is_featured() -> bool {
				todo!()
			}
		}
	}
}

#[macro_export]
macro_rules! impl_attachable {
	($type:ty) => {
		impl $crate::models::traits::Attachable for $type {
			fn attachment() -> Option<$crate::models::base::object_store_ref::ObjRef> {
				todo!()
			}

			fn attachment_type() -> $crate::models::traits::AttachmentType {
				todo!()
			}
		}
	}
}

#[macro_export]
macro_rules! impl_achievable {
	($type:ty) => {
		impl $crate::models::traits::Achievable for $type {
			fn achievements() -> Vec<String> {
				todo!()
			}
		}
	}
}

#[macro_export]
macro_rules! impl_gradeable {
	($type:ty) => {
		impl $crate::models::traits::Gradable for $type {
			fn grade(&self) -> String {
				todo!()
			}

			fn is_passing(&self) -> bool {
				todo!()
			}
		}
	}
}

pub use {
	impl_duration_dateable,
	impl_describable,
	impl_featurable,
	impl_attachable,
	impl_achievable,
	impl_gradeable
};
