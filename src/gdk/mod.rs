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

Bindings and wrappers for __GDK__

*/

pub use gdk_ffi as ffi;

pub use self::rt::{
    init,
    get_display_arg_name,
    notify_startup_complete,
    notify_startup_complete_with_id,
    get_program_class,
    set_program_class,
    flush,
    screen_width,
    screen_height,
    screen_width_mm,
    screen_height_mm,
    beep,
    error_trap_push,
    error_trap_pop,
    error_trap_pop_ignored
};
#[cfg(any(feature = "GTK_3_10",feature = "GTK_3_12", feature = "GTK_3_14"))]
pub use self::rt::set_allowed_backends;

pub use self::events::{
    EventType,
    Event,
    EventAny,
    EventExpose,
    EventVisibility,
    EventMotion,
    EventButton,
    EventTouch,
    EventScroll,
    EventKey,
    EventCrossing,
    EventFocus,
    EventConfigure,
    EventProperty,
    EventSelection,
    EventOwnerChange,
    EventProximity,
    EventSetting,
    EventWindowState,
    EventGrabBroken,
    EventDND,
    VisibilityState,
    ScrollDirection,
    NotifyType,
    CrossingMode,
    PropertyState,
    SettingAction,
    OwnerChange
};

pub use gdk_ffi::enums::modifier_intent::ModifierIntent;
pub use gdk_ffi::enums::modifier_type::ModifierType;
pub use gdk_ffi::enums::{
    self,
    WindowType,
    WindowState,
    WindowEdge,
    WindowHints,
    WindowTypeHint,
    FullscreenMode,
    WMDecoration,
    EventMask,
    InputSource,
    InputMode,
    AxisUse,
    DeviceType,
    GrabOwnership,
    GrabStatus,
    key,
    CursorType,
    PixbufAlphaMode,
    PixbufError,
    ColorSpace,
    FrameClockPhase,
    WindowWindowClass,
    Gravity,
    WMFunction,
    DragAction,
    DragProtocol
};

pub use self::widgets::{
    Color,
    RGBA,
    Device,
    Display,
    Atom,
    Screen,
    Rectangle,
    Window,
    Visual,
    DeviceManager,
    Cursor,
    Pixbuf,
    Point,
    DisplayManager,
    WindowAttr,
    DragContext,
    AppLaunchContext
};
#[cfg(any(feature = "GTK_3_8", feature = "GTK_3_10",feature = "GTK_3_12", feature = "GTK_3_14"))]
pub use self::widgets::{
    FrameClock,
    FrameTimings,
};

pub use self::keys::{
    keyval_name
};

mod events;
mod rt;
mod keys;
pub mod widgets;

