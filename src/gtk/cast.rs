
use gtk;
use traits::GtkWidget;

pub fn to_entry(widget: &GtkWidget) -> gtk::Entry {
    GtkWidget::wrap_widget(widget.get_widget())
}