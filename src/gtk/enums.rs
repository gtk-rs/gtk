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
/// should have type GtkWindowTopLevel; windows with this type are managed by the window manager
/// and have a frame by default (call gtk::window::set_decorated() to toggle the frame).
///
/// Windows with type GtkWindowPopUp are ignored by the window manager; window manager keybindings won't work on them,
/// the window manager won't decorate the window with a frame, many GTK+ features that rely on the window manager will not work
/// (e.g. resize grips and maximization/minimization).
///
/// GGtkWindowPopUp is used to implement widgets such as gtk::Menu
/// or tooltips that you normally don't think of as windows per se. Nearly all windows should be GtkWindowTopLevel.
/// In particular, do not use GtkWindowPopUp just to turn off the window borders; use gtk_window_set_decorated() for that.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkWindowType {
    /// A regular window, such as a dialog.
    GtkWindowTopLevel,
    /// A special window such as a tooltip.
    GtkWindowPopUp
}

/// Window placement can be influenced using this enumeration.
/// Note that using GtkWinPosCenterAlways is almost always a bad idea.
/// It won't necessarily work well with all window managers or on all windowing systems.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkWindowPosition{
    /// No influence is made on placement.
    GtkWinPosNone,
    /// Windows should be placed in the center of the screen.
    GtkWinPosCenter,
    /// Windows should be placed at the current mouse position.
    GtkWinPosMouse,
    /// Keep window centered as it changes size, etc.
    GtkWinPosCenterAlways,
    /// Center the window on its transient parent (see window.set_transient_for()).
    GtkWinPosCenterOnParent
}

/// Used to dictate the style that a gtk::ButtonBox uses to layout the buttons it contains.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkButtonBoxStyle {
    /// Buttons are evenly spread across the box.
    GtkButtonBoxSpread = 1,
    /// Buttons are placed at the edges of the box.
    GtkButtonBoxEdge,
    /// Buttons are grouped towards the start of the box, (on the left for a HBox, or the top for a VBox).
    GtkButtonBoxStart,
    /// Buttons are grouped towards the end of the box, (on the right for a HBox, or the bottom for a VBox).
    GtkButtonBoxEnd,
    /// Buttons are centered in the box. Since 2.12.
    GtkButtonBoxCenter
}

/// Represents the orientation of widgets which can be switched between
/// horizontal and vertical orientation on the fly, like gtk::Toolbar.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkOrientation {
    /// The widget is in horizontal orientation.
    GtkOrientationHorizontal,
    /// The widget is in vertical orientation.
    GtkOrientationVertical
}

/// Availables direction types
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum  GtkDirectionType {
    GtkDirTabForward,
    GtkDirTabBackward,
    GtkDirUp,
    GtkDirDown,
    GtkDirLeft,
    GtkDirRight
}

/// Specifies which corner a child widget should be placed in when packed into a gtk::ScrolledWindow.
/// This is effectively the opposite of where the scroll bars are placed.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum  GtkCornerType {
    /// Place the scrollbars on the right and bottom of the widget (default behaviour).
    GtkCornerTopLeft,
    /// Place the scrollbars on the top and right of the widget.
    GtkCornerBottomLeft,
    /// Place the scrollbars on the left and bottom of the widget.
    GtkCornerTopRight,
    /// Place the scrollbars on the top and left of the widget.
    GtkCornerBottomRight
}

/// Availables resize modes
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkResizeMode {
    /// Pass resize request to the parent
    GtkResizeParent,
    /// Queue resizes on this widget
    GtkResizeQueue,
    /// Resize immediately. Deprecated.
    GtkOrientationtkResizeImmediate
}

/// Describes how the border of a UI element should be rendered.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkBorderStyle {
    /// No visible border
    GtkBorderStyleNone,
    /// A single line segment
    GtkBorderStyleSolid,
    /// Looks as if the content is sunken into the canvas
    GtkBorderStyleInset,
    /// Looks as if the content is coming out of the canvas
    GtkBorderStyleOutset,
    /// Same as GtkBorderStyleNone
    GtkBorderStyleHidden,
    /// A series of round dots
    GtkBorderStyleDotted,
    /// A series of square-ended dashes
    GtkBorderStyleDashed,
    /// Two parallel lines with some space between them
    GtkBorderStyleDouble,
    /// Looks as if it were carved in the canvas
    GtkBorderStyleGroove,
    /// Looks as if it were coming out of the canvas
    GtkBorderStyleRidge
}

