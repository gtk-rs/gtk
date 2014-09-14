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

pub use self::window_type::WindowType;
pub use self::text_direction::TextDirection;
pub use self::window_position::WindowPosition;
pub use self::button_box_style::ButtonBoxStyle;
pub use self::orientation::Orientation;
pub use self::direction_type::DirectionType;
pub use self::corner_type::CornerType;
pub use self::resize_mode::ResizeMode;
pub use self::border_style::BorderStyle;
pub use self::sort_type::SortType;
pub use self::state_flags::StateFlags;
pub use self::drag_result::DragResult;
pub use self::accel_flags::AccelFlags;
pub use self::arrow_placement::ArrowPlacement;
pub use self::arrow_type::ArrowType;
pub use self::attach_options::AttachOptions;
pub use self::delete_type::DeleteType;
pub use self::expander_style::ExpanderStyle;
pub use self::im_preedit_style::IMPreeditStyle;
pub use self::im_status_style::IMStatusStyle;
pub use self::justification::Justification;
pub use self::movement_step::MovementStep;
pub use self::pack_type::PackType;
pub use self::path_priority_type::PathPriorityType;
pub use self::path_type::PathType;
pub use self::policy_type::PolicyType;
pub use self::position_type::PositionType;
pub use self::relief_style::ReliefStyle;
pub use self::scroll_step::ScrollStep;
pub use self::scroll_type::ScrollType;
pub use self::selection_mode::SelectionMode;
pub use self::shadow_type::ShadowType;
pub use self::state_type::StateType;
pub use self::toolbar_style::ToolbarStyle;
pub use self::junction_sides::JunctionSides;
pub use self::region_flags::RegionFlags;
pub use self::icon_size::IconSize;
pub use self::entry_icon_position::EntryIconPosition;
pub use self::input_hints::InputHints;
pub use self::input_purpose::InputPurpose;
pub use self::image_type::ImageType;
pub use self::spin_type::SpinType;
pub use self::spin_button_update_policy::SpinButtonUpdatePolicy;
pub use self::level_bar_mode::LevelBarMode;
pub use self::calendar_display_options::CalendarDisplayOptions;
pub use self::message_type::MessageType;
pub use self::license::License;
pub use self::response_type::ResponseType;
pub use self::dialog_flags::DialogFlags;
pub use self::file_chooser_action::FileChooserAction;
pub use self::buttons_type::ButtonsType;
pub use self::stack_transition_type::StackTransitionType;
pub use self::revealer_transition_type::RevealerTransitionType;
pub use self::scrollable_policy::ScrollablePolicy;
pub use self::file_filter_flags::FileFilterFlags;
pub use self::app_info_create_flags::AppInfoCreateFlags;
pub use self::size_request_mode::SizeRequestMode;
pub use self::align::Align;
pub use self::g_connect_flags::GConnectFlags;
pub use self::builder_error::BuilderError;
pub use self::page_orientation::PageOrientation;
pub use self::unit::Unit;
pub use self::number_up_layout::NumberUpLayout;
pub use self::print_pages::PrintPages;
pub use self::page_set::PageSet;
pub use self::recent_sort_type::RecentSortType;
pub use self::recent_filter_flags::RecentFilterFlags;
pub use self::widget_help_type::WidgetHelpType;
pub use self::text_window_type::TextWindowType;
pub use self::wrap_mode::WrapMode;
pub use self::tree_view_grid_lines::TreeViewGridLines;
pub use self::tree_view_column_sizing::TreeViewColumnSizing;
pub use self::cell_renderer_state::CellRendererState;
pub use self::tree_model_flags::TreeModelFlags;

pub mod window_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum WindowType{
        /// A regular window, such as a dialog.
        TopLevel,
        /// A special window such as a tooltip.
        PopUp
    }
}

/// Reading directions for text
pub mod text_direction{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum TextDirection{
        None,
        Ltr,
        Rtl
    }
}

/// Window placement can be influenced using this enumeration.
/// Note that using WinPosCenterAlways is almost always a bad idea.
/// It won't necessarily work well with all window managers or on all windowing systems.
pub mod window_position{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum WindowPosition{
        /// No influence is made on placement.
        None,
        /// Windows should be placed in the center of the screen.
        Center,
        /// Windows should be placed at the current mouse position.
        Mouse,
        /// Keep window centered as it changes size, etc.
        CenterAlways,
        /// Center the window on its transient parent (see window.set_transient_for()).
        CenterOnParent
    }
}

