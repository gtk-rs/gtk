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

pub use self::color::{Color, RGBA};
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

pub use self::device::{Device};
pub use self::window::{Window};
pub use self::types::{Atom, Screen, Rectangle, Key};

pub use self::enums::modifier_intent::ModifierIntent;
pub use self::enums::modifier_type::ModifierType;
pub use self::enums::WindowType;
pub use self::enums::WindowState;
pub use self::enums::WindowEdge;
pub use self::enums::WindowHints;
pub use self::enums::WindowTypeHint;
pub use self::enums::FullscreenMode;
pub use self::enums::WMDecoration;
pub use self::enums::EventMask;
pub use self::enums::InputSource;
pub use self::enums::InputMode;
pub use self::enums::AxisUse;
pub use self::enums::DeviceType;
pub use self::enums::GrabOwnership;
pub use self::enums::GrabStatus;
pub use self::enums::key;

mod color;
mod events;
mod device;
mod window;
mod types;
pub mod enums;

#[doc(hidden)]
pub mod ffi;
