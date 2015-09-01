// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(non_snake_case)]

use ffi;

pub fn G_OBJECT(widget: *mut ffi::GtkWidget) -> *mut ::gobject_ffi::GObject {
    unsafe { ffi::cast_GtkObject(widget) }
}

pub fn G_OBJECT_FROM_LIST_STORE(widget: *mut ffi::GtkListStore) -> *mut ::gobject_ffi::GObject {
    unsafe { ffi::cast_GtkObjectFromListStore(widget) }
}

pub fn G_OBJECT_FROM_TREE_STORE(widget: *mut ffi::GtkTreeStore) -> *mut ::gobject_ffi::GObject {
    unsafe { ffi::cast_GtkObjectFromTreeStore(widget) }
}

pub fn G_OBJECT_FROM_TREE_VIEW_COLUMN(widget: *mut ffi::GtkTreeViewColumn) -> *mut ::gobject_ffi::GObject {
    unsafe { ffi::cast_GtkObjectFromTreeViewColumn(widget) }
}

pub fn G_OBJECT_FROM_TREE_SELECTION(widget: *mut ffi::GtkTreeSelection) -> *mut ::gobject_ffi::GObject {
    unsafe { ffi::cast_GtkObjectFromTreeSelection(widget) }
}

pub fn GTK_WIDGET(widget: *mut ::gobject_ffi::GObject) -> *mut ffi::GtkWidget {
    unsafe { ffi::cast_GtkWidget(widget) }
}