/// Used to dictate the style that a gtk::ButtonBox uses to layout the buttons it contains.
pub mod button_box_style{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ButtonBoxStyle{
        /// Buttons are evenly spread across the box.
        Spread = 1,
        /// Buttons are placed at the edges of the box.
        Edge,
        /// Buttons are grouped towards the start of the box, (on the left for a HBox, or the top for a VBox).
        Start,
        /// Buttons are grouped towards the end of the box, (on the right for a HBox, or the bottom for a VBox).
        End,
        /// Buttons are centered in the box. Since 2.12.
        Center
    }
}

/// Represents the orientation of widgets which can be switched between
/// horizontal and vertical orientation on the fly, like gtk::Toolbar.
pub mod orientation{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum Orientation{
        /// The widget is in horizontal orientation.
        Horizontal,
        /// The widget is in vertical orientation.
        Vertical
    }
}

/// Availables direction types
pub mod direction_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum DirectionType{
        TabForward,
        TabBackward,
        Up,
        Down,
        Left,
        Right
    }
}

/// Specifies which corner a child widget should be placed in when packed into a gtk::ScrolledWindow.
/// This is effectively the opposite of where the scroll bars are placed.
pub mod corner_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum CornerType{
        /// Place the scrollbars on the right and bottom of the widget (default behaviour).
        TopLeft,
        /// Place the scrollbars on the top and right of the widget.
        BottomLeft,
        /// Place the scrollbars on the left and bottom of the widget.
        TopRight,
        /// Place the scrollbars on the top and left of the widget.
        BottomRight
    }
}

/// Availables resize modes
pub mod resize_mode{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ResizeMode{
        /// Pass resize request to the parent
        Parent,
        /// Queue resizes on this widget
        Queue,
        /// Resize immediately. Deprecated.
        Immediate
    }
}

/// Describes how the border of a UI element should be rendered.
pub mod border_style{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum BorderStyle{
        /// No visible border
        None,
        /// A single line segment
        Solid,
        /// Looks as if the content is sunken into the canvas
        Inset,
        /// Looks as if the content is coming out of the canvas
        Outset,
        /// Same as BorderStyleNone
        Hidden,
        /// A series of round dots
        Dotted,
        /// A series of square-ended dashes
        Dashed,
        /// Two parallel lines with some space between them
        Double,
        /// Looks as if it were carved in the canvas
        Groove,
        /// Looks as if it were coming out of the canvas
        Ridge
    }
}

/// Determines the direction of a sort.
pub mod sort_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum SortType{
        /// Sorting is in ascending order
        Ascending,
        /// Sorting is in descending order.
        Descending
    }
}

/// Describes a widget state. Widget states are used to match the widget against CSS pseudo-classes.
/// Note that GTK extends the regular CSS classes and sometimes uses different names.
pub mod state_flags {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum StateFlags {
        /// State during normal operation.
        Normal       = 0,
        /// Widget is active.
        Active       = 1 << 0,
        /// Widget has a mouse pointer over it.
        Prelight     = 1 << 1,
        /// Widget is selected.
        Selected     = 1 << 2,
        /// Widget is insensitive.
        Insensitive  = 1 << 3,
        /// Widget is inconsistent.
        Inconsistent = 1 << 4,
        /// Widget has the keyboard focus.
        Focused      = 1 << 5,
        /// Widget is in a background toplevel window.
        BackDrop     = 1 << 6,
        /// Widget is in left-to-right text direction. Since 3.8
        DirLTR       = 1 << 7,
        /// Widget is in right-to-left text direction. Since 3.8
        DirRTL       = 1 << 8
    }
}

/// Gives an indication why a drag operation failed.
/// The value can by obtained by connecting to the "drag-failed" signal.
pub mod drag_result{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum DragResult{
        /// The drag operation was successful.
        Success,
        /// No suitable drag target.
        NoTarget,
        /// The user cancelled the drag operation.
        UserCanceled,
        /// The drag operation timed out.
        TimeoutExpired,
        /// The pointer or keyboard grab used for the drag operation was broken.
        GrabBroken,
        /// The drag operation failed due to some unspecified error.
        Error
    }
}

/// Availables accel flags
pub mod accel_flags{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum AccelFlags{
        /// display in AccelLabel?
        Visible        = 1 << 0,
        /// is it removable?
        Locked         = 1 << 1,
        /// Comparison mask
        Mask           = 0x07
    }
}

/// Used to specify the placement of scroll arrows in scrolling menus.
pub mod arrow_placement{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ArrowPlacement{
        /// Place one arrow on each end of the menu.
        Both,
        /// Place both arrows at the top of the menu.
        Start,
        /// Place both arrows at the bottom of the menu.
        End
    }
}

