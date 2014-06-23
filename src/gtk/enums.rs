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

//! Enumeration used with widgets

/// A gtk::Window can be one of these types. Most things you'd consider a "window"
/// should have type WindowTopLevel; windows with this type are managed by the window manager
/// and have a frame by default (call gtk::window::set_decorated() to toggle the frame).
///
/// Windows with type WindowPopUp are ignored by the window manager; window manager keybindings won't work on them,
/// the window manager won't decorate the window with a frame, many GTK+ features that rely on the window manager will not work
/// (e.g. resize grips and maximization/minimization).
///
/// GGtkWindowPopUp is used to implement widgets such as gtk::Menu
/// or tooltips that you normally don't think of as windows per se. Nearly all windows should be WindowTopLevel.
/// In particular, do not use WindowPopUp just to turn off the window borders; use gtk_window_set_decorated() for that.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum WindowType {
    /// A regular window, such as a dialog.
    WindowTopLevel,
    /// A special window such as a tooltip.
    WindowPopUp
}

/// Reading directions for text
pub enum TextDirection{
    TextDirNone,
    TextDirLtr,
    TextDirRtl
}

/// Window placement can be influenced using this enumeration.
/// Note that using WinPosCenterAlways is almost always a bad idea.
/// It won't necessarily work well with all window managers or on all windowing systems.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum WindowPosition{
    /// No influence is made on placement.
    WinPosNone,
    /// Windows should be placed in the center of the screen.
    WinPosCenter,
    /// Windows should be placed at the current mouse position.
    WinPosMouse,
    /// Keep window centered as it changes size, etc.
    WinPosCenterAlways,
    /// Center the window on its transient parent (see window.set_transient_for()).
    WinPosCenterOnParent
}

/// Used to dictate the style that a gtk::ButtonBox uses to layout the buttons it contains.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum ButtonBoxStyle {
    /// Buttons are evenly spread across the box.
    ButtonBoxSpread = 1,
    /// Buttons are placed at the edges of the box.
    ButtonBoxEdge,
    /// Buttons are grouped towards the start of the box, (on the left for a HBox, or the top for a VBox).
    ButtonBoxStart,
    /// Buttons are grouped towards the end of the box, (on the right for a HBox, or the bottom for a VBox).
    ButtonBoxEnd,
    /// Buttons are centered in the box. Since 2.12.
    ButtonBoxCenter
}

/// Represents the orientation of widgets which can be switched between
/// horizontal and vertical orientation on the fly, like gtk::Toolbar.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum Orientation {
    /// The widget is in horizontal orientation.
    OrientationHorizontal,
    /// The widget is in vertical orientation.
    OrientationVertical
}

/// Availables direction types
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum  DirectionType {
    DirTabForward,
    DirTabBackward,
    DirUp,
    DirDown,
    DirLeft,
    DirRight
}

/// Specifies which corner a child widget should be placed in when packed into a gtk::ScrolledWindow.
/// This is effectively the opposite of where the scroll bars are placed.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum  CornerType {
    /// Place the scrollbars on the right and bottom of the widget (default behaviour).
    CornerTopLeft,
    /// Place the scrollbars on the top and right of the widget.
    CornerBottomLeft,
    /// Place the scrollbars on the left and bottom of the widget.
    CornerTopRight,
    /// Place the scrollbars on the top and left of the widget.
    CornerBottomRight
}

/// Availables resize modes
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum ResizeMode {
    /// Pass resize request to the parent
    ResizeParent,
    /// Queue resizes on this widget
    ResizeQueue,
    /// Resize immediately. Deprecated.
    OrientationtkResizeImmediate
}

/// Describes how the border of a UI element should be rendered.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum BorderStyle {
    /// No visible border
    BorderStyleNone,
    /// A single line segment
    BorderStyleSolid,
    /// Looks as if the content is sunken into the canvas
    BorderStyleInset,
    /// Looks as if the content is coming out of the canvas
    BorderStyleOutset,
    /// Same as BorderStyleNone
    BorderStyleHidden,
    /// A series of round dots
    BorderStyleDotted,
    /// A series of square-ended dashes
    BorderStyleDashed,
    /// Two parallel lines with some space between them
    BorderStyleDouble,
    /// Looks as if it were carved in the canvas
    BorderStyleGroove,
    /// Looks as if it were coming out of the canvas
    BorderStyleRidge
}

