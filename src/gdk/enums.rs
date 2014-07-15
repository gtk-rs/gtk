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

//! Enumeration used with gdk types

pub use self::modifier_intent::ModifierIntent;
pub use self::modifier_type::ModifierType;

pub mod modifier_intent {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ModifierIntent {
        /// the primary modifier used to invoke menu accelerators.
        PrimaryAccelerator,
        /// the modifier used to invoke context menus. Note that mouse button 3 always triggers context menus. When this modifier is not 0, it additionally triggers context menus when used with mouse button 1.
        ContextMenu,
        /// the modifier used to extend selections using modifier-click or modifier-cursor-key
        ExtendSelection,
        /// the modifier used to modify selections, which in most cases means toggling the clicked item into or out of the selection.
        ModifySelection,
        /// when any of these modifiers is pressed, the key event cannot produce a symbol directly. This is meant to be used for input methods, and for use cases like typeahead search.
        NoTextInput,
        /// the modifier that switches between keyboard groups (AltGr on X11/Windows and Option/Alt on OS X).
        ShiftGroup
    }
}

pub mod modifier_type {
    #[repr(C)]
    #[deriving(Clone, PartialEq, PartialOrd, Show)]
    pub enum ModifierType {
        /// the Shift key.
        ShiftMask,
        /// a Lock key (depending on the modifier mapping of the X server this may either be CapsLock or ShiftLock).
        LockMask,
        /// the Control key.
        ControlMask,
        /// the fourth modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier, but normally it is the Alt key).
        Mod1Mask,
        /// the fifth modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier).
        Mod2Mask,
        /// the sixth modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier).
        Mod3Mask,
        /// the seventh modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier).
        Mod4Mask,
        /// the eighth modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier).
        Mod5Mask,
        /// the first mouse button.
        Button1Mask,
        /// the second mouse button.
        Button2Mask,
        /// the third mouse button.
        Button3Mask,
        /// the fourth mouse button.
        Button4Mask,
        /// the fifth mouse button.
        Button5Mask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved13Mask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved14Mask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved15Mask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved16Mask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved17Mask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved18Mask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved19Mask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved20Mask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved21Mask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved22Mask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved23Mask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved24Mask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved25Mask,
        /// the Super modifier.
        SuperMask,
        /// the Hyper modifier.
        HyperMask,
        /// the Meta modifier.
        MetaMask,
        /// A reserved bit flag; do not use in your own code
        ModifierReserved29Mask,
        /// not used in GDK itself. GTK+ uses it to differentiate between (keyval, modifiers) pairs from key press and release events.
        ReleaseMask,
        /// a mask covering all modifier types.
        ModifierMask
    }
}