/// Used to indicate the direction in which a Arrow should point.
pub mod arrow_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ArrowType{
        /// Represents an upward pointing arrow.
        Up,
        /// Represents a downward pointing arrow.
        Down,
        /// Represents a left pointing arrow.
        Left,
        /// Represents a right pointing arrow.
        Right,
        /// No arrow. Since 2.10.
        None
    }
}

/// Denotes the expansion properties that a widget will have when it (or its parent) is resized.
pub mod attach_options{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum AttachOptions{
        /// the widget should expand to take up any extra space in its container that has been allocated.
        Expand = 1 << 0,
        /// the widget should shrink as and when possible.
        Shrink = 1 << 1,
        /// the widget should fill the space allocated to it.
        Fill   = 1 << 2
    }
}

/// Deleting modes
pub mod delete_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum DeleteType{
        /// delete chars
        Chars,
        /// delete only the portion of the word to the left/right of cursor if we're in the middle of a word
        WordsEnd,
        /// delete words
        Words,
        /// delete lines
        DisplayLines,
        /// deletes lines end
        DisplayLineEnd,
        /// like C-k in Emacs (or its reverse)
        ParagraphEnds,
        /// C-k in pico, kill whole line
        Paragraphs,
        /// M-\ in Emacs
        Whitespac
    }
}

/// Used to specify the style of the expanders drawn by a TreeView.
pub mod expander_style{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ExpanderStyle{
        /// The style used for a collapsed subtree.
        Collapsed,
        /// Intermediate style used during animation.
        SemiCollapsed,
        /// Intermediate style used during animation.
        SemiExpanded,
        /// The style used for an expanded subtree.
        Expanded
    }
}

/// preedit style
pub mod im_preedit_style{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum IMPreeditStyle{
        Nothing,
        Callback,
        None
    }
}

/// Status styles
pub mod im_status_style{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum IMStatusStyle{
        Nothing,
        Callback,
        None
    }
}

/// Used for justifying the text inside a Label widget. (See also Alignment).
pub mod justification{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum Justification{
        /// The text is placed at the left edge of the label.
        Left,
        /// The text is placed at the right edge of the label.
        Right,
        /// The text is placed in the center of the label.
        Center,
        /// The text is placed is distributed across the label.
        Fill
    }
}

/// Availables movement steps
pub mod movement_step{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum MovementStep{
        /// Move forward or back by graphemes
        LogicalPosition,
        /// Move left or right by graphemes
        VisualPositions,
        /// Move forward or back by words
        Words,
        /// Move up or down lines (wrapped lines)
        DisplayLines,
        /// Move to either end of a line
        DisplayLineEnds,
        /// Move up or down paragraphs (newline-ended lines)
        Paragraphs,
        /// Move to either end of a paragraph
        ParagraphEnds,
        /// Move by pages
        Pages,
        /// Move to ends of the buffer
        BufferEnds,
        /// Move horizontally by pages
        HorizontalPages
    }
}

/// Represents the packing location Box children. (See: VBox, HBox, and ButtonBox).
pub mod pack_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum PackType{
        /// The child is packed into the start of the box
        Start,
        /// The child is packed into the end of the box
        End
    }
}


/// Availables Gtk path priority
pub mod path_priority_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum PathPriorityType{
        Lowest       = 0,
        Gtk          = 4,
        Application  = 8,
        Theme        = 10,
        Rc           = 12,
        Highest      = 15
    }
}

/// Availables Gtk path types
pub mod path_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum PathType{
        Widget,
        WidgetClass,
        Class
    }
}

/// Determines when a scroll bar will be visible.
pub mod policy_type {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum PolicyType {
        /// The scrollbar is always visible.
        Always,
        /// The scrollbar will appear and disappear as necessary. For example, when all of a CList can not be seen.
        Automatic,
        /// The scrollbar will never appear.
        Never
    }
}

/// Describes which edge of a widget a certain feature is positioned at, e.g. the tabs of a Notebook,
/// the handle of a HandleBox or the label of a Scale.
pub mod position_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum PositionType{
        /// The feature is at the left edge.
        Left,
        /// The feature is at the right edge.
        Right,
        /// The feature is at the top edge.
        Top,
        /// The feature is at the bottom edge.
        Bottom
    }
}

/// Indicated the relief to be drawn around a Button.
pub mod relief_style{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ReliefStyle{
        /// Draw a normal relief.
        Normal,
        /// A half relief.
        Half,
        /// No relief.
        None
    }
}

/// Available scroll steps
pub mod scroll_step{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ScrollStep{
        Steps,
        Pages,
        Ends,
        HorizontalSteps,
        HorizontalPages,
        HorizontalEnds
    }
}