pub fn GTK_WINDOW(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkWindow {
    unsafe { ffi::cast_GtkWindow(widget) }
}

pub fn GTK_BIN(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkBin {
    unsafe { ffi::cast_GtkBin(widget) }
}

pub fn GTK_BUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkButton {
    unsafe { ffi::cast_GtkButton(widget) }
}

pub fn GTK_CONTAINER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkContainer {
    unsafe { ffi::cast_GtkContainer(widget) }
}

pub fn GTK_FRAME(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkFrame {
    unsafe { ffi::cast_GtkFrame(widget) }
}

pub fn GTK_LABEL(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkLabel {
    unsafe { ffi::cast_GtkLabel(widget) }
}

pub fn GTK_MISC(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkMisc {
    unsafe { ffi::cast_GtkMisc(widget) }
}

pub fn GTK_ORIENTABLE(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkOrientable {
    unsafe { ffi::cast_GtkOrientable(widget) }
}

pub fn GTK_RANGE(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkRange {
    unsafe { ffi::cast_GtkRange(widget) }
}

pub fn GTK_BOX(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkBox {
    unsafe { ffi::cast_GtkBox(widget) }

}

pub fn GTK_FIXED(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkFixed {
    unsafe { ffi::cast_GtkFixed(widget) }

}

pub fn GTK_BUTTONBOX(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkButtonBox {
    unsafe { ffi::cast_GtkButtonBox(widget) }

}

pub fn GTK_ASPECTFRAME(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkAspectFrame {
    unsafe { ffi::cast_GtkAspectFrame(widget) }

}

pub fn GTK_FONTBUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkFontButton {
    unsafe { ffi::cast_GtkFontButton(widget) }
}

pub fn GTK_TOGGLEBUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkToggleButton {
    unsafe { ffi::cast_GtkToggleButton(widget) }
}

pub fn GTK_MENUBUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkMenuButton {
    unsafe { ffi::cast_GtkMenuButton(widget) }
}

pub fn GTK_COLORBUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkColorButton {
    unsafe { ffi::cast_GtkColorButton(widget) }
}

pub fn GTK_LINKBUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkLinkButton {
    unsafe { ffi::cast_GtkLinkButton(widget) }
}

pub fn GTK_SCALEBUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkScaleButton {
    unsafe { ffi::cast_GtkScaleButton(widget) }
}

pub fn GTK_GRID(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkGrid {
    unsafe { ffi::cast_GtkGrid(widget) }
}

pub fn GTK_ENTRY(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkEntry {
    unsafe { ffi::cast_GtkEntry(widget) }
}

pub fn GTK_SWITCH(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkSwitch {
    unsafe { ffi::cast_GtkSwitch(widget) }
}

pub fn GTK_SCALE(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkScale {
    unsafe { ffi::cast_GtkScale(widget) }
}

pub fn GTK_LEVELBAR(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkLevelBar {
    unsafe { ffi::cast_GtkLevelBar(widget) }
}

#[cfg(gtk_3_10)]
pub fn GTK_SEARCHBAR(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkSearchBar {
    unsafe { ffi::cast_GtkSearchBar(widget) }
}

pub fn GTK_SPINBUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkSpinButton {
    unsafe { ffi::cast_GtkSpinButton(widget) }
}

pub fn GTK_SPINNER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkSpinner {
    unsafe { ffi::cast_GtkSpinner(widget) }
}

pub fn GTK_PROGRESSBAR(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkProgressBar {
    unsafe { ffi::cast_GtkProgressBar(widget) }
}

pub fn GTK_ARROW(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkArrow {
    unsafe { ffi::cast_GtkArrow(widget) }
}

pub fn GTK_CALENDAR(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkCalendar {
    unsafe { ffi::cast_GtkCalendar(widget) }
}

pub fn GTK_ALIGNMENT(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkAlignment {
    unsafe { ffi::cast_GtkAlignment(widget) }
}

pub fn GTK_EXPANDER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkExpander {
    unsafe { ffi::cast_GtkExpander(widget) }
}

pub fn GTK_PANED(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkPaned {
    unsafe { ffi::cast_GtkPaned(widget) }
}

pub fn GTK_INFOBAR(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkInfoBar {
    unsafe { ffi::cast_GtkInfoBar(widget) }
}

pub fn GTK_TOOLSHELL(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkToolShell {
    unsafe { ffi::cast_GtkToolShell(widget) }
}

pub fn GTK_TOOLBAR(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkToolbar {
    unsafe { ffi::cast_GtkToolbar(widget) }
}

pub fn GTK_TOOLITEM(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkToolItem {
    unsafe { ffi::cast_GtkToolItem(widget) }
}

pub fn GTK_SEPARATORTOOLITEM(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkSeparatorToolItem {
    unsafe { ffi::cast_GtkSeparatorToolItem(widget) }
}

pub fn GTK_TOOLBUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkToolButton {
    unsafe { ffi::cast_GtkToolButton(widget) }
}

pub fn GTK_MENUTOOLBUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkMenuToolButton {
    unsafe { ffi::cast_GtkMenuToolButton(widget) }
}

pub fn GTK_TOGGLETOOLBUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkToggleToolButton {
    unsafe { ffi::cast_GtkToggleToolButton(widget) }
}

pub fn GTK_RADIOTOOLBUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkRadioToolButton {
    unsafe { ffi::cast_GtkRadioToolButton(widget) }
}

pub fn GTK_ADJUSTMENT(obj: *mut ::gobject_ffi::GObject) -> *mut ffi::GtkAdjustment {
    unsafe { ffi::cast_GtkAdjustment(obj) }
}

pub fn GTK_DIALOG(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkDialog {
    unsafe { ffi::cast_GtkDialog(widget) }
}

pub fn GTK_ABOUT_DIALOG(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkAboutDialog {
    unsafe { ffi::cast_GtkAboutDialog(widget) }
}

pub fn GTK_MESSAGE_DIALOG(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkMessageDialog {
    unsafe { ffi::cast_GtkMessageDialog(widget) }
}

pub fn GTK_COLOR_CHOOSER_DIALOG(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkColorChooserDialog {
    unsafe { ffi::cast_GtkColorChooserDialog(widget) }
}

pub fn GTK_COLOR_CHOOSER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkColorChooser {
    unsafe { ffi::cast_GtkColorChooser(widget) }
}

pub fn GTK_NOTEBOOK(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkNotebook {
    unsafe { ffi::cast_GtkNotebook(widget) }
}

#[cfg(gtk_3_10)]
pub fn GTK_STACK(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkStack {
    unsafe { ffi::cast_GtkStack(widget) }
}

#[cfg(gtk_3_10)]
pub fn GTK_STACK_SWITCHER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkStackSwitcher {
    unsafe { ffi::cast_GtkStackSwitcher(widget) }
}

#[cfg(gtk_3_10)]
pub fn GTK_REVEALER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkRevealer {
    unsafe { ffi::cast_GtkRevealer(widget) }
}

pub fn GTK_OVERLAY(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkOverlay {
    unsafe { ffi::cast_GtkOverlay(widget) }
}

pub fn GTK_SCROLLABLE(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkScrollable {
    unsafe { ffi::cast_GtkScrollable(widget) }
}

pub fn GTK_LAYOUT(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkLayout {
    unsafe { ffi::cast_GtkLayout(widget) }
}

#[cfg(gtk_3_10)]
pub fn GTK_HEADER_BAR(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkHeaderBar {
    unsafe { ffi::cast_GtkHeaderBar(widget) }
}

#[cfg(gtk_3_12)]
pub fn GTK_FLOW_BOX(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkFlowBox {
    unsafe { ffi::cast_GtkFlowBox(widget) }
}

#[cfg(gtk_3_12)]
pub fn GTK_FLOW_BOX_CHILD(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkFlowBoxChild {
    unsafe { ffi::cast_GtkFlowBoxChild(widget) }
}

#[cfg(gtk_3_10)]
pub fn GTK_LIST_BOX(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkListBox {
    unsafe { ffi::cast_GtkListBox(widget) }
}

#[cfg(gtk_3_10)]
pub fn GTK_LIST_BOX_ROW(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkListBoxRow {
    unsafe { ffi::cast_GtkListBoxRow(widget) }
}

#[cfg(gtk_3_12)]
pub fn GTK_ACTION_BAR(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkActionBar {
    unsafe { ffi::cast_GtkActionBar(widget) }
}

pub fn GTK_FILE_CHOOSER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkFileChooser {
    unsafe { ffi::cast_GtkFileChooser(widget) }
}

pub fn GTK_FILE_FILTER(obj: *mut ::gobject_ffi::GObject) -> *mut ffi::GtkFileFilter {
    unsafe { ffi::cast_GtkFileFilter(obj) }
}

pub fn GTK_APP_CHOOSER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkAppChooser {
    unsafe { ffi::cast_GtkAppChooser(widget) }
}

pub fn GTK_APP_CHOOSER_WIDGET(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkAppChooserWidget {
    unsafe { ffi::cast_GtkAppChooserWidget(widget) }
}

pub fn GTK_APP_CHOOSER_DIALOG(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkAppChooserDialog {
    unsafe { ffi::cast_GtkAppChooserDialog(widget) }
}

pub fn GTK_FONT_CHOOSER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkFontChooser {
    unsafe { ffi::cast_GtkFontChooser(widget) }
}

pub fn GTK_FONT_CHOOSER_DIALOG(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkFontChooserDialog {
    unsafe { ffi::cast_GtkFontChooserDialog(widget) }
}

pub fn GTK_APP_INFO(widget: *mut ffi::GtkWidget) -> *mut ::gio_ffi::GAppInfo {
    unsafe { ffi::cast_GtkAppInfo(widget) }
}

pub fn GTK_APP_LAUNCH_CONTEXT(widget: *mut ffi::GtkWidget) -> *mut ::gio_ffi::GAppLaunchContext {
    unsafe { ffi::cast_GtkAppLaunchContext(widget) }
}

pub fn GTK_PAGE_SETUP(widget: *mut ::gobject_ffi::GObject) -> *mut ffi::GtkPageSetup {
    unsafe { ffi::cast_GtkPageSetup(widget) }
}

pub fn GTK_PAPER_SIZE(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkPaperSize {
    unsafe { ffi::cast_GtkPaperSize(widget) }
}

/*pub fn GTK_PAGE_SETUP_UNIX_DIALOG(widget: *mut ffi::GtkWidget) -> *mut ffi::PageSetupUnixDialog {
    unsafe{ ffi::cast_PageSetupUnixDialog(widget) }
}*/

pub fn GTK_PRINT_SETTINGS(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkPrintSettings {
    unsafe { ffi::cast_GtkPrintSettings(widget) }
}

pub fn GTK_RECENT_CHOOSER_DIALOG(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkRecentChooserDialog {
    unsafe { ffi::cast_GtkRecentChooserDialog(widget) }
}

pub fn GTK_RECENT_MANAGER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkRecentManager {
    unsafe { ffi::cast_GtkRecentManager(widget) }
}

pub fn GTK_RECENT_CHOOSER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkRecentChooser {
    unsafe { ffi::cast_GtkRecentChooser(widget) }
}

pub fn GTK_RECENT_FILTER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkRecentFilter {
    unsafe { ffi::cast_GtkRecentFilter(widget) }
}

pub fn GTK_RECENT_INFO(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkRecentInfo {
    unsafe { ffi::cast_GtkRecentInfo(widget) }
}

pub fn GTK_EDITABLE(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkEditable {
    unsafe { ffi::cast_GtkEditable(widget) }
}

pub fn GTK_TEXT_VIEW(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkTextView {
    unsafe { ffi::cast_GtkTextView(widget) }
}

pub fn GTK_TEXT_BUFFER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkTextBuffer {
    unsafe { ffi::cast_GtkTextBuffer(widget) }
}

pub fn GTK_TEXT_TAG_TABLE(widget: *mut ::gobject_ffi::GObject) -> *mut ffi::GtkTextTagTable {
    unsafe { ffi::cast_GtkTextTagTable(widget) }
}

pub fn GTK_SCROLLED_WINDOW(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkScrolledWindow {
    unsafe { ffi::cast_GtkScrolledWindow(widget) }
}

pub fn GTK_RADIOBUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkRadioButton {
    unsafe { ffi::cast_GtkRadioButton(widget) }
}

pub fn GTK_CELL_RENDERER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkCellRenderer {
    unsafe { ffi::cast_GtkCellRenderer(widget) }
}

pub fn GTK_TREE_VIEW(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkTreeView {
    unsafe { ffi::cast_GtkTreeView(widget) }
}

pub fn GTK_MENU_SHELL(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkMenuShell {
    unsafe { ffi::cast_GtkMenuShell(widget) }
}

pub fn GTK_MENU_ITEM(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkMenuItem {
    unsafe { ffi::cast_GtkMenuItem(widget) }
}

pub fn GTK_CHECK_MENU_ITEM(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkCheckMenuItem {
    unsafe { ffi::cast_GtkCheckMenuItem(widget) }
}

pub fn GTK_VIEWPORT(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkViewport {
    unsafe { ffi::cast_GtkViewport(widget) }
}

pub fn GTK_STATUSBAR(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkStatusbar {
    unsafe { ffi::cast_GtkStatusbar(widget) }
}

pub fn GTK_CELL_EDITABLE(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkCellEditable {
    unsafe { ffi::cast_GtkCellEditable(widget) }
}

pub fn GTK_CELL_RENDERER_TEXT(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkCellRendererText {
    unsafe { ffi::cast_GtkCellRendererText(widget) }
}

pub fn GTK_LOCKBUTTON(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkLockButton {
    unsafe { ffi::cast_GtkLockButton(widget) }
}

pub fn GTK_ACTIONABLE(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkActionable {
    unsafe { ffi::cast_GtkActionable(widget) }
}

pub fn GTK_CELL_LAYOUT(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkCellLayout {
    unsafe { ffi::cast_GtkCellLayout(widget) }
}

pub fn GTK_ENTRY_COMPLETION(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkEntryCompletion {
    unsafe { ffi::cast_GtkEntryCompletion(widget) }
}

pub fn GTK_ICON_VIEW(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkIconView {
    unsafe { ffi::cast_GtkIconView(widget) }
}

pub fn GTK_TREE_SELECTION(widget: *mut ::gobject_ffi::GObject) -> *mut ffi::GtkTreeSelection {
    unsafe { ffi::cast_GtkTreeSelection(widget) }
}

pub fn GTK_IMAGE(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkImage {
    unsafe { ffi::cast_GtkImage(widget) }
}

pub fn GTK_RECENT_CHOOSER_WIDGET(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkRecentChooserWidget {
    unsafe { ffi::cast_GtkRecentChooserWidget(widget) }
}

pub fn GTK_TREE_MODEL_FROM_LIST_STORE(store: *mut ffi::GtkListStore) -> *mut ffi::GtkTreeModel {
    unsafe { ffi::cast_GtkTreeModelFromListStore(store) }
}

pub fn GTK_LIST_STORE_FROM_TREE_MODEL(model: *mut ffi::GtkTreeModel) -> *mut ffi::GtkListStore {
    unsafe { ffi::cast_GtkListStoreFromTreeModel(model) }
}

pub fn GTK_LIST_STORE(model: *mut ::gobject_ffi::GObject) -> *mut ffi::GtkListStore {
    unsafe { ffi::cast_GtkListStore(model) }
}

pub fn GTK_TREE_MODEL_FROM_TREE_STORE(store: *mut ffi::GtkTreeStore) -> *mut ffi::GtkTreeModel {
    unsafe { ffi::cast_GtkTreeModelFromTreeStore(store) }
}

pub fn GTK_TREE_STORE_FROM_TREE_MODEL(model: *mut ffi::GtkTreeModel) -> *mut ffi::GtkTreeStore {
    unsafe { ffi::cast_GtkTreeStoreFromTreeModel(model) }
}

pub fn GTK_TREE_STORE(obj: *mut ::gobject_ffi::GObject) -> *mut ffi::GtkTreeStore {
    unsafe { ffi::cast_GtkTreeStore(obj) }
}

pub fn GTK_TREE_MODEL(obj: *mut ::gobject_ffi::GObject) -> *mut ffi::GtkTreeModel {
    unsafe { ffi::cast_GtkTreeModel(obj) }
}

pub fn GTK_COMBO_BOX(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkComboBox {
    unsafe { ffi::cast_GtkComboBox(widget) }
}

#[cfg(gtk_3_12)]
pub fn GTK_POPOVER(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkPopover {
    unsafe { ffi::cast_GtkPopover(widget) }
}

pub fn GTK_COMBO_BOX_TEXT(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkComboBoxText {
    unsafe { ffi::cast_GtkComboBoxText(widget) }
}

pub fn GTK_ENTRY_BUFFER(obj: *mut ::gobject_ffi::GObject) -> *mut ffi::GtkEntryBuffer {
    unsafe { ffi::cast_GtkEntryBuffer(obj) }
}

pub fn GTK_TEXT_MARK(widget: *mut ::gobject_ffi::GObject) -> *mut ffi::GtkTextMark {
    unsafe { ffi::cast_GtkTextMark(widget) }
}

#[cfg(gtk_3_10)]
pub fn GTK_PLACES_SIDEBAR(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkPlacesSidebar {
    unsafe { ffi::cast_GtkPlacesSidebar(widget) }
}

pub fn GTK_TOOL_PALETTE(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkToolPalette {
    unsafe { ffi::cast_GtkToolPalette(widget) }
}

pub fn GTK_TOOL_ITEM(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkToolItem {
    unsafe { ffi::cast_GtkToolItem(widget) }
}

pub fn GTK_TOOL_ITEM_GROUP(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkToolItemGroup {
    unsafe { ffi::cast_GtkToolItemGroup(widget) }
}

pub fn GTK_FILE_CHOOSER_WIDGET(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkFileChooserWidget {
    unsafe { ffi::cast_GtkFileChooserWidget(widget) }
}

pub fn GTK_COLOR_CHOOSER_WIDGET(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkColorChooserWidget {
    unsafe { ffi::cast_GtkColorChooserWidget(widget) }
}

pub fn GTK_FONT_CHOOSER_WIDGET(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkFontChooserWidget {
    unsafe { ffi::cast_GtkFontChooserWidget(widget) }
}

#[cfg(target_os = "linux")]
pub fn GTK_SOCKET(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkSocket {
    unsafe { ffi::cast_GtkSocket(widget) }
}

pub fn GTK_EVENT_BOX(widget: *mut ffi::GtkWidget) -> *mut ffi::GtkEventBox {
    unsafe { ffi::cast_GtkEventBox(widget) }
}
