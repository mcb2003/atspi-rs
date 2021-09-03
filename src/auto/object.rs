// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use std::fmt;

glib::wrapper! {
    #[doc(alias = "AtspiObject")]
    pub struct Object(Object<ffi::AtspiObject, ffi::AtspiObjectClass>);

    match fn {
        type_ => || ffi::atspi_object_get_type(),
    }
}

impl Object {}

pub const NONE_OBJECT: Option<&Object> = None;

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Object")
    }
}