/// Available scroll types
pub mod scroll_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ScrollType{
        None,
        Jump,
        StepBackward,
        StepForward,
        PageBackward,
        PageForward,
        StepUp,
        StepDown,
        PageUp,
        PageDown,
        StepLeft,
        StepRight,
        PageLeft,
        PageRight,
        Start,
        End
    }
}

/// Used to control what selections users are allowed to make.
pub mod selection_mode{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum SelectionMode{
        /// No selection is possible.
        None,
        /// Zero or one element may be selected.
        Single,
        /// Exactly one element is selected.
        Browse,
        /// Any number of elements may be selected.
        Multiple
    }
}

/// Used to change the appearance of an outline typically provided by a Frame
pub mod shadow_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ShadowType{
        /// No outline.
        None,
        /// The outline is bevelled inwards.
        Im,
        /// The outline is bevelled outwards like a button.
        Out,
        /// The outline has a sunken 3d appearance.
        EtchedIn,
        /// The outline has a raised 3d appearance.
        EtchedOut
    }
}

/// This type indicates the current state of a widget; the state determines how the widget is drawn.
///
/// The StateType enumeration is also used to identify different colors in a Style for drawing,
/// so states can be used for subparts of a widget as well as entire widgets.
pub mod state_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum StateType{
        /// State during normal operation.
        Normal,
        /// State of a currently active widget, such as a depressed button.
        Active,
        /// State indicating that the mouse pointer is over the widget and the widget will respond to mouse clicks.
        Prelight,
        /// State of a selected item, such the selected row in a list.
        Selected,
        /// State indicating that the widget is unresponsive to user actions.
        Insensitive,
        /// The widget is inconsistent, such as checkbuttons or radiobuttons that aren't either set to true nor false,
        /// or buttons requiring the user attention.
        Inconsistent,
        /// The widget has the keyboard focus
        Focused
    }
}

/// Used to customize the appearance of a Toolbar.
///
/// Note that setting the toolbar style overrides the user's preferences for the default toolbar style.
/// Note that if the button has only a label set and GTK_TOOLBAR_ICONS is used, the label will be visible, and vice versa.
pub mod toolbar_style{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ToolbarStyle{
        /// Buttons display only icons in the toolbar.
        Icons,
        /// Buttons display only text labels in the toolbar.
        Text,
        /// Buttons display text and icons in the toolbar.
        Both,
        /// Buttons display icons and text alongside each other, rather than vertically stacked
        BothHoriz
    }
}

/// Describes how a rendered element connects to adjacent elements.
pub mod junction_sides{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum JunctionSides{
        /// No junctions.
        None               = 0,
        /// Element connects on the top-left corner.
        CornerTopleft      = 1 << 0,
        /// Element connects on the top-right corner.
        CornerTopRight     = 1 << 1,
        /// Element connects on the bottom-left corner.
        CornerBottomLeft   = 1 << 2,
        /// Element connects on the bottom-right corner.
        CornerBottomRight  = 1 << 3,
        /// Element connects on the top side.
        Top                = (1 << 0 | 1 << 1),
        /// Element connects on the bottom side.
        Bottom             = (1 << 2 | 1 << 3),
        /// Element connects on the left side.
        Left               = (1 << 0 | 1 << 2),
        /// Element connects on the right side.
        Right              = (1 << 1 | 1 << 3)
    }
}

/// Describes a region within a widget.
pub mod region_flags{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum RegionFlags{
        /// Region has an even number within a set.
        Even    = 1 << 0,
        /// Region has an odd number within a set.
        Odd     = 1 << 1,
        /// Region is the first one within a set.
        First   = 1 << 2,
        /// Region is the last one within a set.
        Last    = 1 << 3,
        /// Region is the only one within a set.
        Only    = 1 << 4,
        /// Region is part of a sorted area.
        Sorted  = 1 << 5
    }
}

/// Built-in stock icon sizes
pub mod icon_size {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum IconSize {
        // Invalid size
        Invalid,
        // Size appropriate for menus (16px)
        Menu,
        // Size appropriate for small toolbars (16px)
        SmallToolbar,
        // Size appropriate for large toolbars (24px)
        LargeToolbar,
        // Size appropriate for buttons (16px)
        Button,
        // Size appropriate for drag and drop (32px)
        Dnd,
        // Size appropriate for dialogs (48px)
        Dialog
    }
}

/// Specifies the side of the entry at which an icon is placed.
pub mod entry_icon_position{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum EntryIconPosition{
        /// At the beginning of the entry (depending on the text direction).
        Primary,
        /// At the end of the entry (depending on the text direction).
        Secondary
    }
}