/// Determines the direction of a sort.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkSortType {
    /// Sorting is in ascending order
    GtkSortAscending,
    /// Sorting is in descending order.
    GtkSortDescending
}

/// Describes a widget state. Widget states are used to match the widget against CSS pseudo-classes.
/// Note that GTK extends the regular CSS classes and sometimes uses different names.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkStateFlags {
    /// State during normal operation.
    GtkStateFlagNormal       = 0,
    /// Widget is active.
    GtkStateFlagActive       = 1 << 0,
    /// Widget has a mouse pointer over it.
    GtkStateFlagPrelight     = 1 << 1,
    /// Widget is selected.
    GtkStateFlagSelected     = 1 << 2,
    /// Widget is insensitive.
    GtkStateFlagInsensitive  = 1 << 3,
    /// Widget is inconsistent.
    GtkStateFlagInconsistent = 1 << 4,
    /// Widget has the keyboard focus.
    GtkStateFlagFocused      = 1 << 5,
    /// Widget is in a background toplevel window.
    GtkStateFlagBackDrop     = 1 << 6,
    /// Widget is in left-to-right text direction. Since 3.8
    GtkStateFlagDirLTR       = 1 << 7,
    /// Widget is in right-to-left text direction. Since 3.8
    GtkStateFlagDirRTL       = 1 << 8
}

/// Gives an indication why a drag operation failed.
/// The value can by obtained by connecting to the "drag-failed" signal.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkDragResult {
    /// The drag operation was successful.
    GtkDragResultSuccess,
    /// No suitable drag target.
    GtkDragResultNoTarget,
    /// The user cancelled the drag operation.
    GtkDragResultUserCanceled,
    /// The drag operation timed out.
    GtkDragResultTimeoutExpired,
    /// The pointer or keyboard grab used for the drag operation was broken.
    GtkDragResultGrabBroken,
    /// The drag operation failed due to some unspecified error.
    GtkDragResultError
}

/// Availables accel flags
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkAccelFlags {
    /// display in GtkAccelLabel?
    GtkAccelVisible        = 1 << 0,
    /// is it removable?
    GtkAccelLocked         = 1 << 1,
    /// Comparison mask
    GtkAccelMask           = 0x07
}

/// Used to specify the placement of scroll arrows in scrolling menus.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkArrowPlacement {
    /// Place one arrow on each end of the menu.
    GtkArrowsBoth,
    /// Place both arrows at the top of the menu.
    GtkArrowsStart,
    /// Place both arrows at the bottom of the menu.
    GtkArrowsEnd
}

/// Used to indicate the direction in which a GtkArrow should point.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkArrowType {
    /// Represents an upward pointing arrow.
    GtkArrowUp,
    /// Represents a downward pointing arrow.
    GtkArrowDown,
    /// Represents a left pointing arrow.
    GtkArrowLeft,
    /// Represents a right pointing arrow.
    GtkArrowRight,
    /// No arrow. Since 2.10.
    GtkArrowNone
}

/// Denotes the expansion properties that a widget will have when it (or its parent) is resized.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkAttachOptions {
    /// the widget should expand to take up any extra space in its container that has been allocated.
    GtkExpand = 1 << 0,
    /// the widget should shrink as and when possible.
    GtkShrink = 1 << 1,
    /// the widget should fill the space allocated to it.
    GtkFill   = 1 << 2
}

/// Deleting modes
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkDeleteType {
    /// delete chars
    GtkDeleteChars,
    /// delete only the portion of the word to the left/right of cursor if we're in the middle of a word
    GtkDeleteWordsEnd,
    /// delete words
    GtkDeleteWords,
    /// delete lines
    GtkDeleteDisplayLines,
    /// deletes lines end
    GtkDeleteDisplayLineEnd,
    /// like C-k in Emacs (or its reverse)
    GtkDeleteParagraphEnds,
    /// C-k in pico, kill whole line
    GtkDeleteParagraphs,
    /// M-\ in Emacs
    GtkDeleteWhitespac
}

/// Used to specify the style of the expanders drawn by a GtkTreeView.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkExpanderStyle {
    /// The style used for a collapsed subtree.
    GtkExpanderCollapsed,
    /// Intermediate style used during animation.
    GtkExpanderSemiCollapsed,
    /// Intermediate style used during animation.
    GtkExpanderSemiExpanded,
    /// The style used for an expanded subtree.
    GGtkExpanderExpanded
}

/// preedit style
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkIMPreeditStyle {
    GtkImPreeditNothing,
    GtkImPreeditCallback,
    GtkImPreeditNone
}

