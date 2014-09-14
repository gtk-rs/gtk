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

/*!

Bindings and wrappers for __GTK__

To implement __GTK+__ inheritance in rust, we implemented gtk superclasses as traits
located in `rgtk::gtk::traits::*`. The various widgets implement these traits and
live in `rgtk::gtk::widgets::*` and are rexported into `rgtk::gtk::*`.

GTK Inheritance in rgtk
======================

You probably know but __Gtk+__ uses its own GObject system: inherited class and interface.

To respect this design I follow a special design on __rgtk__:

* Interface -> Implement them on a trait with only default methods.
* Class -> Implement the construct on the class impl and other methods on a traits.
* Sub-class -> Implement all the methods on the class.

Exemple for GtkOrientable, GtkBox, GtkButtonBox:

GtkOrientable is an interface with all the methods implemented as default method of the trait gtk::traits::Orientable.

GtkBox is a class with constructors implemented on the struct `gtk::Box`, and the other method as default methods of the trait `gtk::traits::Box`. So `gtk::Box` implements `gtk::traits::Orientable` and `gtk::traits::Box`.

GtkButtonBox is a sub-class of GtkBox, the struct `gtk::ButtonBox` implements all the methods of GtkButtonBox and the traits `gtk::traits::Orientable` and `gtk::traits::Box`.

Finally all the gtk widgets implement the trait gtk::traits::Widget.
*/


// These are/should be inlined
pub use self::rt::{
    init,
    main,
    main_quit,
    main_level,
    main_iteration,
    main_iteration_do,
    get_major_version,
    get_minor_version,
    get_micro_version,
    get_binary_age,
    get_interface_age,
    check_version
};


/// GTK Widgets for all versions
pub use self::widgets::{
    Window,
    Label,
    Button,
    Box,
    ButtonBox,
    Frame,
    AspectFrame,
    Fixed,
    Separator,
    FontButton,
    ToggleButton,
    CheckButton,
    ColorButton,
    LinkButton,
    Adjustment,
    ScaleButton,
    VolumeButton,
    Grid,
    EntryBuffer,
    Entry,
    Switch,
    Scale,
    SpinButton,
    Spinner,
    Image,
    ProgressBar,
    Arrow,
    Calendar,
    Alignment,
    Expander,
    Paned,
    InfoBar,
    Toolbar,
    ToolItem,
    SeparatorToolItem,
    ToolButton,
    ToggleToolButton,
    MenuToolButton,
    Dialog,
    AboutDialog,
    ColorChooserDialog,
    FontChooserDialog,
    MessageDialog,
    NoteBook,
    Overlay,
    Layout,
    FileFilter,
    FileChooserDialog,
    AppInfo,
    AppLaunchContext,
    AppChooserDialog,
    DrawingArea,
    PageSetup,
    PaperSize,
    PrintSettings,
    RecentChooserDialog,
    //PageSetupUnixDialog
    RecentInfo,
    RecentFilter,
    RecentFilterInfo,
    RecentData,
    RecentManager,
    TextView,
    TextBuffer,
    TextTagTable,
    ScrolledWindow,
    RadioButton,
    TreeView,
    TreeViewColumn,
    TreePath,
    TreeIter,
    TreeModel,
    MenuItem,
    SeparatorMenuItem,
    CheckMenuItem
};

#[cfg(GTK_3_6)]
#[cfg(GTK_3_8)]
#[cfg(GTK_3_10)]
#[cfg(GTK_3_12)]
/// GTK Widgets for versions since GTK 3.6
pub use self::widgets::{
    MenuButton,
    LevelBar,
};

#[cfg(GTK_3_10)]
#[cfg(GTK_3_12)]
/// GTK Widgets for versions since GTK 3.10
pub use self::widgets::{
    SearchEntry,
    SearchBar,
    Stack,
    StackSwitcher,
    Revealer,
    HeaderBar,
    ListBox,
    ListBoxRow,
};

#[cfg(GTK_3_12)]
/// GTK Widgets for versions since GTK 3.12
pub use self::widgets::{
    FlowBox,
    FlowBoxChild,
    ActionBar,
};

/// GTK Enum members, see submodule `enum` for more info
pub use self::enums::{
    window_type,
    text_direction,
    window_position,
    button_box_style,
    orientation,
    direction_type,
    corner_type,
    resize_mode,
    border_style,
    sort_type,
    state_flags,
    drag_result,
    accel_flags,
    arrow_placement,
    arrow_type,
    attach_options,
    delete_type,
    expander_style,
    im_preedit_style,
    im_status_style,
    justification,
    movement_step,
    pack_type,
    path_priority_type,
    path_type,
    policy_type,
    position_type,
    relief_style,
    scroll_step,
    scroll_type,
    selection_mode,
    shadow_type,
    state_type,
    toolbar_style,
    junction_sides,
    region_flags,
    icon_size,
    entry_icon_position,
    input_hints,
    input_purpose,
    image_type,
    spin_type,
    spin_button_update_policy,
    level_bar_mode,
    calendar_display_options,
    message_type,
    license,
    response_type,
    dialog_flags,
    file_chooser_action,
    buttons_type,
    stack_transition_type,
    revealer_transition_type,
    scrollable_policy,
    file_filter_flags,
    app_info_create_flags,
    size_request_mode,
    align,
    g_connect_flags,
    builder_error,
    page_orientation,
    unit,
    number_up_layout,
    print_pages,
    page_set,
    recent_sort_type,
    recent_filter_flags,
    widget_help_type,
    tree_view_grid_lines,
    tree_view_column_sizing,
    cell_renderer_state,
    tree_model_flags
};

/// GTK Enum types
pub use self::enums::{
    WindowType,
    TextDirection,
    WindowPosition,
    ButtonBoxStyle,
    Orientation,
    DirectionType,
    CornerType,
    ResizeMode,
    BorderStyle,
    SortType,
    StateFlags,
    DragResult,
    AccelFlags,
    ArrowPlacement,
    ArrowType,
    AttachOptions,
    DeleteType,
    ExpanderStyle,
    IMPreeditStyle,
    IMStatusStyle,
    Justification,
    MovementStep,
    PackType,
    PathPriorityType,
    PathType,
    PolicyType,
    PositionType,
    ReliefStyle,
    ScrollStep,
    ScrollType,
    SelectionMode,
    ShadowType,
    StateType,
    ToolbarStyle,
    JunctionSides,
    RegionFlags,
    IconSize,
    EntryIconPosition,
    InputHints,
    InputPurpose,
    ImageType,
    SpinType,
    SpinButtonUpdatePolicy,
    LevelBarMode,
    CalendarDisplayOptions,
    MessageType,
    License,
    ResponseType,
    DialogFlags,
    FileChooserAction,
    ButtonsType,
    StackTransitionType,
    RevealerTransitionType,
    ScrollablePolicy,
    FileFilterFlags,
    AppInfoCreateFlags,
    SizeRequestMode,
    Align,
    GConnectFlags,
    BuilderError,
    PageOrientation,
    Unit,
    NumberUpLayout,
    PrintPages,
    PageSet,
    RecentSortType,
    RecentFilterFlags,
    WidgetHelpType,
    TextWindowType,
    WrapMode,
    TreeViewGridLines,
    TreeViewColumnSizing,
    CellRendererState,
    TreeModelFlags
};

/// GTK various struct
pub use self::types::{
    Tooltip,
};

mod macros;
mod cast;
mod rt;

pub mod traits;
pub mod signals;
pub mod widgets;
pub mod enums;
pub mod types;

#[doc(hidden)]
pub mod ffi;
