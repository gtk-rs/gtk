
use gtk;
use gtk::traits::Widget;

pub fn to_entry(widget: &Widget) -> gtk::Entry {
    Widget::wrap_widget(widget.get_widget())
}