/// Status styles
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkIMStatusStyle {
    GtkImStatusNothing,
    GtkImStatusCallback,
    GtkImStatusNone
}

/// Used for justifying the text inside a GtkLabel widget. (See also GtkAlignment).
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkJustification {
    /// The text is placed at the left edge of the label.
    GtkJustifyLeft,
    /// The text is placed at the right edge of the label.
    GtkJustifyRight,
    /// The text is placed in the center of the label.
    GtkJustifyCenter,
    /// The text is placed is distributed across the label.
    GtkJustifyFill
}

/// Availables movement steps
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkMovementStep {
    /// Move forward or back by graphemes
    GtkMovementLogicalPosition,
    /// Move left or right by graphemes
    GtkMovementVisualPositions,
    /// Move forward or back by words
    GtkMovementWords,
    /// Move up or down lines (wrapped lines)
    GtkMovementDisplayLines,
    /// Move to either end of a line
    GtkMovementDisplayLineEnds,
    /// Move up or down paragraphs (newline-ended lines)
    GtkMovementParagraphs,
    /// Move to either end of a paragraph
    GtkMovementParagraphEnds,
    /// Move by pages
    GtkMovementPages,
    /// Move to ends of the buffer
    GtkMovementBufferEnds,
    /// Move horizontally by pages
    GtkMovementHorizontalPages
}

/// Represents the packing location GtkBox children. (See: GtkVBox, GtkHBox, and GtkButtonBox).
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkPackType {
    /// The child is packed into the start of the box
    GtkPackStart,
    /// The child is packed into the end of the box
    GtkPackEnd
}


/// Availables Gtk path priority
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkPathPriorityType {
    GtkPathPrioLowest       = 0,
    GtkPathPrioGtk          = 4,
    GtkPathPrioApplication  = 8,
    GtkPathPrioTheme        = 10,
    GtkPathPrioRc           = 12,
    GtkPathPrioHighest      = 15
}

/// Availables Gtk path types
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkPathType {
    GtkPathWidget,
    GtkPathWidgetClass,
    GtkPathClass
}

/// Determines when a scroll bar will be visible.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkPolicyType {
    /// The scrollbar is always visible.
    GtkPolicyAlways,
    /// The scrollbar will appear and disappear as necessary. For example, when all of a GtkCList can not be seen.
    GtkPolicyAutomatic,
    /// The scrollbar will never appear.
    GtkPolicyNever
}

/// Describes which edge of a widget a certain feature is positioned at, e.g. the tabs of a GtkNotebook,
/// the handle of a GtkHandleBox or the label of a GtkScale.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkPositionType {
    /// The feature is at the left edge.
    GtkPosLeft,
    /// The feature is at the right edge.
    GtkPosRight,
    /// The feature is at the top edge.
    GtkPosTop,
    /// The feature is at the bottom edge.
    GtkPosBottom
}

/// Indicated the relief to be drawn around a GtkButton.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkReliefStyle {
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
pub enum GtkScrollStep {
  GtkScrollSteps,
  GtkScrollPages,
  GtkScrollEnds,
  GtkScrollHorizontalSteps,
  GtkScrollHorizontalPages,
  GtkScrollHorizontalEnds
}

/// Available scroll types
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkScrollType {
  GtkScrollNone,
  GtkScrollJump,
  GtkScrollStepBackward,
  GtkScrollStepForward,
  GtkScrollPageBackward,
  GtkScrollPageForward,
  GtkScrollStepUp,
  GtkScrollStepDown,
  GtkScrollPageUp,
  GtkScrollPageDown,
  GtkScrollStepLeft,
  GtkScrollStepRight,
  GtkScrollPageLeft,
  GtkScrollPageRight,
  GtkScrollStart,
  GtkScrollEnd
}

/// Used to control what selections users are allowed to make.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkSelectionMode {
    /// No selection is possible.
    GtkSelectionNone,
    /// Zero or one element may be selected.
    GtkSelectionSingle,
    /// Exactly one element is selected.
    GtkSelectionBrowse,
    /// Any number of elements may be selected.
    GtkSelectionMultiple
}

/// Used to change the appearance of an outline typically provided by a GtkFrame
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkShadowType {
    /// No outline.
    GtkShadowNone,
    /// The outline is bevelled inwards.
    GtkShadowIm,
    /// The outline is bevelled outwards like a button.
    GtkShadowOut,
    /// The outline has a sunken 3d appearance.
    GtkShadowEtchedIn,
    /// The outline has a raised 3d appearance.
    GtkShadowEtchedOut
}

