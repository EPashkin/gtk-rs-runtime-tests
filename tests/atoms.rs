extern crate gdk;
extern crate gtk;
extern crate glib;

use glib::translate::*;

#[test]
fn atoms() {
    assert_eq!(1 + 1, 2);
//    gtk::init().unwrap();

    // Predefined atom
    let atom = gdk::SELECTION_CLIPBOARD;
    assert_eq!(format!("{:?}", atom.to_glib_none().0), "0x45");
    assert_eq!(format!("{:?}", atom.name()), "\"CLIPBOARD\"");
/*
    // Custom atom
    let atom2: gdk::Atom = "teststr".into();
    assert_eq!(format!("{:?}", atom2.name()), "\"teststr\"");

    // Vector passing
    let mut atoms = vec![&gdk::SELECTION_TYPE_BITMAP, &gdk::SELECTION_TYPE_PIXMAP];
    assert!(!gtk::targets_include_text(&atoms));

    atoms.push(&gdk::SELECTION_TYPE_STRING);
    assert!(gtk::targets_include_text(&atoms));
*/
}
