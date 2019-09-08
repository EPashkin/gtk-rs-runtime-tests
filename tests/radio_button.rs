extern crate gtk;

use gtk::prelude::*;
use gtk::RadioButton;

#[test]
fn radio_button() {
    gtk::init().unwrap();

    let rb1 = RadioButton::new();
    assert_eq!(rb1.get_group(), vec![rb1.clone()]);
    let rb2 = RadioButton::new_with_label("test");
    assert_eq!(rb2.get_group(), vec![rb2.clone()]);
    let rb3 = RadioButton::new_with_mnemonic("_test");
    assert_eq!(rb3.get_group(), vec![rb3.clone()]);
    rb1.join_group(Some(&rb2));
    assert_eq!(rb1.get_group(), vec![rb1.clone(), rb2.clone()]);
    rb1.join_group(gtk::NONE_RADIO_BUTTON);
    assert_eq!(rb1.get_group(), vec![rb1.clone()]);
    let rb4 = RadioButton::new_from_widget(&rb1);
    assert_eq!(rb4.get_group(), vec![rb4.clone(), rb1.clone()]);
    let rb5 = RadioButton::new_with_label_from_widget(&rb2, "test");
    assert_eq!(rb5.get_group(), vec![rb5.clone(), rb2.clone()]);
    let rb6 = RadioButton::new_with_mnemonic_from_widget(&rb3, "_test");
    assert_eq!(rb6.get_group(), vec![rb6.clone(), rb3.clone()]);
}