/// Describes hints that might be taken into account by input methods or applications.
/// Note that input methods may already tailor their behaviour according to the InputPurpose of the entry.
pub mod input_hints{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum InputHints{
        /// No special behaviour suggested
        None                = 0,
        /// Suggest checking for typos
        Spellcheck          = 1 << 0,
        /// Suggest not checking for typos
        NoSpellcheck        = 1 << 1,
        /// Suggest word completion
        WordCompletion      = 1 << 2,
        /// Suggest to convert all text to lowercase
        Lowercase           = 1 << 3,
        /// Suggest to capitalize all text
        UppercaseChars      = 1 << 4,
        /// Suggest to capitalize the first character of each word
        UppercaseWords      = 1 << 5,
        /// Suggest to capitalize the first word of each sentence
        UppercaseSentences  = 1 << 6,
        /// Suggest to not show an onscreen keyboard (e.g for a calculator that already has all the keys).
        InhibitOsk          = 1 << 7
    }
}

/// Describes primary purpose of the input widget.
/// This information is useful for on-screen keyboards
/// and similar input methods to decide which keys should be presented to the user.
pub mod input_purpose{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum InputPurpose{
        /// Allow any character
        FreeForm,
        /// Allow only alphabetic characters
        Alpha,
        /// Allow only digits
        Digits,
        /// Edited field expects numbers
        Number,
        /// Edited field expects phone number
        Phone,
        /// Edited field expects URL
        Url,
        /// Edited field expects email address
        Email,
        /// Edited field expects the name of a person
        Name,
        /// Like InputPurposeFreeForm, but characters are hidden
        Password,
        /// Like InputPurposeDigits, but characters are hidden
        Pin
    }
}

/// Describes the image data representation used by a Image.
pub mod image_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ImageType{
        /// there is no image displayed by the widget
        Empty,
        /// the widget contains a Gdk::Pixbuf
        Pixbuf,
        /// the widget contains a stock icon name (see Stock Items(3))
        Stock,
        /// the widget contains a Gtk::IconSet
        IconSet,
        /// the widget contains a Gdk::PixbufAnimation
        Animation,
        /// the widget contains a named icon. This image type was added in GTK+ 2.6
        IconName,
        /// the widget contains a GIcon. This image type was added in GTK+ 2.14
        GIcon,
        /// the widget contains a cairo_surface_t. This image type was added in GTK+ 3.10
        Surface
    }
}

/// The values of the SpinType enumeration are used
/// to specify the change to make in gtk::SpinButton::spin().
pub mod spin_type{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum SpinType{
        /// Increment by the adjustments step increment.
        StepForward,
        /// Decrement by the adjustments step increment.
        StepBackward,
        /// Increment by the adjustments page increment.
        PageForward,
        /// Decrement by the adjustments page increment.
        PageBackward,
        /// Go to the adjustments lower bound.
        Home,
        /// Go to the adjustments upper bound.
        End,
        /// Change by a specified amount.
        UserDefined
    }
}

/// The spin button update policy determines whether the spin button displays values
/// even if they are outside the bounds of its adjustment. See gtk::SpinButton::set_update_policy().
pub mod spin_button_update_policy{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum SpinButtonUpdatePolicy{
        /// When refreshing your Gtk::SpinButton, the value is always displayed
        Always,
        /// When refreshing your Gtk::SpinButton, the value is only displayed
        /// if it is valid within the bounds of the spin button's adjustment
        IfValid
    }
}

/// Describes how LevelBar contents should be rendered.
/// Note that this enumeration could be extended with additional modes in the future.
pub mod level_bar_mode{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum LevelBarMode{
        /// the bar has a continuous mode
        Continuous,
        /// the bar has a discrete mode
        Discrete
    }
}

/// These options can be used to influence the display and behaviour of a gtk::Calendar.
pub mod calendar_display_options{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum CalendarDisplayOptions{
        /// Specifies that the month and year should be displayed.
        ShowHeading        = 1 << 0,
        /// Specifies that three letter day descriptions should be present.
        ShowDayNames       = 1 << 1,
        /// Prevents the user from switching months with the calendar.
        NoMonthChange      = 1 << 2,
        /// Displays each week numbers of the current year, down the left side of the calendar.
        ShowWeekNumbers    = 1 << 3,
        /// Just show an indicator, not the full details text when details are provided. See gtk::Calendar::set_detail_func().
        ShowDetails        = 1 << 5
    }
}

/// The type of message being displayed in the dialog.
pub mod message_type {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum MessageType {
        /// Informational message
        Info,
        /// Non-fatal warning message
        Warning,
        /// Question requiring a choice
        Question,
        /// Fatal error message
        Error,
        /// None of the above, doesn't get an icon
        Other
    }
}

