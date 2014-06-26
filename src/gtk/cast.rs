// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

#![allow(non_snake_case_functions)]
#![allow(visible_private_types)]

use gtk::ffi;

pub fn GTK_WINDOW(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkWindow {
    unsafe { ffi::cast_GtkWindow(widget) }
}

pub fn GTK_BIN(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkBin {
    unsafe { ffi::cast_GtkBin(widget) }
}

pub fn GTK_BUTTON(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkButton {
    unsafe { ffi::cast_GtkButton(widget) }
}

pub fn GTK_CONTAINER(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkContainer {
    unsafe { ffi::cast_GtkContainer(widget) }
}

pub fn GTK_FRAME(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkFrame {
    unsafe { ffi::cast_GtkFrame(widget) }
}

pub fn GTK_LABEL(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkLabel {
    unsafe { ffi::cast_GtkLabel(widget) }
}

pub fn GTK_MISC(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkMisc {
    unsafe { ffi::cast_GtkMisc(widget) }
}

pub fn GTK_ORIENTABLE(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkOrientable {
    unsafe { ffi::cast_GtkOrientable(widget) }
}

pub fn GTK_BOX(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkBox {
    unsafe { ffi::cast_GtkBox(widget) }

}

pub fn GTK_FIXED(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkFixed {
    unsafe { ffi::cast_GtkFixed(widget) }

}

pub fn GTK_BUTTONBOX(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkButtonBox {
    unsafe { ffi::cast_GtkButtonBox(widget) }

}

pub fn GTK_ASPECTFRAME(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkAspectFrame {
    unsafe { ffi::cast_GtkAspectFrame(widget) }

}

pub fn GTK_FONTBUTTON(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkFontButton {
    unsafe { ffi::cast_GtkFontButton(widget) }
}

pub fn GTK_TOGGLEBUTTON(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkToggleButton {
    unsafe { ffi::cast_GtkToggleButton(widget) }
}

pub fn GTK_MENUBUTTON(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkMenuButton {
    unsafe { ffi::cast_GtkMenuButton(widget) }
}

pub fn GTK_COLORBUTTON(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkColorButton {
    unsafe { ffi::cast_GtkColorButton(widget) }
}

pub fn GTK_LINKBUTTON(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkLinkButton {
    unsafe { ffi::cast_GtkLinkButton(widget) }
}

pub fn GTK_SCALEBUTTON(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkScaleButton {
    unsafe { ffi::cast_GtkScaleButton(widget) }
}

pub fn GTK_GRID(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkGrid {
    unsafe { ffi::cast_GtkGrid(widget) }
}

pub fn GTK_ENTRY(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkEntry {
    unsafe { ffi::cast_GtkEntry(widget) }
}

pub fn GTK_SWITCH(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkSwitch {
    unsafe { ffi::cast_GtkSwitch(widget) }
}

pub fn GTK_SCALE(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkScale {
    unsafe { ffi::cast_GtkScale(widget) }
}

pub fn GTK_LEVELBAR(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkLevelBar {
    unsafe { ffi::cast_GtkLevelBar(widget) }
}

pub fn GTK_SEARCHBAR(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkSearchBar {
    unsafe { ffi::cast_GtkSearchBar(widget) }
}

pub fn GTK_SPINBUTTON(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkSpinButton {
    unsafe { ffi::cast_GtkSpinButton(widget) }
}

pub fn GTK_SPINNER(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkSpinner {
    unsafe { ffi::cast_GtkSpinner(widget) }
}

pub fn GTK_PROGRESSBAR(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkProgressBar {
    unsafe { ffi::cast_GtkProgressBar(widget) }
}

pub fn GTK_ARROW(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkArrow {
    unsafe { ffi::cast_GtkArrow(widget) }
}

pub fn GTK_CALENDAR(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkCalendar {
    unsafe { ffi::cast_GtkCalendar(widget) }
}

pub fn GTK_ALIGNMENT(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkAlignment {
    unsafe { ffi::cast_GtkAlignment(widget) }
}

pub fn GTK_EXPANDER(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkExpander {
    unsafe { ffi::cast_GtkExpander(widget) }
}

pub fn GTK_PANED(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkPaned {
    unsafe { ffi::cast_GtkPaned(widget) }
}

pub fn GTK_INFOBAR(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkInfoBar {
    unsafe { ffi::cast_GtkInfoBar(widget) }
}

pub fn GTK_TOOLSHELL(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkToolShell {
    unsafe { ffi::cast_GtkToolShell(widget) }
}

pub fn GTK_TOOLBAR(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkToolbar {
    unsafe{ ffi::cast_GtkToolbar(widget) }
}

pub fn GTK_TOOLITEM(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkToolItem {
    unsafe{ ffi::cast_GtkToolItem(widget) }
}

pub fn GTK_SEPARATORTOOLITEM(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkSeparatorToolItem {
    unsafe{ ffi::cast_GtkSeparatorToolItem(widget) }
}

pub fn GTK_TOOLBUTTON(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkToolButton {
    unsafe{ ffi::cast_GtkToolButton(widget) }
}

pub fn GTK_MENUTOOLBUTTON(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkMenuToolButton {
    unsafe{ ffi::cast_GtkMenuToolButton(widget) }
}

pub fn GTK_TOGGLETOOLBUTTON(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkToggleToolButton {
    unsafe{ ffi::cast_GtkToggleToolButton(widget) }
}

pub fn GTK_RADIOTOOLBUTTON(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkRadioToolButton {
    unsafe{ ffi::cast_GtkRadioToolButton(widget) }
}

pub fn GTK_ADJUSTMENT(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkAdjustment {
    unsafe{ ffi::cast_GtkAdjustment(widget) }
}

pub fn GTK_DIALOG(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkDialog {
    unsafe{ ffi::cast_GtkDialog(widget) }
}

pub fn GTK_ABOUT_DIALOG(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkAboutDialog {
    unsafe{ ffi::cast_GtkAboutDialog(widget) }
}

pub fn GTK_MESSAGE_DIALOG(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkMessageDialog {
    unsafe{ ffi::cast_GtkMessageDialog(widget) }
}

pub fn GTK_COLOR_CHOOSER_DIALOG(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkColorChooserDialog {
    unsafe{ ffi::cast_GtkColorChooserDialog(widget) }
}

pub fn GTK_COLOR_CHOOSER(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkColorChooser {
    unsafe{ ffi::cast_GtkColorChooser(widget) }
}

pub fn GTK_NOTEBOOK(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkNotebook {
    unsafe{ ffi::cast_GtkNotebook(widget) }
}

pub fn GTK_STACK(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkStack {
    unsafe{ ffi::cast_GtkStack(widget) }
}

pub fn GTK_STACK_SWITCHER(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkStackSwitcher {
    unsafe{ ffi::cast_GtkStackSwitcher(widget) }
}

pub fn GTK_REVEALER(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkRevealer {
    unsafe{ ffi::cast_GtkRevealer(widget) }
}

pub fn GTK_OVERLAY(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkOverlay {
    unsafe{ ffi::cast_GtkOverlay(widget) }
}

pub fn GTK_SCROLLABLE(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkScrollable {
    unsafe{ ffi::cast_GtkScrollable(widget) }
}

pub fn GTK_LAYOUT(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkLayout {
    unsafe{ ffi::cast_GtkLayout(widget) }
}

pub fn GTK_HEADER_BAR(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkHeaderBar {
    unsafe{ ffi::cast_GtkHeaderBar(widget) }
}

pub fn GTK_FLOW_BOX(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkFlowBox {
    unsafe{ ffi::cast_GtkFlowBox(widget) }
}

pub fn GTK_FLOW_BOX_CHILD(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkFlowBoxChild {
    unsafe{ ffi::cast_GtkFlowBoxChild(widget) }
}

pub fn GTK_LIST_BOX(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkListBox {
    unsafe{ ffi::cast_GtkListBox(widget) }
}

pub fn GTK_LIST_BOX_ROW(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkListBoxRow {
    unsafe{ ffi::cast_GtkListBoxRow(widget) }
}

pub fn GTK_ACTION_BAR(widget: *ffi::C_GtkWidget) -> *ffi::C_GtkActionBar {
    unsafe{ ffi::cast_GtkActionBar(widget) }
}
