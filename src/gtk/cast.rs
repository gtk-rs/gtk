
use gtk;
use gtk::traits::WidgetTrait;

pub fn to_entry(widget: &WidgetTrait) -> gtk::Entry {
    WidgetTrait::wrap_widget(widget.get_widget())
}