/// Prebuilt sets of buttons for the dialog.
/// If none of these choices are appropriate, simply use GTK_BUTTONS_NONE then call gtk_dialog_add_buttons().
pub mod buttons_type {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ButtonsType {
        /// no buttons at all
        None,
        /// an OK button
        Ok,
        /// a Close button
        Close,
        /// a Cancel button
        Cancel,
        /// Yes and No buttons
        YesNo,
        /// OK and Cancel buttons
        OkCancel
    }
}

/// Flags used to influence dialog construction.
pub mod dialog_flags {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum DialogFlags {
        /// Make the constructed dialog modal, see gtk_window_set_modal()
        Modal,
        /// Destroy the dialog when its parent is destroyed, see gtk_window_set_destroy_with_parent()
        DestroyWithParents,
        /// Create dialog with actions in header bar instead of action area.
        UseHeaderBar
    }
}

/// The type of license for an application.
/// This enumeration can be expanded at later date.
#[allow(non_camel_case_types)]
pub mod license {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum License {
        /// No license specified
        Unknown,
        /// A license text is going to be specified by the developer
        Custom,
        /// The GNU General Public License, version 2.0 or later
        GPL_2_0,
        /// The GNU General Public License, version 3.0 or later
        GPL_3_0,
        /// The GNU Lesser General Public License, version 2.1 or later
        LGPL_2_1,
        /// The GNU Lesser General Public License, version 3.0 or later
        LGPL_3_0,
        /// The BSD standard license
        BSD,
        /// The MIT/X11 standard license
        MIT_X11,
        /// The Artistic License, version 2.0
        Artistic,
        /// The GNU General Public License, version 2.0 only
        GPL_2_0_Only,
        /// The GNU General Public License, version 3.0 only
        GPL_3_0_Only,
        /// The GNU Lesser General Public License, version 2.1 only
        LGPL_2_1_Only,
        /// The GNU Lesser General Public License, version 3.0 only
        LGPL_3_0_Only
    }
}

/// Predefined values for use as response ids in gtk_dialog_add_button().
/// All predefined values are negative, GTK+ leaves positive values for application-defined response ids.
pub mod response_type {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show, FromPrimitive)]
    pub enum ResponseType {
        /// Returned if an action widget has no response id, or if the dialog gets programmatically hidden or destroyed
        None        = -1,
        /// Generic response id, not used by GTK+ dialogs
        Reject      = -2,
        /// Generic response id, not used by GTK+ dialogs
        Accept      = -3,
        /// Returned if the dialog is deleted
        DeleteEvent = -4,
        /// Returned by OK buttons in GTK+ dialogs
        Ok          = -5,
        /// Returned by Cancel buttons in GTK+ dialogs
        Cancel      = -6,
        /// Returned by Close buttons in GTK+ dialogs
        Close       = -7,
        /// Returned by Yes buttons in GTK+ dialogs
        Yes         = -8,
        /// Returned by No buttons in GTK+ dialogs
        No          = -9,
        /// Returned by Apply buttons in GTK+ dialogs
        Apply       = -10,
        /// Returned by Help buttons in GTK+ dialogs
        Help        = -11
    }
}

/// Describes whether a GtkFileChooser is being used to open existing files or to save to a possibly new file.
pub mod file_chooser_action {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum FileChooserAction {
        /// Indicates open mode. The file chooser will only let the user pick an existing file.
        Open,
        /// Indicates save mode. The file chooser will let the user pick an existing file, or type in a new filename.
        Save,
        /// Indicates an Open mode for selecting folders. The file chooser will let the user pick an existing folder.
        SelectFolder,
        /// Indicates a mode for creating a new folder. The file chooser will let the user name an existing or new folder.
        CreateFolder
    }
}

/// These flags indicate what parts of a GtkFileFilterInfo struct are filled or need to be filled.
pub mod file_filter_flags {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum FileFilterFlags {
        /// the filename of the file being tested
        Filename,
        /// the URI for the file being tested
        Uri,
        /// the string that will be used to display the file in the file chooser
        DisplayName,
        /// the mime type of the file
        MimeType
    }
}