/// Determines the direction of a sort.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum SortType {
    /// Sorting is in ascending order
    SortAscending,
    /// Sorting is in descending order.
    SortDescending
}

/// Describes a widget state. Widget states are used to match the widget against CSS pseudo-classes.
/// Note that GTK extends the regular CSS classes and sometimes uses different names.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum StateFlags {
    /// State during normal operation.
    StateFlagNormal       = 0,
    /// Widget is active.
    StateFlagActive       = 1 << 0,
    /// Widget has a mouse pointer over it.
    StateFlagPrelight     = 1 << 1,
    /// Widget is selected.
    StateFlagSelected     = 1 << 2,
    /// Widget is insensitive.
    StateFlagInsensitive  = 1 << 3,
    /// Widget is inconsistent.
    StateFlagInconsistent = 1 << 4,
    /// Widget has the keyboard focus.
    StateFlagFocused      = 1 << 5,
    /// Widget is in a background toplevel window.
    StateFlagBackDrop     = 1 << 6,
    /// Widget is in left-to-right text direction. Since 3.8
    StateFlagDirLTR       = 1 << 7,
    /// Widget is in right-to-left text direction. Since 3.8
    StateFlagDirRTL       = 1 << 8
}

/// Gives an indication why a drag operation failed.
/// The value can by obtained by connecting to the "drag-failed" signal.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum DragResult {
    /// The drag operation was successful.
    DragResultSuccess,
    /// No suitable drag target.
    DragResultNoTarget,
    /// The user cancelled the drag operation.
    DragResultUserCanceled,
    /// The drag operation timed out.
    DragResultTimeoutExpired,
    /// The pointer or keyboard grab used for the drag operation was broken.
    DragResultGrabBroken,
    /// The drag operation failed due to some unspecified error.
    DragResultError
}

/// Availables accel flags
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum AccelFlags {
    /// display in AccelLabel?
    AccelVisible        = 1 << 0,
    /// is it removable?
    AccelLocked         = 1 << 1,
    /// Comparison mask
    AccelMask           = 0x07
}

/// Used to specify the placement of scroll arrows in scrolling menus.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum ArrowPlacement {
    /// Place one arrow on each end of the menu.
    ArrowsBoth,
    /// Place both arrows at the top of the menu.
    ArrowsStart,
    /// Place both arrows at the bottom of the menu.
    ArrowsEnd
}

/// Used to indicate the direction in which a Arrow should point.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum ArrowType {
    /// Represents an upward pointing arrow.
    ArrowUp,
    /// Represents a downward pointing arrow.
    ArrowDown,
    /// Represents a left pointing arrow.
    ArrowLeft,
    /// Represents a right pointing arrow.
    ArrowRight,
    /// No arrow. Since 2.10.
    ArrowNone
}

/// Denotes the expansion properties that a widget will have when it (or its parent) is resized.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum AttachOptions {
    /// the widget should expand to take up any extra space in its container that has been allocated.
    Expand = 1 << 0,
    /// the widget should shrink as and when possible.
    Shrink = 1 << 1,
    /// the widget should fill the space allocated to it.
    Fill   = 1 << 2
}

/// Deleting modes
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum DeleteType {
    /// delete chars
    DeleteChars,
    /// delete only the portion of the word to the left/right of cursor if we're in the middle of a word
    DeleteWordsEnd,
    /// delete words
    DeleteWords,
    /// delete lines
    DeleteDisplayLines,
    /// deletes lines end
    DeleteDisplayLineEnd,
    /// like C-k in Emacs (or its reverse)
    DeleteParagraphEnds,
    /// C-k in pico, kill whole line
    DeleteParagraphs,
    /// M-\ in Emacs
    DeleteWhitespac
}

/// Used to specify the style of the expanders drawn by a TreeView.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum ExpanderStyle {
    /// The style used for a collapsed subtree.
    ExpanderCollapsed,
    /// Intermediate style used during animation.
    ExpanderSemiCollapsed,
    /// Intermediate style used during animation.
    ExpanderSemiExpanded,
    /// The style used for an expanded subtree.
    GGtkExpanderExpanded
}