/// This type indicates the current state of a widget; the state determines how the widget is drawn.
///
/// The GtkStateType enumeration is also used to identify different colors in a GtkStyle for drawing,
/// so states can be used for subparts of a widget as well as entire widgets.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkStateType {
    /// State during normal operation.
    GtkStateNormal,
    /// State of a currently active widget, such as a depressed button.
    GtkStateActive,
    /// State indicating that the mouse pointer is over the widget and the widget will respond to mouse clicks.
    GtkStatePrelight,
    /// State of a selected item, such the selected row in a list.
    GtkStateSelected,
    /// State indicating that the widget is unresponsive to user actions.
    GtkStateInsensitive,
    /// The widget is inconsistent, such as checkbuttons or radiobuttons that aren't either set to true nor false,
    /// or buttons requiring the user attention.
    GtkStateInconsistent,
    /// The widget has the keyboard focus
    GtkStateFocused
}

/// Used to customize the appearance of a GtkToolbar.
///
/// Note that setting the toolbar style overrides the user's preferences for the default toolbar style.
/// Note that if the button has only a label set and GTK_TOOLBAR_ICONS is used, the label will be visible, and vice versa.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkToolbarStyle {
    /// Buttons display only icons in the toolbar.
    GtkToolbarIcons,
    /// Buttons display only text labels in the toolbar.
    GtkToolbarText,
    /// Buttons display text and icons in the toolbar.
    GtkToolbarBoth,
    /// Buttons display icons and text alongside each other, rather than vertically stacked
    GtkToolbarBothHoriz
}

/// Describes how a rendered element connects to adjacent elements.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkJunctionSides {
    /// No junctions.
    GtkJunctionNone               = 0,
    /// Element connects on the top-left corner.
    GtkJunctionCornerTopleft      = 1 << 0,
    /// Element connects on the top-right corner.
    GtkJunctionCornerTopRight     = 1 << 1,
    /// Element connects on the bottom-left corner.
    GtkJunctionCornerBottomLeft   = 1 << 2,
    /// Element connects on the bottom-right corner.
    GtkJunctionCornerBottomRight  = 1 << 3,
    /// Element connects on the top side.
    GtkJunctionTop                = (1 << 0 | 1 << 1),
    /// Element connects on the bottom side.
    GtkJunctionBottom             = (1 << 2 | 1 << 3),
    /// Element connects on the left side.
    GtkJunctionLeft               = (1 << 0 | 1 << 2),
    /// Element connects on the right side.
    GtkJunctionRight              = (1 << 1 | 1 << 3)
}

/// Describes a region within a widget.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkRegionFlags {
    /// Region has an even number within a set.
    GtkRegionEven    = 1 << 0,
    /// Region has an odd number within a set.
    GtkRegionOdd     = 1 << 1,
    /// Region is the first one within a set.
    GtkRegionFirst   = 1 << 2,
    /// Region is the last one within a set.
    GtkRegionLast    = 1 << 3,
    /// Region is the only one within a set.
    GtkRegionOnly    = 1 << 4,
    /// Region is part of a sorted area.
    GtkRegionSorted  = 1 << 5
}

/// Icon Sizes : temporary
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkIconSize {
  GtkIconSizeInvalid,
  GtkIconSizeMemu,
  GtkIconSizeSmallToolbar,
  GtkIconSizeLargeToolbar,
  GtkIconSizeButton,
  GtkIconSizeDnd,
  GtkIconSizeDisalog
}

/// Specifies the side of the entry at which an icon is placed.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkEntryIconPosition {
    /// At the beginning of the entry (depending on the text direction).
    GtkEntryIconPrimary,
    /// At the end of the entry (depending on the text direction).
    GtkEntryIconSecondary
}

/// Describes hints that might be taken into account by input methods or applications.
/// Note that input methods may already tailor their behaviour according to the GtkInputPurpose of the entry.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkInputHints {
    /// No special behaviour suggested
    GtkInputHintNone                = 0,
    /// Suggest checking for typos
    GtkInputHintSpellcheck          = 1 << 0,
    /// Suggest not checking for typos
    GtkInputHintNoSpellcheck        = 1 << 1,
    /// Suggest word completion
    GtkInputHintWordCompletion      = 1 << 2,
    /// Suggest to convert all text to lowercase
    GtkInputHintLowercase           = 1 << 3,
    /// Suggest to capitalize all text
    GtkInputHintUppercaseChars      = 1 << 4,
    /// Suggest to capitalize the first character of each word
    GtkInputHintUppercaseWords      = 1 << 5,
    /// Suggest to capitalize the first word of each sentence
    GtkInputHintUppercaseSentences  = 1 << 6,
    /// Suggest to not show an onscreen keyboard (e.g for a calculator that already has all the keys).
    GtkInputHintInhibitOsk          = 1 << 7
}