/// Describe the possible transitions between pages in a GtkStack widget.
pub mod stack_transition_type {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum StackTransitionType {
        /// No transition
        None,
        /// A cross-fade
        CrossFade,
        /// Slide from left to right
        SlideRight,
        /// Slide from right to left
        SlideLeft,
        /// Slide from bottom up
        SlideUp,
        /// Slide from top down
        SlideDown,
        /// Slide from left or right according to the children order
        SlideLeftRight,
        /// Slide from top down or bottom up according to the order
        SlideUpDown,
        /// Cover the old page by sliding up. Since 3.12
        OverUp,
        /// Cover the old page by sliding down. Since: 3.12
        OverDown,
        /// Cover the old page by sliding to the left. Since: 3.12
        OverLeft,
        /// Cover the old page by sliding to the right. Since: 3.12
        OverRight,
        /// Uncover the new page by sliding up. Since 3.12
        UnderUp,
        /// Uncover the new page by sliding down. Since: 3.12
        UnderDown,
        /// Uncover the new page by sliding to the left. Since: 3.12
        UnderLeft,
        /// Uncover the new page by sliding to the right. Since: 3.12
        UnderRight,
        /// Cover the old page or uncover the new page, according to order. Since: 3.12
        OverUpDown
    }
}

/// Describe the possible transitions when the child of a GtkRevealer widget is shown or hidden.
pub mod revealer_transition_type {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum RevealerTransitionType {
        /// No transition
        None,
        /// Fade in
        CrossFade,
        /// Slide in from the left
        SlideRight,
        /// Slide in from the right
        SlideLeft,
        /// Slide in from the bottom
        SlideUp,
        /// Slide in from the top
        SlideDown
    }
}

/// The policy to be used in a scrollable widget when updating the scrolled window adjustments in
/// a given orientation.
pub mod scrollable_policy {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ScrollablePolicy {
        /// Scrollable adjustments are based on the minimum size
        Minimum,
        /// Scrollable adjustments are based on the natural size
        Natural
    }
}

/// Flags used when creating a GAppInfo.
pub mod app_info_create_flags {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum AppInfoCreateFlags {
        /// No flags.
        None,
        /// Application opens in a terminal window.
        NeedsTerminal,
        /// Application supports URI arguments.
        SupportsUris,
        /// Application supports startup notification.
        SupportsStartupNotification
    }
}

/// Specifies a preference for height-for-width or width-for-height geometry management.
pub mod size_request_mode {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum SizeRequestMode {
        /// Prefer height-for-width geometry management
        HeightForWidth,
        /// Prefer width-for-height geometry management
        WidthForHeight,
        /// Don’t trade height-for-width or width-for-height
        ConstantSize
    }
}

/// Controls how a widget deals with extra space in a single (x or y) dimension.
pub mod align {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum Align {
        /// stretch to fill all space if possible, center if no meaningful way to stretch
        Fill,
        /// snap to left or top side, leaving space on right or bottom
        Start,
        /// snap to right or bottom side, leaving space on left or top
        End,
        /// center natural width of widget inside the allocation
        Center,
        /// align the widget according to the baseline.
        AlignBaseline
    }
}

/// The connection flags are used to specify the behaviour of a signal's connection.
pub mod g_connect_flags {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum GConnectFlags {
        /// whether the handler should be called before or after the default handler of the signal.
        After,
        /// whether the instance and data should be swapped when calling the handler; see g_signal_connect_swapped() for an example.
        Swapped
    }
}

/// Error codes that identify various errors that can occur while using GtkBuilder.
pub mod builder_error {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum BuilderError {
        /// A type-func attribute didn’t name a function that returns a GType.
        InvalidTypeFunction,
        /// The input contained a tag that GtkBuilder can’t handle.
        UnhandledTag,
        /// An attribute that is required by GtkBuilder was missing.
        MissingAttribute,
        /// GtkBuilder found an attribute that it doesn’t understand.
        InvalidAttribute,
        /// GtkBuilder found a tag that it doesn’t understand.
        InvalidTag,
        /// A required property value was missing.
        MissingPropertyValue,
        /// GtkBuilder couldn’t parse some attribute value.
        InvalidValue,
        /// The input file requires a newer version of GTK+.
        VersionMismatch,
        /// An object id occurred twice.
        DuplicateID,
        /// A specified object type is of the same type or derived from the type of the composite class being extended with builder XML.
        ObjectTypeRefused,
        /// The wrong type was specified in a composite class’s template XML
        TemplateMismatch
    }
}

/// Orientation of a page
pub mod page_orientation {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum PageOrientation {
        /// Portrait mode.
        Portrait,
        /// Landscape mode.
        Landscape,
        /// Reverse portrait mode.
        ReversePortrait,
        /// Reverse landscape mode.
        ReverseLandscape
    }
}

/// Unit systems
pub mod unit {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum Unit {
        /// No units.
        None,
        /// Dimensions in points.
        Points,
        /// Dimensions in inches.
        Inch,
        /// Dimensions in millimeters
        MM
    }
}