/// preedit style
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum IMPreeditStyle {
    ImPreeditNothing,
    ImPreeditCallback,
    ImPreeditNone
}

/// Status styles
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum IMStatusStyle {
    ImStatusNothing,
    ImStatusCallback,
    ImStatusNone
}

/// Used for justifying the text inside a Label widget. (See also Alignment).
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum Justification {
    /// The text is placed at the left edge of the label.
    JustifyLeft,
    /// The text is placed at the right edge of the label.
    JustifyRight,
    /// The text is placed in the center of the label.
    JustifyCenter,
    /// The text is placed is distributed across the label.
    JustifyFill
}

/// Availables movement steps
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum MovementStep {
    /// Move forward or back by graphemes
    MovementLogicalPosition,
    /// Move left or right by graphemes
    MovementVisualPositions,
    /// Move forward or back by words
    MovementWords,
    /// Move up or down lines (wrapped lines)
    MovementDisplayLines,
    /// Move to either end of a line
    MovementDisplayLineEnds,
    /// Move up or down paragraphs (newline-ended lines)
    MovementParagraphs,
    /// Move to either end of a paragraph
    MovementParagraphEnds,
    /// Move by pages
    MovementPages,
    /// Move to ends of the buffer
    MovementBufferEnds,
    /// Move horizontally by pages
    MovementHorizontalPages
}

/// Represents the packing location Box children. (See: VBox, HBox, and ButtonBox).
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum PackType {
    /// The child is packed into the start of the box
    PackStart,
    /// The child is packed into the end of the box
    PackEnd
}


/// Availables Gtk path priority
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum PathPriorityType {
    PathPrioLowest       = 0,
    PathPrioGtk          = 4,
    PathPrioApplication  = 8,
    PathPrioTheme        = 10,
    PathPrioRc           = 12,
    PathPrioHighest      = 15
}

/// Availables Gtk path types
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum PathType {
    PathWidget,
    PathWidgetClass,
    PathClass
}

/// Determines when a scroll bar will be visible.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum PolicyType {
    /// The scrollbar is always visible.
    PolicyAlways,
    /// The scrollbar will appear and disappear as necessary. For example, when all of a CList can not be seen.
    PolicyAutomatic,
    /// The scrollbar will never appear.
    PolicyNever
}

/// Describes which edge of a widget a certain feature is positioned at, e.g. the tabs of a Notebook,
/// the handle of a HandleBox or the label of a Scale.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum PositionType {
    /// The feature is at the left edge.
    PosLeft,
    /// The feature is at the right edge.
    PosRight,
    /// The feature is at the top edge.
    PosTop,
    /// The feature is at the bottom edge.
    PosBottom
}

/// Indicated the relief to be drawn around a Button.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum ReliefStyle {
    /// Draw a normal relief.
    GTkReliefNormal,
    /// A half relief.
    GTkReliefHalf,
    /// No relief.
    GTkReliefNone
}

/// Available scroll steps
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum ScrollStep {
  ScrollSteps,
  ScrollPages,
  ScrollEnds,
  ScrollHorizontalSteps,
  ScrollHorizontalPages,
  ScrollHorizontalEnds
}

/// Available scroll types
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum ScrollType {
  ScrollNone,
  ScrollJump,
  ScrollStepBackward,
  ScrollStepForward,
  ScrollPageBackward,
  ScrollPageForward,
  ScrollStepUp,
  ScrollStepDown,
  ScrollPageUp,
  ScrollPageDown,
  ScrollStepLeft,
  ScrollStepRight,
  ScrollPageLeft,
  ScrollPageRight,
  ScrollStart,
  ScrollEnd
}

/// Used to control what selections users are allowed to make.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum SelectionMode {
    /// No selection is possible.
    SelectionNone,
    /// Zero or one element may be selected.
    SelectionSingle,
    /// Exactly one element is selected.
    SelectionBrowse,
    /// Any number of elements may be selected.
    SelectionMultiple
}

/// Used to change the appearance of an outline typically provided by a Frame
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum ShadowType {
    /// No outline.
    ShadowNone,
    /// The outline is bevelled inwards.
    ShadowIm,
    /// The outline is bevelled outwards like a button.
    ShadowOut,
    /// The outline has a sunken 3d appearance.
    ShadowEtchedIn,
    /// The outline has a raised 3d appearance.
    ShadowEtchedOut
}