/// Describes primary purpose of the input widget.
/// This information is useful for on-screen keyboards
/// and similar input methods to decide which keys should be presented to the user.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkInputPurpose {
    /// Allow any character
    GtkInputPurposeFreeForm,
    /// Allow only alphabetic characters
    GtkInputPurposeAlpha,
    /// Allow only digits
    GtkInputPurposeDigits,
    /// Edited field expects numbers
    GtkInputPurposeNumber,
    /// Edited field expects phone number
    GtkInputPurposePhone,
    /// Edited field expects URL
    GtkInputPurposeUrl,
    /// Edited field expects email address
    GtkInputPurposeEmail,
    /// Edited field expects the name of a person
    GtkInputPurposeName,
    /// Like GtkInputPurposeFreeForm, but characters are hidden
    GtkInputPurposePassword,
    /// Like GtkInputPurposeDigits, but characters are hidden
    GtkInputPurposePin
}

/// Describes the image data representation used by a GtkImage.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkImageType {
    /// there is no image displayed by the widget
    GtkImageEmpty,
    /// the widget contains a Gdk::Pixbuf
    GtkImagePixbuf,
    /// the widget contains a stock icon name (see Stock Items(3))
    GtkImageStock,
    /// the widget contains a Gtk::IconSet
    GtkImageIconSet,
    /// the widget contains a Gdk::PixbufAnimation
    GtkImageAnimation,
    /// the widget contains a named icon. This image type was added in GTK+ 2.6
    GtkImageIconName,
    /// the widget contains a GIcon. This image type was added in GTK+ 2.14
    GtkImageGIcon,
    /// the widget contains a cairo_surface_t. This image type was added in GTK+ 3.10
    GtkImageSurface
}

/// The values of the GtkSpinType enumeration are used
/// to specify the change to make in gtk::SpinButton::spin().
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkSpinType {
    /// Increment by the adjustments step increment.
    GtkSpinStepForward,
    /// Decrement by the adjustments step increment.
    GtkSpinStepBackward,
    /// Increment by the adjustments page increment.
    GtkSpinPageForward,
    /// Decrement by the adjustments page increment.
    GtkSpinPageBackward,
    /// Go to the adjustments lower bound.
    GtkSpinHome,
    /// Go to the adjustments upper bound.
    GtkSpinEnd,
    /// Change by a specified amount.
    GtkSpinUserDefined
}

/// The spin button update policy determines whether the spin button displays values
/// even if they are outside the bounds of its adjustment. See gtk::SpinButton::set_update_policy().
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkSpinButtonUpdatePolicy {
    /// When refreshing your Gtk::SpinButton, the value is always displayed
    GtkUpdateAlways,
    /// When refreshing your Gtk::SpinButton, the value is only displayed
    /// if it is valid within the bounds of the spin button's adjustment
    GtkUpdateIfValid
}

/// Describes how GtkLevelBar contents should be rendered.
/// Note that this enumeration could be extended with additional modes in the future.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkLevelBarMode {
    /// the bar has a continuous mode
    GtkLevelBarModeContinuous,
    /// the bar has a discrete mode
    GtkLevelBarModeDiscrete
}

/// These options can be used to influence the display and behaviour of a gtk::Calendar.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkCalendarDisplayOptions {
    /// Specifies that the month and year should be displayed.
    GtkCalendarShowHeading        = 1 << 0,
    /// Specifies that three letter day descriptions should be present.
    GtkCalendarShowDayNames       = 1 << 1,
    /// Prevents the user from switching months with the calendar.
    GtkCalendarNoMonthChange      = 1 << 2,
    /// Displays each week numbers of the current year, down the left side of the calendar.
    GtkCalendarShowWeekNumbers    = 1 << 3,
    /// Just show an indicator, not the full details text when details are provided. See gtk::Calendar::set_detail_func().
    GtkCalendarShowDetails        = 1 << 5
}

/// The type of message being displayed in the dialog.
#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum GtkMessageType {
    /// Informational message
    GtkMessageInfo,
    /// Non-fatal warning message
    GtkMessageWarning,
    /// Question requiring a choice
    GtkMessageQuestion,
    /// Fatal error message
    GtkMessageError,
    /// None of the above, doesn't get an icon
    GtkMessageOther
}