/// Used to determine the layout of pages on a sheet when printing multiple pages per sheet.
pub mod number_up_layout {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum NumberUpLayout {
        /// 1 2 3 4
        LeftToRight_TopToBottom,
        /// 3 4 2 1
        LeftToRight_BottomToTop,
        /// 2 1 4 3
        RightToLeft_TopToBottom,
        /// 4 3 2 1
        RightToLeft_BottomToTop,
        /// 1 3 2 4
        TopToBottom_LeftToRight,
        /// 3 1 4 2
        TopToBottom_RightToLeft,
        /// 2 4 1 3
        BottomToTop_LeftToRight,
        /// 4 2 3 1
        BottomToTop_RightToLeft
    }
}

/// Used to know which quantity you want to print
pub mod print_pages {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum PrintPages {
        /// All pages.
        All,
        /// Current page.
        Current,
        /// Range of pages.
        Ranges,
        /// Selected pages.
        Selection
    }
}

/// Different types of page to set
pub mod page_set {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum PageSet {
        /// All pages.
        All,
        /// Even pages.
        Even,
        /// Odd pages.
        Odd
    }
}

/// Used to specify the sorting method to be applyed to the recently used resource list.
pub mod recent_sort_type {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum RecentSortType {
        /// Do not sort the returned list of recently used resources.
        None,
        /// Sort the returned list with the most recently used items first.
        MRU,
        /// Sort the returned list with the least recently used items first.
        LRU,
        /// Sort the returned list using a custom sorting function passed using gtk_recent_chooser_set_sort_func().
        Custom
    }
}

/// These flags indicate what parts of a GtkRecentFilterInfo struct are filled or need to be filled.
pub mod recent_filter_flags {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum RecentFilterFlags {
        /// the URI of the file being tested
        URI,
        /// the string that will be used to display the file in the recent chooser
        DisplayName,
        /// the string that will be used to display the file in the recent chooser
        MimeType,
        /// the list of applications that have registered the file
        Application,
        /// the groups to which the file belongs to
        Group,
        /// the number of days elapsed since the file has been registered
        Age
    }
}

/// Kinds of widget-specific help. Used by the ::show-help signal.
pub mod widget_help_type {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum WidgetHelpType {
        /// Tooltip
        WidgetHelpTooltip,
        /// What’s this
        WidgetHelpWhatsThis
    }
}

/// Used to reference the parts of GtkTextView.
pub mod text_window_type {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum TextWindowType {
        /// Window that floats over scrolling areas.
        Widget,
        /// Scrollable text window.
        Text,
        /// Left side border window.
        Left,
        /// Right side border window.
        Right,
        /// Top border window.
        Top,
        // Bottom border window.
        Bottom
    }
}

/// Describes a type of line wrapping.
pub mod wrap_mode {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum WrapMode {
        /// do not wrap lines; just make the text area wider
        None,
        /// wrap text, breaking lines anywhere the cursor can appear (between characters, usually - if you want to be technical, between graphemes, see pango_get_log_attrs())
        Char,
        /// wrap text, breaking lines in between words
        Word,
        /// wrap text, breaking lines in between words, or if that is not enough, also between graphemes
        WordChar
    }
}

/// Used to indicate which grid lines to draw in a tree view.
pub mod tree_view_grid_lines{
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum TreeViewGridLines {
        /// No grid lines.
        None,
        /// Horizontal grid lines.
        Horizontal,
        /// Vertical grid lines.
        Vertical,
        /// Horizontal and vertical grid lines.
        Both
    }
}

/// The sizing method the column uses to determine its width.
/// Please note that AutoSize are inefficient for large views,
/// and can make columns appear choppy.
pub mod tree_view_column_sizing {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum TreeViewColumnSizing {
        /// Columns only get bigger in reaction to changes in the model
        GrowOnly,
        /// Columns resize to be the optimal size everytime the model changes.
        AutoSize,
        /// Columns are a fixed numbers of pixels wide.
        Fixed

    }
}

/// Tells how a cell is to be rendered.
pub mod cell_renderer_state {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum CellRendererState {
        /// The cell is currently selected, and probably has a selection colored background to render to.
        Selected,
        /// The mouse is hovering over the cell.
        Prelit,
        /// The cell is drawn in an insensitive manner.
        Insensitive,
        /// The cell is in a sorted row.
        Sorted,
        /// The cell is in the focus row.
        Focused,
        /// The cell is in a row that can be expanded.
        Expandable,
        /// The cell is in a row that is expanded.
        Expanded
    }
}

/// These flags indicate various properties of a GtkTreeModel.
pub mod tree_model_flags {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum TreeModelFlags {
        /// iterators survive all signals emitted by the tree
        ItersPersist,
        /// the model is a list only, and never has children
        ListOnly,
    }
}