/// This type indicates the current state of a widget; the state determines how the widget is drawn.
///
/// The StateType enumeration is also used to identify different colors in a Style for drawing,
/// so states can be used for subparts of a widget as well as entire widgets.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum StateType {
    /// State during normal operation.
    StateNormal,
    /// State of a currently active widget, such as a depressed button.
    StateActive,
    /// State indicating that the mouse pointer is over the widget and the widget will respond to mouse clicks.
    StatePrelight,
    /// State of a selected item, such the selected row in a list.
    StateSelected,
    /// State indicating that the widget is unresponsive to user actions.
    StateInsensitive,
    /// The widget is inconsistent, such as checkbuttons or radiobuttons that aren't either set to true nor false,
    /// or buttons requiring the user attention.
    StateInconsistent,
    /// The widget has the keyboard focus
    StateFocused
}

/// Used to customize the appearance of a Toolbar.
///
/// Note that setting the toolbar style overrides the user's preferences for the default toolbar style.
/// Note that if the button has only a label set and GTK_TOOLBAR_ICONS is used, the label will be visible, and vice versa.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum ToolbarStyle {
    /// Buttons display only icons in the toolbar.
    ToolbarIcons,
    /// Buttons display only text labels in the toolbar.
    ToolbarText,
    /// Buttons display text and icons in the toolbar.
    ToolbarBoth,
    /// Buttons display icons and text alongside each other, rather than vertically stacked
    ToolbarBothHoriz
}

/// Describes how a rendered element connects to adjacent elements.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum JunctionSides {
    /// No junctions.
    JunctionNone               = 0,
    /// Element connects on the top-left corner.
    JunctionCornerTopleft      = 1 << 0,
    /// Element connects on the top-right corner.
    JunctionCornerTopRight     = 1 << 1,
    /// Element connects on the bottom-left corner.
    JunctionCornerBottomLeft   = 1 << 2,
    /// Element connects on the bottom-right corner.
    JunctionCornerBottomRight  = 1 << 3,
    /// Element connects on the top side.
    JunctionTop                = (1 << 0 | 1 << 1),
    /// Element connects on the bottom side.
    JunctionBottom             = (1 << 2 | 1 << 3),
    /// Element connects on the left side.
    JunctionLeft               = (1 << 0 | 1 << 2),
    /// Element connects on the right side.
    JunctionRight              = (1 << 1 | 1 << 3)
}

/// Describes a region within a widget.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum RegionFlags {
    /// Region has an even number within a set.
    RegionEven    = 1 << 0,
    /// Region has an odd number within a set.
    RegionOdd     = 1 << 1,
    /// Region is the first one within a set.
    RegionFirst   = 1 << 2,
    /// Region is the last one within a set.
    RegionLast    = 1 << 3,
    /// Region is the only one within a set.
    RegionOnly    = 1 << 4,
    /// Region is part of a sorted area.
    RegionSorted  = 1 << 5
}

/// Icon Sizes : temporary
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum IconSize {
  IconSizeInvalid,
  IconSizeMemu,
  IconSizeSmallToolbar,
  IconSizeLargeToolbar,
  IconSizeButton,
  IconSizeDnd,
  IconSizeDisalog
}

/// Specifies the side of the entry at which an icon is placed.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum EntryIconPosition {
    /// At the beginning of the entry (depending on the text direction).
    EntryIconPrimary,
    /// At the end of the entry (depending on the text direction).
    EntryIconSecondary
}

/// Describes hints that might be taken into account by input methods or applications.
/// Note that input methods may already tailor their behaviour according to the InputPurpose of the entry.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum InputHints {
    /// No special behaviour suggested
    InputHintNone                = 0,
    /// Suggest checking for typos
    InputHintSpellcheck          = 1 << 0,
    /// Suggest not checking for typos
    InputHintNoSpellcheck        = 1 << 1,
    /// Suggest word completion
    InputHintWordCompletion      = 1 << 2,
    /// Suggest to convert all text to lowercase
    InputHintLowercase           = 1 << 3,
    /// Suggest to capitalize all text
    InputHintUppercaseChars      = 1 << 4,
    /// Suggest to capitalize the first character of each word
    InputHintUppercaseWords      = 1 << 5,
    /// Suggest to capitalize the first word of each sentence
    InputHintUppercaseSentences  = 1 << 6,
    /// Suggest to not show an onscreen keyboard (e.g for a calculator that already has all the keys).
    InputHintInhibitOsk          = 1 << 7
}

