// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Event;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "AtspiEventListener")]
    pub struct EventListener(Object<ffi::AtspiEventListener, ffi::AtspiEventListenerClass>);

    match fn {
        type_ => || ffi::atspi_event_listener_get_type(),
    }
}

impl EventListener {
    #[doc(alias = "atspi_event_listener_new")]
    pub fn new<P: Fn(&Event) + 'static>(callback: P) -> EventListener {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: Fn(&Event) + 'static>(event: *mut ffi::AtspiEvent, user_data: glib::ffi::gpointer) {
            let event = from_glib_full(event);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&event);
        }
        let callback = Some(callback_func::<P> as _);
        unsafe extern "C" fn callback_destroyed_func<P: Fn(&Event) + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call2 = Some(callback_destroyed_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            from_glib_full(ffi::atspi_event_listener_new(callback, Box_::into_raw(super_callback0) as *mut _, destroy_call2))
        }
    }

    //#[doc(alias = "atspi_event_listener_new_simple")]
    //pub fn new_simple<P: Fn(&Event) + 'static>(callback: P) -> EventListener {
    //    unsafe { TODO: call ffi:atspi_event_listener_new_simple() }
    //}

    #[doc(alias = "atspi_event_listener_deregister_from_callback")]
    pub fn deregister_from_callback<P: FnMut(&Event)>(callback: P, event_type: &str) -> Result<(), glib::Error> {
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&Event)>(event: *mut ffi::AtspiEvent, user_data: glib::ffi::gpointer) {
            let event = from_glib_full(event);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&event);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_event_listener_deregister_from_callback(callback, super_callback0 as *const _ as usize as *mut _, event_type.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //#[doc(alias = "atspi_event_listener_deregister_no_data")]
    //pub fn deregister_no_data<P: FnMut(&Event)>(callback: P, event_type: &str) -> Result<(), glib::Error> {
    //    unsafe { TODO: call ffi:atspi_event_listener_deregister_no_data() }
    //}

    #[doc(alias = "atspi_event_listener_register_from_callback")]
    pub fn register_from_callback<P: Fn(&Event) + 'static>(callback: P, event_type: &str) -> Result<(), glib::Error> {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: Fn(&Event) + 'static>(event: *mut ffi::AtspiEvent, user_data: glib::ffi::gpointer) {
            let event = from_glib_full(event);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&event);
        }
        let callback = Some(callback_func::<P> as _);
        unsafe extern "C" fn callback_destroyed_func<P: Fn(&Event) + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call2 = Some(callback_destroyed_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_event_listener_register_from_callback(callback, Box_::into_raw(super_callback0) as *mut _, destroy_call2, event_type.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //#[doc(alias = "atspi_event_listener_register_from_callback_full")]
    //pub fn register_from_callback_full(callback: Option<Box_<dyn Fn(&Event) + 'static>>, event_type: &str, properties: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 28 }) -> Result<(), glib::Error> {
    //    unsafe { TODO: call ffi:atspi_event_listener_register_from_callback_full() }
    //}

    //#[doc(alias = "atspi_event_listener_register_no_data")]
    //pub fn register_no_data<P: Fn(&Event) + 'static>(callback: P, event_type: &str) -> Result<(), glib::Error> {
    //    unsafe { TODO: call ffi:atspi_event_listener_register_no_data() }
    //}
}

pub const NONE_EVENT_LISTENER: Option<&EventListener> = None;

pub trait EventListenerExt: 'static {
    #[doc(alias = "atspi_event_listener_deregister")]
    fn deregister(&self, event_type: &str) -> Result<(), glib::Error>;

    #[doc(alias = "atspi_event_listener_register")]
    fn register(&self, event_type: &str) -> Result<(), glib::Error>;

    //#[doc(alias = "atspi_event_listener_register_full")]
    //fn register_full(&self, event_type: &str, properties: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 28 }) -> Result<(), glib::Error>;
}

impl<O: IsA<EventListener>> EventListenerExt for O {
    fn deregister(&self, event_type: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_event_listener_deregister(self.as_ref().to_glib_none().0, event_type.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn register(&self, event_type: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::atspi_event_listener_register(self.as_ref().to_glib_none().0, event_type.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //fn register_full(&self, event_type: &str, properties: /*Unknown conversion*//*Unimplemented*/Array TypeId { ns_id: 0, id: 28 }) -> Result<(), glib::Error> {
    //    unsafe { TODO: call ffi:atspi_event_listener_register_full() }
    //}
}

impl fmt::Display for EventListener {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EventListener")
    }
}
