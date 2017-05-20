extern crate gtk;
extern crate gtk_sys;
extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;

use glib::translate::ToGlibPtr;

#[test]
fn floating_ref_leak() {
    gtk::init().unwrap();

    let ptr = {
        let button = gtk::Button::new();
        let ptr = get_pointer(&button);
        assert_eq!(get_ref_count(ptr), 1);
        let c = button.clone();
        let ptr = get_pointer(&c);
        assert_eq!(get_ref_count(ptr), 2);
        ptr
    };
    //Warning: direct access to freed memory
    assert_eq!(get_ref_count(ptr), 0);
}

fn get_pointer(btn: &gtk::Button) -> *const gobject_sys::GObject {
    let ptr: *const gtk_sys::GtkButton = btn.to_glib_none().0;
    ptr as *const gobject_sys::GObject
}

fn get_ref_count(obj: *const gobject_sys::GObject) -> u32 {
    unsafe { *(&(*obj).ref_count as *const glib_sys::Volatile<u32> as *const u32) }
}
