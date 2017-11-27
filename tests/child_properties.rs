extern crate gdk;
extern crate gtk;
extern crate gtk_sys as ffi;

use gdk::ModifierType;
use gtk::{BoxExt, Button, CellRendererAccel, CellRendererAccelExt, ContainerExt, PackType};
use gtk::Orientation::Vertical;

#[test]
fn child_properties() {
    gtk::init().unwrap();

    let vbox = gtk::Box::new(Vertical, 0);
    let button = Button::new();
    vbox.add(&button);

    //gboolean
    assert_eq!(false, vbox.get_child_expand(&button));
    vbox.set_child_expand(&button, true);
    assert_eq!(true, vbox.get_child_expand(&button));

    //guint
    assert_eq!(0, vbox.get_child_padding(&button));
    vbox.set_child_padding(&button, 50);
    assert_eq!(50, vbox.get_child_padding(&button));

    //enum Gtk.PackType
    assert_eq!(PackType::Start, vbox.get_child_pack_type(&button));
    vbox.set_child_pack_type(&button, PackType::End);
    assert_eq!(PackType::End, vbox.get_child_pack_type(&button));

    //flags Gdk.ModifierType
    let accel = CellRendererAccel::new();
    let flags = ModifierType::SHIFT_MASK | ModifierType::CONTROL_MASK;
    assert_eq!(ModifierType::empty(), accel.get_property_accel_mods());
    accel.set_property_accel_mods(flags);
    assert_eq!(flags, accel.get_property_accel_mods());
}
