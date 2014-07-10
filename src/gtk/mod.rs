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

To implement __GTK+__ inheritance in rust, we implemented gtk superclasses as traits located in `rgtk::gtk::traits::*`. The various widgets implement these traits and live in `rgtk::gtk::*`.

For you're conveniance the various traits are reexported in the `rgtk::*` namespace as `Gtk{trait_name}Trait` so you can just use...

```Rust
extern mod rgtk;
use rgtk::*;
```

...to easily access all the gtk widgets and all traits methods:

```Rust
let button = gtk::Button:new(); // You have access to the struct methods of gtk::Button aswell
                                // as the trait methods from gtk::traits::Button as GtkButtonTrait.
```

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

#[cfg(GTK_3_6)]
#[cfg(GTK_3_8)]
#[cfg(GTK_3_10)]
#[cfg(GTK_3_12)]
pub use self::widgets::{
    MenuButton,
    LevelBar,
};

#[cfg(GTK_3_10)]
#[cfg(GTK_3_12)]
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
pub use self::widgets::{
    FlowBox,
    FlowBoxChild,
    ActionBar,
};

pub use self::widgets::{
    Window,
    Label,
    Button,
    _Box,
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
    MessageDialog,
    NoteBook,
    Overlay,
    Layout,
    FileFilter,
    FileChooserDialog,
    AppInfo,
    AppLaunchContext,
    AppChooserDialog,
    Widget, // FIXME @jeremyletang
    DrawingArea
};

pub use self::enums::{
    window_type,
    WindowType,
    text_direction,
    TextDirection,
    window_position,
    WindowPosition,
    button_box_style,
    ButtonBoxStyle,
    orientation,
    Orientation,
    direction_type,
    DirectionType,
    corner_type,
    CornerType,
    resize_mode,
    ResizeMode,
    border_style,
    BorderStyle,
    sort_type,
    SortType,
    state_flags,
    StateFlags,
    drag_result,
    DragResult,
    accel_flags,
    AccelFlags,
    arrow_placement,
    ArrowPlacement,
    arrow_type,
    ArrowType,
    attach_options,
    AttachOptions,
    delete_type,
    DeleteType,
    expander_style,
    ExpanderStyle,
    im_preedit_style,
    IMPreeditStyle,
    im_status_style,
    IMStatusStyle,
    justification,
    Justification,
    movement_step,
    MovementStep,
    pack_type,
    PackType,
    path_priority_type,
    PathPriorityType,
    path_type,
    PathType,
    policy_type,
    PolicyType,
    position_type,
    PositionType,
    relief_style,
    ReliefStyle,
    scroll_step,
    ScrollStep,
    scroll_type,
    ScrollType,
    selection_mode,
    SelectionMode,
    shadow_type,
    ShadowType,
    state_type,
    StateType,
    toolbar_style,
    ToolbarStyle,
    junction_sides,
    JunctionSides,
    region_flags,
    RegionFlags,
    icon_size,
    IconSize,
    entry_icon_position,
    EntryIconPosition,
    input_hints,
    InputHints,
    input_purpose,
    InputPurpose,
    image_type,
    ImageType,
    spin_type,
    SpinType,
    spin_button_update_policy,
    SpinButtonUpdatePolicy,
    level_bar_mode,
    LevelBarMode,
    calendar_display_options,
    CalendarDisplayOptions,
    message_type,
    MessageType,
    license,
    License,
    response_type,
    ResponseType,
    dialog_flags,
    DialogFlags,
    file_chooser_action,
    FileChooserAction,
    buttons_type,
    ButtonsType,
    stack_transition_type,
    StackTransitionType,
    revealer_transition_type,
    RevealerTransitionType,
    scrollable_policy,
    ScrollablePolicy,
    file_filter_flags,
    FileFilterFlags,
    app_info_create_flags,
    AppInfoCreateFlags
};

pub use self::types::{
    Tooltip,
    WidgetHelpType
};

pub mod traits;
pub mod signals;

mod macros;
mod cast;
mod rt;
mod widgets;
mod enums;
mod types;
mod ffi;
