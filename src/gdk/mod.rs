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

pub use self::rt::{
    init,
    get_display_arg_name,
    notify_startup_complete,
    notify_startup_complete_with_id,
    set_allowed_backends,
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

pub use self::enums::modifier_intent::ModifierIntent;
pub use self::enums::modifier_type::ModifierType;
pub use self::enums::{
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
    FrameClockPhase
};

pub use self::widgets::{
    Color,
    RGBA,
    Device,
    Display,
    Atom,
    Screen,
    Rectangle,
    Key,
    Window,
    Visual,
    DeviceManager,
    Cursor,
    Pixbuf,
    Point,
    DisplayManager,
    FrameClock,
    FrameTimings
};

mod events;
mod rt;
pub mod enums;
pub mod widgets;

#[doc(hidden)]
pub mod ffi;