/// Describes primary purpose of the input widget.
/// This information is useful for on-screen keyboards
/// and similar input methods to decide which keys should be presented to the user.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum InputPurpose {
    /// Allow any character
    InputPurposeFreeForm,
    /// Allow only alphabetic characters
    InputPurposeAlpha,
    /// Allow only digits
    InputPurposeDigits,
    /// Edited field expects numbers
    InputPurposeNumber,
    /// Edited field expects phone number
    InputPurposePhone,
    /// Edited field expects URL
    InputPurposeUrl,
    /// Edited field expects email address
    InputPurposeEmail,
    /// Edited field expects the name of a person
    InputPurposeName,
    /// Like InputPurposeFreeForm, but characters are hidden
    InputPurposePassword,
    /// Like InputPurposeDigits, but characters are hidden
    InputPurposePin
}

/// Describes the image data representation used by a Image.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum ImageType {
    /// there is no image displayed by the widget
    ImageEmpty,
    /// the widget contains a Gdk::Pixbuf
    ImagePixbuf,
    /// the widget contains a stock icon name (see Stock Items(3))
    ImageStock,
    /// the widget contains a Gtk::IconSet
    ImageIconSet,
    /// the widget contains a Gdk::PixbufAnimation
    ImageAnimation,
    /// the widget contains a named icon. This image type was added in GTK+ 2.6
    ImageIconName,
    /// the widget contains a GIcon. This image type was added in GTK+ 2.14
    ImageGIcon,
    /// the widget contains a cairo_surface_t. This image type was added in GTK+ 3.10
    ImageSurface
}

/// The values of the SpinType enumeration are used
/// to specify the change to make in gtk::SpinButton::spin().
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum SpinType {
    /// Increment by the adjustments step increment.
    SpinStepForward,
    /// Decrement by the adjustments step increment.
    SpinStepBackward,
    /// Increment by the adjustments page increment.
    SpinPageForward,
    /// Decrement by the adjustments page increment.
    SpinPageBackward,
    /// Go to the adjustments lower bound.
    SpinHome,
    /// Go to the adjustments upper bound.
    SpinEnd,
    /// Change by a specified amount.
    SpinUserDefined
}

/// The spin button update policy determines whether the spin button displays values
/// even if they are outside the bounds of its adjustment. See gtk::SpinButton::set_update_policy().
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum SpinButtonUpdatePolicy {
    /// When refreshing your Gtk::SpinButton, the value is always displayed
    UpdateAlways,
    /// When refreshing your Gtk::SpinButton, the value is only displayed
    /// if it is valid within the bounds of the spin button's adjustment
    UpdateIfValid
}

/// Describes how LevelBar contents should be rendered.
/// Note that this enumeration could be extended with additional modes in the future.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum LevelBarMode {
    /// the bar has a continuous mode
    LevelBarModeContinuous,
    /// the bar has a discrete mode
    LevelBarModeDiscrete
}

/// These options can be used to influence the display and behaviour of a gtk::Calendar.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum CalendarDisplayOptions {
    /// Specifies that the month and year should be displayed.
    CalendarShowHeading        = 1 << 0,
    /// Specifies that three letter day descriptions should be present.
    CalendarShowDayNames       = 1 << 1,
    /// Prevents the user from switching months with the calendar.
    CalendarNoMonthChange      = 1 << 2,
    /// Displays each week numbers of the current year, down the left side of the calendar.
    CalendarShowWeekNumbers    = 1 << 3,
    /// Just show an indicator, not the full details text when details are provided. See gtk::Calendar::set_detail_func().
    CalendarShowDetails        = 1 << 5
}

/// The type of message being displayed in the dialog.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum MessageType {
    /// Informational message
    MessageInfo,
    /// Non-fatal warning message
    MessageWarning,
    /// Question requiring a choice
    MessageQuestion,
    /// Fatal error message
    MessageError,
    /// None of the above, doesn't get an icon
    MessageOther
}
