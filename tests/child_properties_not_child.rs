extern crate gtk;
extern crate gtk_sys as ffi;

use gtk::{ContainerExt, FixedExtManual};

#[test]
#[should_panic]
fn child_properties_not_child() {
    gtk::init().unwrap();
    
    let container = gtk::Fixed::new();

    let button1 = gtk::Button::new_with_label("Button 1");
    let button2 = gtk::Button::new_with_label("Button 2");
    container.add(&button1);
    let _ = container.get_child_y(&button1);

    // Should panic here
    let _ = container.get_child_y(&button2);
}
