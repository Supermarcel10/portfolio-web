use crate::base::object_store_ref::ObjRef;

pub enum Element {
	HTMLElement { html: String },
	ImageElement { html: String, source: ObjRef, alt: Option<ObjRef> }
}
