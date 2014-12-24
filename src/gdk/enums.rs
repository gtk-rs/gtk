// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version _3 of the License, or
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

pub mod modifier_intent {
    #![allow(non_upper_case_globals)]

    bitflags! {
    #[derive(Show)]
    #[repr(C)]
    flags ModifierIntent: u32 {
        /// the primary modifier used to invoke menu accelerators.,
        const PrimaryAccelerator = 1 << 0,
        /// the modifier used to invoke context menus. Note that mouse button _3 always triggers context menus. When this modifier is not 0, it additionally triggers context menus when used with mouse button 1.,
        const ContextMenu = 1 << 1,
        /// the modifier used to extend selections using modifier-click or modifier-cursor-key,
        const ExtendSelection = 1 << 2,
        /// the modifier used to modify selections, which in most cases means toggling the clicked item into or out of the selection.,
        const ModifySelection = 1 << 3,
        /// when any of these modifiers is pressed, the key event cannot produce a symbol directly. This is meant to be used for input methods, and for use cases like typeahead search.,
        const NoTextInput = 1 << 4,
        /// the modifier that switches between keyboard groups (AltGr on X11/Windows and Option/Alt on OS X).,
        const ShiftGroup = 1 << 5
    }
    }

}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// Describes the kind of window.
pub enum WindowType {
    /// root window; this window has no parent, covers the entire screen, and is created by the window system
    Root,
    /// toplevel window (used to implement GtkWindow)
    TopLevel,
    /// child window (used to implement e.g. GtkEntry)
    Child,
    /// override redirect temporary window (used to implement GtkMenu)
    Temp,
    /// foreign window (see gdk_window_foreign_new())
    Foreign,
    /// offscreen window (see Offscreen Windows).
    Offscreen,
    /// subsurface-based window; This window is visually tied to a toplevel, and is moved/stacked with it.
    /// Currently this window type is only implemented in Wayland.
    SubSurface
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// Specifies the state of a toplevel window.
pub enum WindowState {
    /// the window is not shown.
    WindowStateWithdrawn  = 1 << 0,
    /// the window is minimized.
    WindowStateIconified  = 1 << 1,
    /// the window is maximized.
    WindowStateMaximized  = 1 << 2,
    /// the window is sticky.
    WindowStateSticky     = 1 << 3,
    /// the window is maximized without decorations.
    WindowStateFullscreen = 1 << 4,
    /// the window is kept above other windows.
    WindowStateAbove      = 1 << 5,
    /// the window is kept below other windows.
    WindowStateBelow      = 1 << 6,
    /// the window is presented as focused (with active decorations).
    WindowStateFocused    = 1 << 7,
    /// the window is in a tiled state
    WindowStateTiled      = 1 << 8
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// Indicates which monitor (in a multi-head setup) a window should span over when in fullscreen mode.
pub enum FullscreenMode {
    /// Fullscreen on current monitor only.
    OnCurrentMonitor,
    /// Span across all monitors when fullscreen.
    OnAllMonitors
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// Determines a window edge or corner.
pub enum WindowEdge {
    /// the top left corner.
    NorthWest,
    /// the top edge.
    North,
    /// the top right corner.
    NorthEast,
    /// the left edge.
    West,
    /// the right edge.
    East,
    /// the lower left corner.
    SouthWest,
    /// the lower edge.
    South,
    /// the lower right corner.
    SouthEast
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// Used to indicate which fields of a GdkGeometry struct should be paid attention to. Also, the presence/absence of GDK_HINT_POS,
/// GDK_HINT_USER_POS , and GDK_HINT_USER_SIZE is significant, though they don't directly refer to GdkGeometry fields. GDK_HINT_USER_POS
/// will be set automatically by GtkWindow if you call gtk_window_move(). GDK_HINT_USER_POS and GDK_HINT_USER_SIZE should be set if the
/// user specified a size/position using a --geometry command-line argument; gtk_window_parse_geometry() automatically sets these flags.
pub enum WindowHints {
    /// indicates that the program has positioned the window
    Pos,
    /// min size fields are set
    MinSize,
    /// max size fields are set
    MaxSize,
    /// base size fields are set
    BaseSize,
    /// aspect ratio fields are set
    Aspect,
    /// resize increment fields are set
    ResizeInc,
    /// window gravity field is set
    WinGravity,
    /// indicates that the window’s position was explicitly set by the user
    UserPos,
    /// indicates that the window’s size was explicitly set by the user
    UserSize
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// These are hints for the window manager that indicate what type of function the window has. The window manager can use this when
/// determining decoration and behaviour of the window. The hint must be set before mapping the window.
/// 
/// See the [Extended Window Manager Hints](http://www.freedesktop.org/wiki/Standards/wm-spec/) specification for more details about
/// window types.
pub enum WindowTypeHint {
    /// Normal toplevel window.
    Normal,
    /// Dialog window.
    Dialog,
    /// Window used to implement a menu; GTK+ uses this hint only for torn-off menus, see [GtkTearoffMenuItem](https://developer.gnome.org/gtk2/GtkTearoffMenuItem.html).
    Menu,
    /// Window used to implement toolbars.
    Toolbar,
    /// Window used to display a splash screen during application startup.
    SplashScreen,
    /// Utility windows which are not detached toolbars or dialogs.
    Utility,
    /// Used for creating dock or panel windows.
    Dock,
    /// Used for creating the desktop background window.
    Desktop,
    /// A menu that belongs to a menubar.
    DropdownMenu,
    /// A menu that does not belong to a menubar, e.g. a context menu.
    PopupMenu,
    /// A tooltip.
    ToolTip,
    /// A notification - typically a “bubble” that belongs to a status icon.
    Notification,
    /// A popup from a combo box.
    Combo,
    /// A window that is used to implement a DND cursor.
    DND
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// A set of bit-flags to indicate which events a window is to receive. Most of these masks map onto one or more of the GdkEventType
/// event types above.
/// 
/// GDK_POINTER_MOTION_HINT_MASK is deprecated. It is a special mask to reduce the number of GDK_MOTION_NOTIFY events received. When
/// using GDK_POINTER_MOTION_HINT_MASK, fewer GDK_MOTION_NOTIFY events will be sent, some of which are marked as a hint (the is_hint
/// member is TRUE). To receive more motion events after a motion hint event, the application needs to asks for more, by calling
/// gdk_event_request_motions().
/// 
/// Since GTK 3.8, motion events are already compressed by default, independent of this mechanism. This compression can be disabled with
/// gdk_window_set_event_compression(). See the documentation of that function for details.
/// 
/// If GDK_TOUCH_MASK is enabled, the window will receive touch events from touch-enabled devices. Those will come as sequences of
/// GdkEventTouch with type GDK_TOUCH_UPDATE, enclosed by two events with type GDK_TOUCH_BEGIN and GDK_TOUCH_END (or GDK_TOUCH_CANCEL).
/// gdk_event_get_event_sequence() returns the event sequence for these events, so different sequences may be distinguished.
pub enum EventMask {
    /// receive expose events
    ExposureMask,
    /// receive all pointer motion events
    PointerMotionMask,
    /// deprecated. see the explanation above
    PointerMotionHintMask,
    /// receive pointer motion events while any button is pressed
    ButtonMotionMask,
    /// receive pointer motion events while 1 button is pressed
    Button1MotionMask,
    /// receive pointer motion events while 2 button is pressed
    Button2MotionMask,
    /// receive pointer motion events while 3 button is pressed
    Button3MotionMask,
    /// receive button press events
    ButtonPressMask,
    /// receive button release events
    ButtonReleaseMask,
    /// receive key press events
    KeyPressMask,
    /// receive key release events
    KeyReleaseMask,
    /// receive window enter events
    EnterNotifyMask,
    /// receive window leave events
    LeaveNotifyMask,
    /// receive focus change events
    FocusChangeMask,
    /// receive events about window configuration change
    StructureMask,
    /// receive property change events
    PropertyChangeMask,
    /// receive visibility change events
    VisibilityNotifyMask,
    /// receive proximity in events
    ProximityInMask,
    /// receive proximity out events
    ProximityOutMask,
    /// receive events about window configuration changes of child windows
    SubstructureMask,
    /// receive scroll events
    ScrollMask,
    /// receive touch events. Since 3.4
    TouchMask,
    /// receive smooth scrolling events. Since 3.4
    SmoothScrollMask,
    /// the combination of all the above event masks.
    AllEventsMask
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// These are hints originally defined by the Motif toolkit. The window manager can use them when determining how to decorate the
/// window. The hint must be set before mapping the window.
pub enum WMDecoration {
    /// all decorations should be applied.
    All,
    /// a frame should be drawn around the window.
    Border,
    /// the frame should have resize handles.
    ResizeH,
    /// a titlebar should be placed above the window.
    Title,
    /// a button for opening a menu should be included.
    Menu,
    /// a minimize button should be included.
    Minimize,
    /// a maximize button should be included.
    Maximize
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// An enumeration describing the type of an input device in general terms.
pub enum InputSource {
    /// the device is a mouse. (This will be reported for the core pointer, even if it is something else, such as a trackball.)
    Mouse,
    /// the device is a stylus of a graphics tablet or similar device.
    Pen,
    /// the device is an eraser. Typically, this would be the other end of a stylus on a graphics tablet
    Eraser,
    /// the device is a graphics tablet “puck” or similar device.
    Cursor,
    /// the device is a keyboard.
    Keyboard,
    /// the device is a direct-input touch device, such as a touchscreen or tablet. This device type has been added in 3.4.
    TouchScreen,
    /// the device is an indirect touch device, such as a touchpad. This device type has been added in 3.4.
    TouchPad
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// An enumeration that describes the mode of an input device.
pub enum InputMode {
    /// the device is disabled and will not report any events.
    Disabled,
    /// the device is enabled. The device’s coordinate space maps to the entire screen.
    Screen,
    /// the device is enabled. The device’s coordinate space is mapped to a single window. The manner in which this window is chosen is
    /// undefined, but it will typically be the same way in which the focus window for key events is determined.
    Window
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// An enumeration describing the way in which a device axis (valuator) maps onto the predefined valuator types that GTK+ understands.
pub enum AxisUse {
    /// the axis is ignored.
    Ignore,
    /// the axis is used as the x axis.
    X,
    /// the axis is used as the y axis.
    Y,
    /// the axis is used for pressure information.
    Pressure,
    /// the axis is used for x tilt information.
    XTilt,
    /// the axis is used for y tilt information.
    YTilt,
    /// the axis is used for wheel information.
    Wheel,
    /// a constant equal to the numerically highest axis value.
    Last
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// Indicates the device type. See [above](https://developer.gnome.org/gdk3/stable/GdkDeviceManager.html#GdkDeviceManager.description)
/// for more information about the meaning of these device types.
pub enum DeviceType {
    /// Device is a master (or virtual) device. There will be an associated focus indicator on the screen.
    Master,
    /// Device is a slave (or physical) device.
    Slave,
    /// Device is a physical device, currently not attached to any virtual device.
    Floating
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// Defines how device grabs interact with other devices.
pub enum GrabOwnership {
    /// All other devices’ events are allowed.
    None,
    /// Other devices’ events are blocked for the grab window.
    Window,
    /// Other devices’ events are blocked for the whole application.
    Application
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// Returned by gdk_device_grab(), gdk_pointer_grab() and gdk_keyboard_grab() to indicate success or the reason for the failure of
/// the grab attempt.
pub enum GrabStatus {
    /// the resource was successfully grabbed.
    Success,
    /// the resource is actively grabbed by another client.
    AlreadyGrabbed,
    /// the resource was grabbed more recently than the specified time.
    InvalidTime,
    /// the grab window or the confine_to window are not viewable.
    NotViewable,
    /// the resource is frozen by an active grab of another client.
    Frozen
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// The standard cursors available.
/// For more information, please go [here](https://developer.gnome.org/gdk3/stable/gdk3-Cursors.html#GdkCursorType).
pub enum CursorType {
    X_Cursor,
    Arrow,
    ArrowDown,
    ArrowUp,
    Boat,
    Bogosity,
    BottomLeftCorner,
    BottomRightCorner,
    BottomSide,
    BottomTee,
    BoxSpiral,
    CenterPtr,
    Circle,
    Clock,
    CoffeeMug,
    Cross,
    CrossReverse,
    Crosshair,
    DiamondCross,
    Dot,
    Dotbox,
    DoubleArrow,
    DraftLarge,
    DraftSmall,
    DrapedBox,
    Exchange,
    Fleur,
    Gobbler,
    Gumby,
    Hand1,
    Hand2,
    Heart,
    Icon,
    IronCross,
    LeftPtr,
    LeftSide,
    LeftTee,
    LeftButton,
    LL_Angle,
    LR_Angle,
    Man,
    MiddleButton,
    Mouse,
    Pencil,
    Pirate,
    Plus,
    QuestionArrow,
    RightPtr,
    RightSide,
    RightTee,
    RightButton,
    RTL_Logo,
    SailBoat,
    SB_DownArrow,
    SB_H_DoubleArrow,
    SB_LeftArrow,
    SB_RightArrow,
    SB_UpArrow,
    SB_V_DoubleArrow,
    Shuttle,
    Sizing,
    Spider,
    Spraycan,
    Star,
    Target,
    TCross,
    TopLeftArrow,
    TopLeftCorner,
    TopRightCorner,
    TopSide,
    TopTee,
    Trek,
    UL_Angle,
    Umbrella,
    UR_Angle,
    Watch,
    XTerm,
    LastCursor,
    BlankCursor,
    CursorIsPixmap
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// This enumeration defines the color spaces that are supported by the &gdk-pixbuf; library. Currently only RGB is supported.
pub enum ColorSpace {
    RGB,
    /// doesn't exist in gtk+, don't use it !
    Unused
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// An error code in the GDK_PIXBUF_ERROR domain. Many &gdk-pixbuf; operations can cause errors in this domain, or in the G_FILE_ERROR domain.
pub enum PixbufError {
    /// An image file was broken somehow.
    CorruptImage,
    /// Not enough memory.
    InsufficientMemory,
    /// A bad option was passed to a pixbuf save module.
    BadOption,
    /// Unknown image type.
    UnknownType,
    /// Don't know how to perform the given operation on the type of image at hand.
    UnsupportedOperation,
    /// Generic failure code, something went wrong.
    Failed
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// These values can be passed to gdk_pixbuf_render_to_drawable_alpha() to control how the alpha channel of an image should be handled.
/// This function can create a bilevel clipping mask (black and white) and use it while painting the image. In the future, when the X
/// Window System gets an alpha channel extension, it will be possible to do full alpha compositing onto arbitrary drawables. For now
/// both cases fall back to a bilevel clipping mask.
pub enum PixbufAlphaMode {
    /// A bilevel clipping mask (black and white) will be created and used to draw the image. Pixels below 0.5 opacity will be considered
    /// fully transparent, and all others will be considered fully opaque.
    AlphaBiLevel,
    /// For now falls back to GDK_PIXBUF_ALPHA_BILEVEL. In the future it will do full alpha compositing.
    AlphaFull
}

#[repr(C)]
#[deriving(Clone, PartialEq, PartialOrd, Show, Copy)]
/// GdkFrameClockPhase is used to represent the different paint clock phases that can be requested. The elements of the enumeration
/// correspond to the signals of GdkFrameClock.
/// 
/// !!! SINCE 3.8 !!!
pub enum FrameClockPhase {
    /// no phase
    None,
    /// corresponds to GdkFrameClock::flush-events. Should not be handled by applications.
    FlushEvents,
    /// corresponds to GdkFrameClock::before-paint. Should not be handled by applications.
    BeforePaint,
    /// corresponds to GdkFrameClock::update.
    Update,
    /// corresponds to GdkFrameClock::layout.
    Layout,
    /// corresponds to GdkFrameClock::paint.
    Paint,
    /// corresponds to GdkFrameClock::resume-events. Should not be handled by applications.
    ResumeEvents,
    /// corresponds to GdkFrameClock::after-paint. Should not be handled by applications.
    AfterPaint
}

pub mod modifier_type {
    #![allow(non_upper_case_globals)]

    bitflags! {
    #[repr(C)]
    #[derive(Show)]
    flags ModifierType: u32 {
        /// the Shift key.,
        const ShiftMask              = 1 << 0,
        /// a Lock key (depending on the modifier mapping of the X server this may either be CapsLock or ShiftLock).,
        const LockMask               = 1 << 1,
        /// the Control key.,
        const ControlMask            = 1 << 2,
        /// the fourth modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier, but normally it is the Alt key).,
        const Mod1Mask               = 1 << 3,
        /// the fifth modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier).,
        const Mod2Mask               = 1 << 4,
        /// the sixth modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier).,
        const Mod3Mask               = 1 << 5,
        /// the seventh modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier).,
        const Mod4Mask               = 1 << 6,
        /// the eighth modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier).,
        const Mod5Mask               = 1 << 7,
        /// the first mouse button.,
        const Button1Mask               = 1 << 8,
        /// the second mouse button.,
        const Button2Mask               = 1 << 9,
        /// the third mouse button.,
        const Button3Mask               = 1 << 10,
        /// the fourth mouse button.,
        const Button4Mask               = 1 << 11,
        /// the fifth mouse button.,
        const Button5Mask               = 1 << 12,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved13Mask               = 1 << 13,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved14Mask               = 1 << 14,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved15Mask               = 1 << 15,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved16Mask               = 1 << 16,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved17Mask               = 1 << 17,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved18Mask               = 1 << 18,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved19Mask               = 1 << 19,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved20Mask               = 1 << 20,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved21Mask               = 1 << 21,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved22Mask               = 1 << 22,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved23Mask               = 1 << 23,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved24Mask               = 1 << 24,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved25Mask               = 1 << 25,
        /// the Super modifier.,
        const SuperMask               = 1 << 26,
        /// the Hyper modifier.,
        const HyperMask               = 1 << 27,
        /// the Meta modifier.,
        const MetaMask               = 1 << 28,
        /// A reserved bit flag; do not use in your own code,
        const ModifierReserved29Mask               = 1 << 29,
        /// not used in GDK itself. GTK+ uses it to differentiate between (keyval, modifiers) pairs from key press and release events.,
        const ReleaseMask               = 1 << 30,
        /// a mask covering all modifier types.,
        const ModifierMask               = 0x5c001fff,
    }
}

}

/// Corresponding values to keys
#[allow(non_upper_case_globals)]
pub mod key {
    use gdk::Key;

    pub const VoidSymbol : Key = 0xffffff;
    pub const BackSpace : Key = 0xff08;
    pub const Tab : Key = 0xff09;
    pub const Linefeed : Key = 0xff0a;
    pub const Clear : Key = 0xff0b;
    pub const Return : Key = 0xff0d;
    pub const Pause : Key = 0xff13;
    pub const Scroll_Lock : Key = 0xff14;
    pub const Sys_Req : Key = 0xff15;
    pub const Escape : Key = 0xff1b;
    pub const Delete : Key = 0xffff;
    pub const Multi_key : Key = 0xff20;
    pub const Codeinput : Key = 0xff37;
    pub const SingleCandidate : Key = 0xff3c;
    pub const MultipleCandidate : Key = 0xff3d;
    pub const PreviousCandidate : Key = 0xff3e;
    pub const Kanji : Key = 0xff21;
    pub const Muhenkan : Key = 0xff22;
    pub const Henkan_Mode : Key = 0xff23;
    pub const Henkan : Key = 0xff23;
    pub const Romaji : Key = 0xff24;
    pub const Hiragana : Key = 0xff25;
    pub const Katakana : Key = 0xff26;
    pub const Hiragana_Katakana : Key = 0xff27;
    pub const Zenkaku : Key = 0xff28;
    pub const Hankaku : Key = 0xff29;
    pub const Zenkaku_Hankaku : Key = 0xff2a;
    pub const Touroku : Key = 0xff2b;
    pub const Massyo : Key = 0xff2c;
    pub const Kana_Lock : Key = 0xff2d;
    pub const Kana_Shift : Key = 0xff2e;
    pub const Eisu_Shift : Key = 0xff2f;
    pub const Eisu_toggle : Key = 0xff30;
    pub const Kanji_Bangou : Key = 0xff37;
    pub const Zen_Koho : Key = 0xff3d;
    pub const Mae_Koho : Key = 0xff3e;
    pub const Home : Key = 0xff50;
    pub const Left : Key = 0xff51;
    pub const Up : Key = 0xff52;
    pub const Right : Key = 0xff53;
    pub const Down : Key = 0xff54;
    pub const Prior : Key = 0xff55;
    pub const Page_Up : Key = 0xff55;
    pub const Next : Key = 0xff56;
    pub const Page_Down : Key = 0xff56;
    pub const End : Key = 0xff57;
    pub const Begin : Key = 0xff58;
    pub const Select : Key = 0xff60;
    pub const Print : Key = 0xff61;
    pub const Execute : Key = 0xff62;
    pub const Insert : Key = 0xff63;
    pub const Undo : Key = 0xff65;
    pub const Redo : Key = 0xff66;
    pub const Menu : Key = 0xff67;
    pub const Find : Key = 0xff68;
    pub const Cancel : Key = 0xff69;
    pub const Help : Key = 0xff6a;
    pub const Break : Key = 0xff6b;
    pub const Mode_switch : Key = 0xff7e;
    pub const script_switch : Key = 0xff7e;
    pub const Num_Lock : Key = 0xff7f;
    pub const KP_Space : Key = 0xff80;
    pub const KP_Tab : Key = 0xff89;
    pub const KP_Enter : Key = 0xff8d;
    pub const KP_F1 : Key = 0xff91;
    pub const KP_F2 : Key = 0xff92;
    pub const KP_F3 : Key = 0xff93;
    pub const KP_F4 : Key = 0xff94;
    pub const KP_Home : Key = 0xff95;
    pub const KP_Left : Key = 0xff96;
    pub const KP_Up : Key = 0xff97;
    pub const KP_Right : Key = 0xff98;
    pub const KP_Down : Key = 0xff99;
    pub const KP_Prior : Key = 0xff9a;
    pub const KP_Page_Up : Key = 0xff9a;
    pub const KP_Next : Key = 0xff9b;
    pub const KP_Page_Down : Key = 0xff9b;
    pub const KP_End : Key = 0xff9c;
    pub const KP_Begin : Key = 0xff9d;
    pub const KP_Insert : Key = 0xff9e;
    pub const KP_Delete : Key = 0xff9f;
    pub const KP_Equal : Key = 0xffbd;
    pub const KP_Multiply : Key = 0xffaa;
    pub const KP_Add : Key = 0xffab;
    pub const KP_Separator : Key = 0xffac;
    pub const KP_Subtract : Key = 0xffad;
    pub const KP_Decimal : Key = 0xffae;
    pub const KP_Divide : Key = 0xffaf;
    pub const KP_0 : Key = 0xffb0;
    pub const KP_1 : Key = 0xffb1;
    pub const KP_2 : Key = 0xffb2;
    pub const KP_3 : Key = 0xffb3;
    pub const KP_4 : Key = 0xffb4;
    pub const KP_5 : Key = 0xffb5;
    pub const KP_6 : Key = 0xffb6;
    pub const KP_7 : Key = 0xffb7;
    pub const KP_8 : Key = 0xffb8;
    pub const KP_9 : Key = 0xffb9;
    pub const F1 : Key = 0xffbe;
    pub const F2 : Key = 0xffbf;
    pub const F3 : Key = 0xffc0;
    pub const F4 : Key = 0xffc1;
    pub const F5 : Key = 0xffc2;
    pub const F6 : Key = 0xffc3;
    pub const F7 : Key = 0xffc4;
    pub const F8 : Key = 0xffc5;
    pub const F9 : Key = 0xffc6;
    pub const F10 : Key = 0xffc7;
    pub const F11 : Key = 0xffc8;
    pub const L1 : Key = 0xffc8;
    pub const F12 : Key = 0xffc9;
    pub const L2 : Key = 0xffc9;
    pub const F13 : Key = 0xffca;
    pub const L3 : Key = 0xffca;
    pub const F14 : Key = 0xffcb;
    pub const L4 : Key = 0xffcb;
    pub const F15 : Key = 0xffcc;
    pub const L5 : Key = 0xffcc;
    pub const F16 : Key = 0xffcd;
    pub const L6 : Key = 0xffcd;
    pub const F17 : Key = 0xffce;
    pub const L7 : Key = 0xffce;
    pub const F18 : Key = 0xffcf;
    pub const L8 : Key = 0xffcf;
    pub const F19 : Key = 0xffd0;
    pub const L9 : Key = 0xffd0;
    pub const F20 : Key = 0xffd1;
    pub const L10 : Key = 0xffd1;
    pub const F21 : Key = 0xffd2;
    pub const R1 : Key = 0xffd2;
    pub const F22 : Key = 0xffd3;
    pub const R2 : Key = 0xffd3;
    pub const F23 : Key = 0xffd4;
    pub const R3 : Key = 0xffd4;
    pub const F24 : Key = 0xffd5;
    pub const R4 : Key = 0xffd5;
    pub const F25 : Key = 0xffd6;
    pub const R5 : Key = 0xffd6;
    pub const F26 : Key = 0xffd7;
    pub const R6 : Key = 0xffd7;
    pub const F27 : Key = 0xffd8;
    pub const R7 : Key = 0xffd8;
    pub const F28 : Key = 0xffd9;
    pub const R8 : Key = 0xffd9;
    pub const F29 : Key = 0xffda;
    pub const R9 : Key = 0xffda;
    pub const F30 : Key = 0xffdb;
    pub const R10 : Key = 0xffdb;
    pub const F31 : Key = 0xffdc;
    pub const R11 : Key = 0xffdc;
    pub const F32 : Key = 0xffdd;
    pub const R12 : Key = 0xffdd;
    pub const F33 : Key = 0xffde;
    pub const R13 : Key = 0xffde;
    pub const F34 : Key = 0xffdf;
    pub const R14 : Key = 0xffdf;
    pub const F35 : Key = 0xffe0;
    pub const R15 : Key = 0xffe0;
    pub const Shift_L : Key = 0xffe1;
    pub const Shift_R : Key = 0xffe2;
    pub const Control_L : Key = 0xffe3;
    pub const Control_R : Key = 0xffe4;
    pub const Caps_Lock : Key = 0xffe5;
    pub const Shift_Lock : Key = 0xffe6;
    pub const Meta_L : Key = 0xffe7;
    pub const Meta_R : Key = 0xffe8;
    pub const Alt_L : Key = 0xffe9;
    pub const Alt_R : Key = 0xffea;
    pub const Super_L : Key = 0xffeb;
    pub const Super_R : Key = 0xffec;
    pub const Hyper_L : Key = 0xffed;
    pub const Hyper_R : Key = 0xffee;
    pub const ISO_Lock : Key = 0xfe01;
    pub const ISO_Level2_Latch : Key = 0xfe02;
    pub const ISO_Level3_Shift : Key = 0xfe03;
    pub const ISO_Level3_Latch : Key = 0xfe04;
    pub const ISO_Level3_Lock : Key = 0xfe05;
    pub const ISO_Level5_Shift : Key = 0xfe11;
    pub const ISO_Level5_Latch : Key = 0xfe12;
    pub const ISO_Level5_Lock : Key = 0xfe13;
    pub const ISO_Group_Shift : Key = 0xff7e;
    pub const ISO_Group_Latch : Key = 0xfe06;
    pub const ISO_Group_Lock : Key = 0xfe07;
    pub const ISO_Next_Group : Key = 0xfe08;
    pub const ISO_Next_Group_Lock : Key = 0xfe09;
    pub const ISO_Prev_Group : Key = 0xfe0a;
    pub const ISO_Prev_Group_Lock : Key = 0xfe0b;
    pub const ISO_First_Group : Key = 0xfe0c;
    pub const ISO_First_Group_Lock : Key = 0xfe0d;
    pub const ISO_Last_Group : Key = 0xfe0e;
    pub const ISO_Last_Group_Lock : Key = 0xfe0f;
    pub const ISO_Left_Tab : Key = 0xfe20;
    pub const ISO_Move_Line_Up : Key = 0xfe21;
    pub const ISO_Move_Line_Down : Key = 0xfe22;
    pub const ISO_Partial_Line_Up : Key = 0xfe23;
    pub const ISO_Partial_Line_Down : Key = 0xfe24;
    pub const ISO_Partial_Space_Left : Key = 0xfe25;
    pub const ISO_Partial_Space_Right : Key = 0xfe26;
    pub const ISO_Set_Margin_Left : Key = 0xfe27;
    pub const ISO_Set_Margin_Right : Key = 0xfe28;
    pub const ISO_Release_Margin_Left : Key = 0xfe29;
    pub const ISO_Release_Margin_Right : Key = 0xfe2a;
    pub const ISO_Release_Both_Margins : Key = 0xfe2b;
    pub const ISO_Fast_Cursor_Left : Key = 0xfe2c;
    pub const ISO_Fast_Cursor_Right : Key = 0xfe2d;
    pub const ISO_Fast_Cursor_Up : Key = 0xfe2e;
    pub const ISO_Fast_Cursor_Down : Key = 0xfe2f;
    pub const ISO_Continuous_Underline : Key = 0xfe30;
    pub const ISO_Discontinuous_Underline : Key = 0xfe31;
    pub const ISO_Emphasize : Key = 0xfe32;
    pub const ISO_Center_Object : Key = 0xfe33;
    pub const ISO_Enter : Key = 0xfe34;
    pub const dead_grave : Key = 0xfe50;
    pub const dead_acute : Key = 0xfe51;
    pub const dead_circumflex : Key = 0xfe52;
    pub const dead_tilde : Key = 0xfe53;
    pub const dead_perispomeni : Key = 0xfe53;
    pub const dead_macron : Key = 0xfe54;
    pub const dead_breve : Key = 0xfe55;
    pub const dead_abovedot : Key = 0xfe56;
    pub const dead_diaeresis : Key = 0xfe57;
    pub const dead_abovering : Key = 0xfe58;
    pub const dead_doubleacute : Key = 0xfe59;
    pub const dead_caron : Key = 0xfe5a;
    pub const dead_cedilla : Key = 0xfe5b;
    pub const dead_ogonek : Key = 0xfe5c;
    pub const dead_iota : Key = 0xfe5d;
    pub const dead_voiced_sound : Key = 0xfe5e;
    pub const dead_semivoiced_sound : Key = 0xfe5f;
    pub const dead_belowdot : Key = 0xfe60;
    pub const dead_hook : Key = 0xfe61;
    pub const dead_horn : Key = 0xfe62;
    pub const dead_stroke : Key = 0xfe63;
    pub const dead_abovecomma : Key = 0xfe64;
    pub const dead_psili : Key = 0xfe64;
    pub const dead_abovereversedcomma : Key = 0xfe65;
    pub const dead_dasia : Key = 0xfe65;
    pub const dead_doublegrave : Key = 0xfe66;
    pub const dead_belowring : Key = 0xfe67;
    pub const dead_belowmacron : Key = 0xfe68;
    pub const dead_belowcircumflex : Key = 0xfe69;
    pub const dead_belowtilde : Key = 0xfe6a;
    pub const dead_belowbreve : Key = 0xfe6b;
    pub const dead_belowdiaeresis : Key = 0xfe6c;
    pub const dead_invertedbreve : Key = 0xfe6d;
    pub const dead_belowcomma : Key = 0xfe6e;
    pub const dead_currency : Key = 0xfe6f;
    pub const dead_a : Key = 0xfe80;
    pub const dead_A : Key = 0xfe81;
    pub const dead_e : Key = 0xfe82;
    pub const dead_E : Key = 0xfe83;
    pub const dead_i : Key = 0xfe84;
    pub const dead_I : Key = 0xfe85;
    pub const dead_o : Key = 0xfe86;
    pub const dead_O : Key = 0xfe87;
    pub const dead_u : Key = 0xfe88;
    pub const dead_U : Key = 0xfe89;
    pub const dead_small_schwa : Key = 0xfe8a;
    pub const dead_capital_schwa : Key = 0xfe8b;
    pub const dead_greek : Key = 0xfe8c;
    pub const First_Virtual_Screen : Key = 0xfed0;
    pub const Prev_Virtual_Screen : Key = 0xfed1;
    pub const Next_Virtual_Screen : Key = 0xfed2;
    pub const Last_Virtual_Screen : Key = 0xfed4;
    pub const Terminate_Server : Key = 0xfed5;
    pub const AccessX_Enable : Key = 0xfe70;
    pub const AccessX_Feedback_Enable : Key = 0xfe71;
    pub const RepeatKeys_Enable : Key = 0xfe72;
    pub const SlowKeys_Enable : Key = 0xfe73;
    pub const BounceKeys_Enable : Key = 0xfe74;
    pub const StickyKeys_Enable : Key = 0xfe75;
    pub const MouseKeys_Enable : Key = 0xfe76;
    pub const MouseKeys_Accel_Enable : Key = 0xfe77;
    pub const Overlay1_Enable : Key = 0xfe78;
    pub const Overlay2_Enable : Key = 0xfe79;
    pub const AudibleBell_Enable : Key = 0xfe7a;
    pub const Pointer_Left : Key = 0xfee0;
    pub const Pointer_Right : Key = 0xfee1;
    pub const Pointer_Up : Key = 0xfee2;
    pub const Pointer_Down : Key = 0xfee3;
    pub const Pointer_UpLeft : Key = 0xfee4;
    pub const Pointer_UpRight : Key = 0xfee5;
    pub const Pointer_DownLeft : Key = 0xfee6;
    pub const Pointer_DownRight : Key = 0xfee7;
    pub const Pointer_Button_Dflt : Key = 0xfee8;
    pub const Pointer_Button1 : Key = 0xfee9;
    pub const Pointer_Button2 : Key = 0xfeea;
    pub const Pointer_Button3 : Key = 0xfeeb;
    pub const Pointer_Button4 : Key = 0xfeec;
    pub const Pointer_Button5 : Key = 0xfeed;
    pub const Pointer_DblClick_Dflt : Key = 0xfeee;
    pub const Pointer_DblClick1 : Key = 0xfeef;
    pub const Pointer_DblClick2 : Key = 0xfef0;
    pub const Pointer_DblClick3 : Key = 0xfef1;
    pub const Pointer_DblClick4 : Key = 0xfef2;
    pub const Pointer_DblClick5 : Key = 0xfef3;
    pub const Pointer_Drag_Dflt : Key = 0xfef4;
    pub const Pointer_Drag1 : Key = 0xfef5;
    pub const Pointer_Drag2 : Key = 0xfef6;
    pub const Pointer_Drag3 : Key = 0xfef7;
    pub const Pointer_Drag4 : Key = 0xfef8;
    pub const Pointer_Drag5 : Key = 0xfefd;
    pub const Pointer_EnableKeys : Key = 0xfef9;
    pub const Pointer_Accelerate : Key = 0xfefa;
    pub const Pointer_DfltBtnNext : Key = 0xfefb;
    pub const Pointer_DfltBtnPrev : Key = 0xfefc;
    pub const ch : Key = 0xfea0;
    pub const Ch : Key = 0xfea1;
    pub const CH : Key = 0xfea2;
    pub const c_h : Key = 0xfea3;
    pub const C_h : Key = 0xfea4;
    pub const C_H : Key = 0xfea5;
    pub const _3270_Duplicate : Key = 0xfd01;
    pub const _3270_FieldMark : Key = 0xfd02;
    pub const _3270_Right2 : Key = 0xfd03;
    pub const _3270_Left2 : Key = 0xfd04;
    pub const _3270_BackTab : Key = 0xfd05;
    pub const _3270_EraseEOF : Key = 0xfd06;
    pub const _3270_EraseInput : Key = 0xfd07;
    pub const _3270_Reset : Key = 0xfd08;
    pub const _3270_Quit : Key = 0xfd09;
    pub const _3270_PA1 : Key = 0xfd0a;
    pub const _3270_PA2 : Key = 0xfd0b;
    pub const _3270_PA3 : Key = 0xfd0c;
    pub const _3270_Test : Key = 0xfd0d;
    pub const _3270_Attn : Key = 0xfd0e;
    pub const _3270_CursorBlink : Key = 0xfd0f;
    pub const _3270_AltCursor : Key = 0xfd10;
    pub const _3270_KeyClick : Key = 0xfd11;
    pub const _3270_Jump : Key = 0xfd12;
    pub const _3270_Ident : Key = 0xfd13;
    pub const _3270_Rule : Key = 0xfd14;
    pub const _3270_Copy : Key = 0xfd15;
    pub const _3270_Play : Key = 0xfd16;
    pub const _3270_Setup : Key = 0xfd17;
    pub const _3270_Record : Key = 0xfd18;
    pub const _3270_ChangeScreen : Key = 0xfd19;
    pub const _3270_DeleteWord : Key = 0xfd1a;
    pub const _3270_ExSelect : Key = 0xfd1b;
    pub const _3270_CursorSelect : Key = 0xfd1c;
    pub const _3270_PrintScreen : Key = 0xfd1d;
    pub const _3270_Enter : Key = 0xfd1e;
    pub const space : Key = 0x020;
    pub const exclam : Key = 0x021;
    pub const quotedbl : Key = 0x022;
    pub const numbersign : Key = 0x023;
    pub const dollar : Key = 0x024;
    pub const percent : Key = 0x025;
    pub const ampersand : Key = 0x026;
    pub const apostrophe : Key = 0x027;
    pub const quoteright : Key = 0x027;
    pub const parenleft : Key = 0x028;
    pub const parenright : Key = 0x029;
    pub const asterisk : Key = 0x02a;
    pub const plus : Key = 0x02b;
    pub const comma : Key = 0x02c;
    pub const minus : Key = 0x02d;
    pub const period : Key = 0x02e;
    pub const slash : Key = 0x02f;
    pub const _0 : Key = 0x030; // equivalent to keypad '0'
    pub const _1 : Key = 0x031; // equivalent to keypad '1'
    pub const _2 : Key = 0x032; // equivalent to keypad '2'
    pub const _3 : Key = 0x033; // equivalent to keypad '3'
    pub const _4 : Key = 0x034; // equivalent to keypad '4'
    pub const _5 : Key = 0x035; // equivalent to keypad '5'
    pub const _6 : Key = 0x036; // equivalent to keypad '6'
    pub const _7 : Key = 0x037; // equivalent to keypad '7'
    pub const _8 : Key = 0x038; // equivalent to keypad '8'
    pub const _9 : Key = 0x039; // equivalent to keypad '9'
    pub const colon : Key = 0x03a;
    pub const semicolon : Key = 0x03b;
    pub const less : Key = 0x03c;
    pub const equal : Key = 0x03d;
    pub const greater : Key = 0x03e;
    pub const question : Key = 0x03f;
    pub const at : Key = 0x040;
    pub const A : Key = 0x041;
    pub const B : Key = 0x042;
    pub const C : Key = 0x043;
    pub const D : Key = 0x044;
    pub const E : Key = 0x045;
    pub const F : Key = 0x046;
    pub const G : Key = 0x047;
    pub const H : Key = 0x048;
    pub const I : Key = 0x049;
    pub const J : Key = 0x04a;
    pub const K : Key = 0x04b;
    pub const L : Key = 0x04c;
    pub const M : Key = 0x04d;
    pub const N : Key = 0x04e;
    pub const O : Key = 0x04f;
    pub const P : Key = 0x050;
    pub const Q : Key = 0x051;
    pub const R : Key = 0x052;
    pub const S : Key = 0x053;
    pub const T : Key = 0x054;
    pub const U : Key = 0x055;
    pub const V : Key = 0x056;
    pub const W : Key = 0x057;
    pub const X : Key = 0x058;
    pub const Y : Key = 0x059;
    pub const Z : Key = 0x05a;
    pub const bracketleft : Key = 0x05b;
    pub const backslash : Key = 0x05c;
    pub const bracketright : Key = 0x05d;
    pub const asciicircum : Key = 0x05e;
    pub const underscore : Key = 0x05f;
    pub const grave : Key = 0x060;
    pub const quoteleft : Key = 0x060;
    pub const a : Key = 0x061;
    pub const b : Key = 0x062;
    pub const c : Key = 0x063;
    pub const d : Key = 0x064;
    pub const e : Key = 0x065;
    pub const f : Key = 0x066;
    pub const g : Key = 0x067;
    pub const h : Key = 0x068;
    pub const i : Key = 0x069;
    pub const j : Key = 0x06a;
    pub const k : Key = 0x06b;
    pub const l : Key = 0x06c;
    pub const m : Key = 0x06d;
    pub const n : Key = 0x06e;
    pub const o : Key = 0x06f;
    pub const p : Key = 0x070;
    pub const q : Key = 0x071;
    pub const r : Key = 0x072;
    pub const s : Key = 0x073;
    pub const t : Key = 0x074;
    pub const u : Key = 0x075;
    pub const v : Key = 0x076;
    pub const w : Key = 0x077;
    pub const x : Key = 0x078;
    pub const y : Key = 0x079;
    pub const z : Key = 0x07a;
    pub const braceleft : Key = 0x07b;
    pub const bar : Key = 0x07c;
    pub const braceright : Key = 0x07d;
    pub const asciitilde : Key = 0x07e;
    pub const nobreakspace : Key = 0x0a0;
    pub const exclamdown : Key = 0x0a1;
    pub const cent : Key = 0x0a2;
    pub const sterling : Key = 0x0a3;
    pub const currency : Key = 0x0a4;
    pub const yen : Key = 0x0a5;
    pub const brokenbar : Key = 0x0a6;
    pub const section : Key = 0x0a7;
    pub const diaeresis : Key = 0x0a8;
    pub const copyright : Key = 0x0a9;
    pub const ordfeminine : Key = 0x0aa;
    pub const guillemotleft : Key = 0x0ab;
    pub const notsign : Key = 0x0ac;
    pub const hyphen : Key = 0x0ad;
    pub const registered : Key = 0x0ae;
    pub const macron : Key = 0x0af;
    pub const degree : Key = 0x0b0;
    pub const plusminus : Key = 0x0b1;
    pub const twosuperior : Key = 0x0b2;
    pub const threesuperior : Key = 0x0b3;
    pub const acute : Key = 0x0b4;
    pub const mu : Key = 0x0b5;
    pub const paragraph : Key = 0x0b6;
    pub const periodcentered : Key = 0x0b7;
    pub const cedilla : Key = 0x0b8;
    pub const onesuperior : Key = 0x0b9;
    pub const masculine : Key = 0x0ba;
    pub const guillemotright : Key = 0x0bb;
    pub const onequarter : Key = 0x0bc;
    pub const onehalf : Key = 0x0bd;
    pub const threequarters : Key = 0x0be;
    pub const questiondown : Key = 0x0bf;
    pub const Agrave : Key = 0x0c0;
    pub const Aacute : Key = 0x0c1;
    pub const Acircumflex : Key = 0x0c2;
    pub const Atilde : Key = 0x0c3;
    pub const Adiaeresis : Key = 0x0c4;
    pub const Aring : Key = 0x0c5;
    pub const AE : Key = 0x0c6;
    pub const Ccedilla : Key = 0x0c7;
    pub const Egrave : Key = 0x0c8;
    pub const Eacute : Key = 0x0c9;
    pub const Ecircumflex : Key = 0x0ca;
    pub const Ediaeresis : Key = 0x0cb;
    pub const Igrave : Key = 0x0cc;
    pub const Iacute : Key = 0x0cd;
    pub const Icircumflex : Key = 0x0ce;
    pub const Idiaeresis : Key = 0x0cf;
    pub const ETH : Key = 0x0d0;
    pub const Eth : Key = 0x0d0;
    pub const Ntilde : Key = 0x0d1;
    pub const Ograve : Key = 0x0d2;
    pub const Oacute : Key = 0x0d3;
    pub const Ocircumflex : Key = 0x0d4;
    pub const Otilde : Key = 0x0d5;
    pub const Odiaeresis : Key = 0x0d6;
    pub const multiply : Key = 0x0d7;
    pub const Oslash : Key = 0x0d8;
    pub const Ooblique : Key = 0x0d8;
    pub const Ugrave : Key = 0x0d9;
    pub const Uacute : Key = 0x0da;
    pub const Ucircumflex : Key = 0x0db;
    pub const Udiaeresis : Key = 0x0dc;
    pub const Yacute : Key = 0x0dd;
    pub const THORN : Key = 0x0de;
    pub const Thorn : Key = 0x0de;
    pub const ssharp : Key = 0x0df;
    pub const agrave : Key = 0x0e0;
    pub const aacute : Key = 0x0e1;
    pub const acircumflex : Key = 0x0e2;
    pub const atilde : Key = 0x0e3;
    pub const adiaeresis : Key = 0x0e4;
    pub const aring : Key = 0x0e5;
    pub const ae : Key = 0x0e6;
    pub const ccedilla : Key = 0x0e7;
    pub const egrave : Key = 0x0e8;
    pub const eacute : Key = 0x0e9;
    pub const ecircumflex : Key = 0x0ea;
    pub const ediaeresis : Key = 0x0eb;
    pub const igrave : Key = 0x0ec;
    pub const iacute : Key = 0x0ed;
    pub const icircumflex : Key = 0x0ee;
    pub const idiaeresis : Key = 0x0ef;
    pub const eth : Key = 0x0f0;
    pub const ntilde : Key = 0x0f1;
    pub const ograve : Key = 0x0f2;
    pub const oacute : Key = 0x0f3;
    pub const ocircumflex : Key = 0x0f4;
    pub const otilde : Key = 0x0f5;
    pub const odiaeresis : Key = 0x0f6;
    pub const division : Key = 0x0f7;
    pub const oslash : Key = 0x0f8;
    pub const ooblique : Key = 0x0f8;
    pub const ugrave : Key = 0x0f9;
    pub const uacute : Key = 0x0fa;
    pub const ucircumflex : Key = 0x0fb;
    pub const udiaeresis : Key = 0x0fc;
    pub const yacute : Key = 0x0fd;
    pub const thorn : Key = 0x0fe;
    pub const ydiaeresis : Key = 0x0ff;
    pub const Aogonek : Key = 0x1a1;
    pub const breve : Key = 0x1a2;
    pub const Lstroke : Key = 0x1a3;
    pub const Lcaron : Key = 0x1a5;
    pub const Sacute : Key = 0x1a6;
    pub const Scaron : Key = 0x1a9;
    pub const Scedilla : Key = 0x1aa;
    pub const Tcaron : Key = 0x1ab;
    pub const Zacute : Key = 0x1ac;
    pub const Zcaron : Key = 0x1ae;
    pub const Zabovedot : Key = 0x1af;
    pub const aogonek : Key = 0x1b1;
    pub const ogonek : Key = 0x1b2;
    pub const lstroke : Key = 0x1b3;
    pub const lcaron : Key = 0x1b5;
    pub const sacute : Key = 0x1b6;
    pub const caron : Key = 0x1b7;
    pub const scaron : Key = 0x1b9;
    pub const scedilla : Key = 0x1ba;
    pub const tcaron : Key = 0x1bb;
    pub const zacute : Key = 0x1bc;
    pub const doubleacute : Key = 0x1bd;
    pub const zcaron : Key = 0x1be;
    pub const zabovedot : Key = 0x1bf;
    pub const Racute : Key = 0x1c0;
    pub const Abreve : Key = 0x1c3;
    pub const Lacute : Key = 0x1c5;
    pub const Cacute : Key = 0x1c6;
    pub const Ccaron : Key = 0x1c8;
    pub const Eogonek : Key = 0x1ca;
    pub const Ecaron : Key = 0x1cc;
    pub const Dcaron : Key = 0x1cf;
    pub const Dstroke : Key = 0x1d0;
    pub const Nacute : Key = 0x1d1;
    pub const Ncaron : Key = 0x1d2;
    pub const Odoubleacute : Key = 0x1d5;
    pub const Rcaron : Key = 0x1d8;
    pub const Uring : Key = 0x1d9;
    pub const Udoubleacute : Key = 0x1db;
    pub const Tcedilla : Key = 0x1de;
    pub const racute : Key = 0x1e0;
    pub const abreve : Key = 0x1e3;
    pub const lacute : Key = 0x1e5;
    pub const cacute : Key = 0x1e6;
    pub const ccaron : Key = 0x1e8;
    pub const eogonek : Key = 0x1ea;
    pub const ecaron : Key = 0x1ec;
    pub const dcaron : Key = 0x1ef;
    pub const dstroke : Key = 0x1f0;
    pub const nacute : Key = 0x1f1;
    pub const ncaron : Key = 0x1f2;
    pub const odoubleacute : Key = 0x1f5;
    pub const rcaron : Key = 0x1f8;
    pub const uring : Key = 0x1f9;
    pub const udoubleacute : Key = 0x1fb;
    pub const tcedilla : Key = 0x1fe;
    pub const abovedot : Key = 0x1ff;
    pub const Hstroke : Key = 0x2a1;
    pub const Hcircumflex : Key = 0x2a6;
    pub const Iabovedot : Key = 0x2a9;
    pub const Gbreve : Key = 0x2ab;
    pub const Jcircumflex : Key = 0x2ac;
    pub const hstroke : Key = 0x2b1;
    pub const hcircumflex : Key = 0x2b6;
    pub const idotless : Key = 0x2b9;
    pub const gbreve : Key = 0x2bb;
    pub const jcircumflex : Key = 0x2bc;
    pub const Cabovedot : Key = 0x2c5;
    pub const Ccircumflex : Key = 0x2c6;
    pub const Gabovedot : Key = 0x2d5;
    pub const Gcircumflex : Key = 0x2d8;
    pub const Ubreve : Key = 0x2dd;
    pub const Scircumflex : Key = 0x2de;
    pub const cabovedot : Key = 0x2e5;
    pub const ccircumflex : Key = 0x2e6;
    pub const gabovedot : Key = 0x2f5;
    pub const gcircumflex : Key = 0x2f8;
    pub const ubreve : Key = 0x2fd;
    pub const scircumflex : Key = 0x2fe;
    pub const kra : Key = 0x3a2;
    pub const kappa : Key = 0x3a2;
    pub const Rcedilla : Key = 0x3a3;
    pub const Itilde : Key = 0x3a5;
    pub const Lcedilla : Key = 0x3a6;
    pub const Emacron : Key = 0x3aa;
    pub const Gcedilla : Key = 0x3ab;
    pub const Tslash : Key = 0x3ac;
    pub const rcedilla : Key = 0x3b3;
    pub const itilde : Key = 0x3b5;
    pub const lcedilla : Key = 0x3b6;
    pub const emacron : Key = 0x3ba;
    pub const gcedilla : Key = 0x3bb;
    pub const tslash : Key = 0x3bc;
    pub const ENG : Key = 0x3bd;
    pub const eng : Key = 0x3bf;
    pub const Amacron : Key = 0x3c0;
    pub const Iogonek : Key = 0x3c7;
    pub const Eabovedot : Key = 0x3cc;
    pub const Imacron : Key = 0x3cf;
    pub const Ncedilla : Key = 0x3d1;
    pub const Omacron : Key = 0x3d2;
    pub const Kcedilla : Key = 0x3d3;
    pub const Uogonek : Key = 0x3d9;
    pub const Utilde : Key = 0x3dd;
    pub const Umacron : Key = 0x3de;
    pub const amacron : Key = 0x3e0;
    pub const iogonek : Key = 0x3e7;
    pub const eabovedot : Key = 0x3ec;
    pub const imacron : Key = 0x3ef;
    pub const ncedilla : Key = 0x3f1;
    pub const omacron : Key = 0x3f2;
    pub const kcedilla : Key = 0x3f3;
    pub const uogonek : Key = 0x3f9;
    pub const utilde : Key = 0x3fd;
    pub const umacron : Key = 0x3fe;
    pub const Wcircumflex : Key = 0x1000174;
    pub const wcircumflex : Key = 0x1000175;
    pub const Ycircumflex : Key = 0x1000176;
    pub const ycircumflex : Key = 0x1000177;
    pub const Babovedot : Key = 0x1001e02;
    pub const babovedot : Key = 0x1001e03;
    pub const Dabovedot : Key = 0x1001e0a;
    pub const dabovedot : Key = 0x1001e0b;
    pub const Fabovedot : Key = 0x1001e1e;
    pub const fabovedot : Key = 0x1001e1f;
    pub const Mabovedot : Key = 0x1001e40;
    pub const mabovedot : Key = 0x1001e41;
    pub const Pabovedot : Key = 0x1001e56;
    pub const pabovedot : Key = 0x1001e57;
    pub const Sabovedot : Key = 0x1001e60;
    pub const sabovedot : Key = 0x1001e61;
    pub const Tabovedot : Key = 0x1001e6a;
    pub const tabovedot : Key = 0x1001e6b;
    pub const Wgrave : Key = 0x1001e80;
    pub const wgrave : Key = 0x1001e81;
    pub const Wacute : Key = 0x1001e82;
    pub const wacute : Key = 0x1001e83;
    pub const Wdiaeresis : Key = 0x1001e84;
    pub const wdiaeresis : Key = 0x1001e85;
    pub const Ygrave : Key = 0x1001ef2;
    pub const ygrave : Key = 0x1001ef3;
    pub const OE : Key = 0x13bc;
    pub const oe : Key = 0x13bd;
    pub const Ydiaeresis : Key = 0x13be;
    pub const overline : Key = 0x47e;
    pub const kana_fullstop : Key = 0x4a1;
    pub const kana_openingbracket : Key = 0x4a2;
    pub const kana_closingbracket : Key = 0x4a3;
    pub const kana_comma : Key = 0x4a4;
    pub const kana_conjunctive : Key = 0x4a5;
    pub const kana_middledot : Key = 0x4a5;
    pub const kana_WO : Key = 0x4a6;
    pub const kana_a : Key = 0x4a7;
    pub const kana_i : Key = 0x4a8;
    pub const kana_u : Key = 0x4a9;
    pub const kana_e : Key = 0x4aa;
    pub const kana_o : Key = 0x4ab;
    pub const kana_ya : Key = 0x4ac;
    pub const kana_yu : Key = 0x4ad;
    pub const kana_yo : Key = 0x4ae;
    pub const kana_tsu : Key = 0x4af;
    pub const kana_tu : Key = 0x4af;
    pub const prolongedsound : Key = 0x4b0;
    pub const kana_A : Key = 0x4b1;
    pub const kana_I : Key = 0x4b2;
    pub const kana_U : Key = 0x4b3;
    pub const kana_E : Key = 0x4b4;
    pub const kana_O : Key = 0x4b5;
    pub const kana_KA : Key = 0x4b6;
    pub const kana_KI : Key = 0x4b7;
    pub const kana_KU : Key = 0x4b8;
    pub const kana_KE : Key = 0x4b9;
    pub const kana_KO : Key = 0x4ba;
    pub const kana_SA : Key = 0x4bb;
    pub const kana_SHI : Key = 0x4bc;
    pub const kana_SU : Key = 0x4bd;
    pub const kana_SE : Key = 0x4be;
    pub const kana_SO : Key = 0x4bf;
    pub const kana_TA : Key = 0x4c0;
    pub const kana_CHI : Key = 0x4c1;
    pub const kana_TI : Key = 0x4c1;
    pub const kana_TSU : Key = 0x4c2;
    pub const kana_TU : Key = 0x4c2;
    pub const kana_TE : Key = 0x4c3;
    pub const kana_TO : Key = 0x4c4;
    pub const kana_NA : Key = 0x4c5;
    pub const kana_NI : Key = 0x4c6;
    pub const kana_NU : Key = 0x4c7;
    pub const kana_NE : Key = 0x4c8;
    pub const kana_NO : Key = 0x4c9;
    pub const kana_HA : Key = 0x4ca;
    pub const kana_HI : Key = 0x4cb;
    pub const kana_FU : Key = 0x4cc;
    pub const kana_HU : Key = 0x4cc;
    pub const kana_HE : Key = 0x4cd;
    pub const kana_HO : Key = 0x4ce;
    pub const kana_MA : Key = 0x4cf;
    pub const kana_MI : Key = 0x4d0;
    pub const kana_MU : Key = 0x4d1;
    pub const kana_ME : Key = 0x4d2;
    pub const kana_MO : Key = 0x4d3;
    pub const kana_YA : Key = 0x4d4;
    pub const kana_YU : Key = 0x4d5;
    pub const kana_YO : Key = 0x4d6;
    pub const kana_RA : Key = 0x4d7;
    pub const kana_RI : Key = 0x4d8;
    pub const kana_RU : Key = 0x4d9;
    pub const kana_RE : Key = 0x4da;
    pub const kana_RO : Key = 0x4db;
    pub const kana_WA : Key = 0x4dc;
    pub const kana_N : Key = 0x4dd;
    pub const voicedsound : Key = 0x4de;
    pub const semivoicedsound : Key = 0x4df;
    pub const kana_switch : Key = 0xff7e;
    pub const Farsi_0 : Key = 0x10006f0;
    pub const Farsi_1 : Key = 0x10006f1;
    pub const Farsi_2 : Key = 0x10006f2;
    pub const Farsi_3 : Key = 0x10006f3;
    pub const Farsi_4 : Key = 0x10006f4;
    pub const Farsi_5 : Key = 0x10006f5;
    pub const Farsi_6 : Key = 0x10006f6;
    pub const Farsi_7 : Key = 0x10006f7;
    pub const Farsi_8 : Key = 0x10006f8;
    pub const Farsi_9 : Key = 0x10006f9;
    pub const Arabic_percent : Key = 0x100066a;
    pub const Arabic_superscript_alef : Key = 0x1000670;
    pub const Arabic_tteh : Key = 0x1000679;
    pub const Arabic_peh : Key = 0x100067e;
    pub const Arabic_tcheh : Key = 0x1000686;
    pub const Arabic_ddal : Key = 0x1000688;
    pub const Arabic_rreh : Key = 0x1000691;
    pub const Arabic_comma : Key = 0x5ac;
    pub const Arabic_fullstop : Key = 0x10006d4;
    pub const Arabic_0 : Key = 0x1000660;
    pub const Arabic_1 : Key = 0x1000661;
    pub const Arabic_2 : Key = 0x1000662;
    pub const Arabic_3 : Key = 0x1000663;
    pub const Arabic_4 : Key = 0x1000664;
    pub const Arabic_5 : Key = 0x1000665;
    pub const Arabic_6 : Key = 0x1000666;
    pub const Arabic_7 : Key = 0x1000667;
    pub const Arabic_8 : Key = 0x1000668;
    pub const Arabic_9 : Key = 0x1000669;
    pub const Arabic_semicolon : Key = 0x5bb;
    pub const Arabic_question_mark : Key = 0x5bf;
    pub const Arabic_hamza : Key = 0x5c1;
    pub const Arabic_maddaonalef : Key = 0x5c2;
    pub const Arabic_hamzaonalef : Key = 0x5c3;
    pub const Arabic_hamzaonwaw : Key = 0x5c4;
    pub const Arabic_hamzaunderalef : Key = 0x5c5;
    pub const Arabic_hamzaonyeh : Key = 0x5c6;
    pub const Arabic_alef : Key = 0x5c7;
    pub const Arabic_beh : Key = 0x5c8;
    pub const Arabic_tehmarbuta : Key = 0x5c9;
    pub const Arabic_teh : Key = 0x5ca;
    pub const Arabic_theh : Key = 0x5cb;
    pub const Arabic_jeem : Key = 0x5cc;
    pub const Arabic_hah : Key = 0x5cd;
    pub const Arabic_khah : Key = 0x5ce;
    pub const Arabic_dal : Key = 0x5cf;
    pub const Arabic_thal : Key = 0x5d0;
    pub const Arabic_ra : Key = 0x5d1;
    pub const Arabic_zain : Key = 0x5d2;
    pub const Arabic_seen : Key = 0x5d3;
    pub const Arabic_sheen : Key = 0x5d4;
    pub const Arabic_sad : Key = 0x5d5;
    pub const Arabic_dad : Key = 0x5d6;
    pub const Arabic_tah : Key = 0x5d7;
    pub const Arabic_zah : Key = 0x5d8;
    pub const Arabic_ain : Key = 0x5d9;
    pub const Arabic_ghain : Key = 0x5da;
    pub const Arabic_tatweel : Key = 0x5e0;
    pub const Arabic_feh : Key = 0x5e1;
    pub const Arabic_qaf : Key = 0x5e2;
    pub const Arabic_kaf : Key = 0x5e3;
    pub const Arabic_lam : Key = 0x5e4;
    pub const Arabic_meem : Key = 0x5e5;
    pub const Arabic_noon : Key = 0x5e6;
    pub const Arabic_ha : Key = 0x5e7;
    pub const Arabic_heh : Key = 0x5e7;
    pub const Arabic_waw : Key = 0x5e8;
    pub const Arabic_alefmaksura : Key = 0x5e9;
    pub const Arabic_yeh : Key = 0x5ea;
    pub const Arabic_fathatan : Key = 0x5eb;
    pub const Arabic_dammatan : Key = 0x5ec;
    pub const Arabic_kasratan : Key = 0x5ed;
    pub const Arabic_fatha : Key = 0x5ee;
    pub const Arabic_damma : Key = 0x5ef;
    pub const Arabic_kasra : Key = 0x5f0;
    pub const Arabic_shadda : Key = 0x5f1;
    pub const Arabic_sukun : Key = 0x5f2;
    pub const Arabic_madda_above : Key = 0x1000653;
    pub const Arabic_hamza_above : Key = 0x1000654;
    pub const Arabic_hamza_below : Key = 0x1000655;
    pub const Arabic_jeh : Key = 0x1000698;
    pub const Arabic_veh : Key = 0x10006a4;
    pub const Arabic_keheh : Key = 0x10006a9;
    pub const Arabic_gaf : Key = 0x10006af;
    pub const Arabic_noon_ghunna : Key = 0x10006ba;
    pub const Arabic_heh_doachashmee : Key = 0x10006be;
    pub const Farsi_yeh : Key = 0x10006cc;
    pub const Arabic_farsi_yeh : Key = 0x10006cc;
    pub const Arabic_yeh_baree : Key = 0x10006d2;
    pub const Arabic_heh_goal : Key = 0x10006c1;
    pub const Arabic_switch : Key = 0xff7e;
    pub const Cyrillic_GHE_bar : Key = 0x1000492;
    pub const Cyrillic_ghe_bar : Key = 0x1000493;
    pub const Cyrillic_ZHE_descender : Key = 0x1000496;
    pub const Cyrillic_zhe_descender : Key = 0x1000497;
    pub const Cyrillic_KA_descender : Key = 0x100049a;
    pub const Cyrillic_ka_descender : Key = 0x100049b;
    pub const Cyrillic_KA_vertstroke : Key = 0x100049c;
    pub const Cyrillic_ka_vertstroke : Key = 0x100049d;
    pub const Cyrillic_EN_descender : Key = 0x10004a2;
    pub const Cyrillic_en_descender : Key = 0x10004a3;
    pub const Cyrillic_U_straight : Key = 0x10004ae;
    pub const Cyrillic_u_straight : Key = 0x10004af;
    pub const Cyrillic_U_straight_bar : Key = 0x10004b0;
    pub const Cyrillic_u_straight_bar : Key = 0x10004b1;
    pub const Cyrillic_HA_descender : Key = 0x10004b2;
    pub const Cyrillic_ha_descender : Key = 0x10004b3;
    pub const Cyrillic_CHE_descender : Key = 0x10004b6;
    pub const Cyrillic_che_descender : Key = 0x10004b7;
    pub const Cyrillic_CHE_vertstroke : Key = 0x10004b8;
    pub const Cyrillic_che_vertstroke : Key = 0x10004b9;
    pub const Cyrillic_SHHA : Key = 0x10004ba;
    pub const Cyrillic_shha : Key = 0x10004bb;
    pub const Cyrillic_SCHWA : Key = 0x10004d8;
    pub const Cyrillic_schwa : Key = 0x10004d9;
    pub const Cyrillic_I_macron : Key = 0x10004e2;
    pub const Cyrillic_i_macron : Key = 0x10004e3;
    pub const Cyrillic_O_bar : Key = 0x10004e8;
    pub const Cyrillic_o_bar : Key = 0x10004e9;
    pub const Cyrillic_U_macron : Key = 0x10004ee;
    pub const Cyrillic_u_macron : Key = 0x10004ef;
    pub const Serbian_dje : Key = 0x6a1;
    pub const Macedonia_gje : Key = 0x6a2;
    pub const Cyrillic_io : Key = 0x6a3;
    pub const Ukrainian_ie : Key = 0x6a4;
    pub const Ukranian_je : Key = 0x6a4;
    pub const Macedonia_dse : Key = 0x6a5;
    pub const Ukrainian_i : Key = 0x6a6;
    pub const Ukranian_i : Key = 0x6a6;
    pub const Ukrainian_yi : Key = 0x6a7;
    pub const Ukranian_yi : Key = 0x6a7;
    pub const Cyrillic_je : Key = 0x6a8;
    pub const Serbian_je : Key = 0x6a8;
    pub const Cyrillic_lje : Key = 0x6a9;
    pub const Serbian_lje : Key = 0x6a9;
    pub const Cyrillic_nje : Key = 0x6aa;
    pub const Serbian_nje : Key = 0x6aa;
    pub const Serbian_tshe : Key = 0x6ab;
    pub const Macedonia_kje : Key = 0x6ac;
    pub const Ukrainian_ghe_with_upturn : Key = 0x6ad;
    pub const Byelorussian_shortu : Key = 0x6ae;
    pub const Cyrillic_dzhe : Key = 0x6af;
    pub const Serbian_dze : Key = 0x6af;
    pub const numerosign : Key = 0x6b0;
    pub const Serbian_DJE : Key = 0x6b1;
    pub const Macedonia_GJE : Key = 0x6b2;
    pub const Cyrillic_IO : Key = 0x6b3;
    pub const Ukrainian_IE : Key = 0x6b4;
    pub const Ukranian_JE : Key = 0x6b4;
    pub const Macedonia_DSE : Key = 0x6b5;
    pub const Ukrainian_I : Key = 0x6b6;
    pub const Ukranian_I : Key = 0x6b6;
    pub const Ukrainian_YI : Key = 0x6b7;
    pub const Ukranian_YI : Key = 0x6b7;
    pub const Cyrillic_JE : Key = 0x6b8;
    pub const Serbian_JE : Key = 0x6b8;
    pub const Cyrillic_LJE : Key = 0x6b9;
    pub const Serbian_LJE : Key = 0x6b9;
    pub const Cyrillic_NJE : Key = 0x6ba;
    pub const Serbian_NJE : Key = 0x6ba;
    pub const Serbian_TSHE : Key = 0x6bb;
    pub const Macedonia_KJE : Key = 0x6bc;
    pub const Ukrainian_GHE_WITH_UPTURN : Key = 0x6bd;
    pub const Byelorussian_SHORTU : Key = 0x6be;
    pub const Cyrillic_DZHE : Key = 0x6bf;
    pub const Serbian_DZE : Key = 0x6bf;
    pub const Cyrillic_yu : Key = 0x6c0;
    pub const Cyrillic_a : Key = 0x6c1;
    pub const Cyrillic_be : Key = 0x6c2;
    pub const Cyrillic_tse : Key = 0x6c3;
    pub const Cyrillic_de : Key = 0x6c4;
    pub const Cyrillic_ie : Key = 0x6c5;
    pub const Cyrillic_ef : Key = 0x6c6;
    pub const Cyrillic_ghe : Key = 0x6c7;
    pub const Cyrillic_ha : Key = 0x6c8;
    pub const Cyrillic_i : Key = 0x6c9;
    pub const Cyrillic_shorti : Key = 0x6ca;
    pub const Cyrillic_ka : Key = 0x6cb;
    pub const Cyrillic_el : Key = 0x6cc;
    pub const Cyrillic_em : Key = 0x6cd;
    pub const Cyrillic_en : Key = 0x6ce;
    pub const Cyrillic_o : Key = 0x6cf;
    pub const Cyrillic_pe : Key = 0x6d0;
    pub const Cyrillic_ya : Key = 0x6d1;
    pub const Cyrillic_er : Key = 0x6d2;
    pub const Cyrillic_es : Key = 0x6d3;
    pub const Cyrillic_te : Key = 0x6d4;
    pub const Cyrillic_u : Key = 0x6d5;
    pub const Cyrillic_zhe : Key = 0x6d6;
    pub const Cyrillic_ve : Key = 0x6d7;
    pub const Cyrillic_softsign : Key = 0x6d8;
    pub const Cyrillic_yeru : Key = 0x6d9;
    pub const Cyrillic_ze : Key = 0x6da;
    pub const Cyrillic_sha : Key = 0x6db;
    pub const Cyrillic_e : Key = 0x6dc;
    pub const Cyrillic_shcha : Key = 0x6dd;
    pub const Cyrillic_che : Key = 0x6de;
    pub const Cyrillic_hardsign : Key = 0x6df;
    pub const Cyrillic_YU : Key = 0x6e0;
    pub const Cyrillic_A : Key = 0x6e1;
    pub const Cyrillic_BE : Key = 0x6e2;
    pub const Cyrillic_TSE : Key = 0x6e3;
    pub const Cyrillic_DE : Key = 0x6e4;
    pub const Cyrillic_IE : Key = 0x6e5;
    pub const Cyrillic_EF : Key = 0x6e6;
    pub const Cyrillic_GHE : Key = 0x6e7;
    pub const Cyrillic_HA : Key = 0x6e8;
    pub const Cyrillic_I : Key = 0x6e9;
    pub const Cyrillic_SHORTI : Key = 0x6ea;
    pub const Cyrillic_KA : Key = 0x6eb;
    pub const Cyrillic_EL : Key = 0x6ec;
    pub const Cyrillic_EM : Key = 0x6ed;
    pub const Cyrillic_EN : Key = 0x6ee;
    pub const Cyrillic_O : Key = 0x6ef;
    pub const Cyrillic_PE : Key = 0x6f0;
    pub const Cyrillic_YA : Key = 0x6f1;
    pub const Cyrillic_ER : Key = 0x6f2;
    pub const Cyrillic_ES : Key = 0x6f3;
    pub const Cyrillic_TE : Key = 0x6f4;
    pub const Cyrillic_U : Key = 0x6f5;
    pub const Cyrillic_ZHE : Key = 0x6f6;
    pub const Cyrillic_VE : Key = 0x6f7;
    pub const Cyrillic_SOFTSIGN : Key = 0x6f8;
    pub const Cyrillic_YERU : Key = 0x6f9;
    pub const Cyrillic_ZE : Key = 0x6fa;
    pub const Cyrillic_SHA : Key = 0x6fb;
    pub const Cyrillic_E : Key = 0x6fc;
    pub const Cyrillic_SHCHA : Key = 0x6fd;
    pub const Cyrillic_CHE : Key = 0x6fe;
    pub const Cyrillic_HARDSIGN : Key = 0x6ff;
    pub const Greek_ALPHAaccent : Key = 0x7a1;
    pub const Greek_EPSILONaccent : Key = 0x7a2;
    pub const Greek_ETAaccent : Key = 0x7a3;
    pub const Greek_IOTAaccent : Key = 0x7a4;
    pub const Greek_IOTAdieresis : Key = 0x7a5;
    pub const Greek_IOTAdiaeresis : Key = 0x7a5;
    pub const Greek_OMICRONaccent : Key = 0x7a7;
    pub const Greek_UPSILONaccent : Key = 0x7a8;
    pub const Greek_UPSILONdieresis : Key = 0x7a9;
    pub const Greek_OMEGAaccent : Key = 0x7ab;
    pub const Greek_accentdieresis : Key = 0x7ae;
    pub const Greek_horizbar : Key = 0x7af;
    pub const Greek_alphaaccent : Key = 0x7b1;
    pub const Greek_epsilonaccent : Key = 0x7b2;
    pub const Greek_etaaccent : Key = 0x7b3;
    pub const Greek_iotaaccent : Key = 0x7b4;
    pub const Greek_iotadieresis : Key = 0x7b5;
    pub const Greek_iotaaccentdieresis : Key = 0x7b6;
    pub const Greek_omicronaccent : Key = 0x7b7;
    pub const Greek_upsilonaccent : Key = 0x7b8;
    pub const Greek_upsilondieresis : Key = 0x7b9;
    pub const Greek_upsilonaccentdieresis : Key = 0x7ba;
    pub const Greek_omegaaccent : Key = 0x7bb;
    pub const Greek_ALPHA : Key = 0x7c1;
    pub const Greek_BETA : Key = 0x7c2;
    pub const Greek_GAMMA : Key = 0x7c3;
    pub const Greek_DELTA : Key = 0x7c4;
    pub const Greek_EPSILON : Key = 0x7c5;
    pub const Greek_ZETA : Key = 0x7c6;
    pub const Greek_ETA : Key = 0x7c7;
    pub const Greek_THETA : Key = 0x7c8;
    pub const Greek_IOTA : Key = 0x7c9;
    pub const Greek_KAPPA : Key = 0x7ca;
    pub const Greek_LAMDA : Key = 0x7cb;
    pub const Greek_LAMBDA : Key = 0x7cb;
    pub const Greek_MU : Key = 0x7cc;
    pub const Greek_NU : Key = 0x7cd;
    pub const Greek_XI : Key = 0x7ce;
    pub const Greek_OMICRON : Key = 0x7cf;
    pub const Greek_PI : Key = 0x7d0;
    pub const Greek_RHO : Key = 0x7d1;
    pub const Greek_SIGMA : Key = 0x7d2;
    pub const Greek_TAU : Key = 0x7d4;
    pub const Greek_UPSILON : Key = 0x7d5;
    pub const Greek_PHI : Key = 0x7d6;
    pub const Greek_CHI : Key = 0x7d7;
    pub const Greek_PSI : Key = 0x7d8;
    pub const Greek_OMEGA : Key = 0x7d9;
    pub const Greek_alpha : Key = 0x7e1;
    pub const Greek_beta : Key = 0x7e2;
    pub const Greek_gamma : Key = 0x7e3;
    pub const Greek_delta : Key = 0x7e4;
    pub const Greek_epsilon : Key = 0x7e5;
    pub const Greek_zeta : Key = 0x7e6;
    pub const Greek_eta : Key = 0x7e7;
    pub const Greek_theta : Key = 0x7e8;
    pub const Greek_iota : Key = 0x7e9;
    pub const Greek_kappa : Key = 0x7ea;
    pub const Greek_lamda : Key = 0x7eb;
    pub const Greek_lambda : Key = 0x7eb;
    pub const Greek_mu : Key = 0x7ec;
    pub const Greek_nu : Key = 0x7ed;
    pub const Greek_xi : Key = 0x7ee;
    pub const Greek_omicron : Key = 0x7ef;
    pub const Greek_pi : Key = 0x7f0;
    pub const Greek_rho : Key = 0x7f1;
    pub const Greek_sigma : Key = 0x7f2;
    pub const Greek_finalsmallsigma : Key = 0x7f3;
    pub const Greek_tau : Key = 0x7f4;
    pub const Greek_upsilon : Key = 0x7f5;
    pub const Greek_phi : Key = 0x7f6;
    pub const Greek_chi : Key = 0x7f7;
    pub const Greek_psi : Key = 0x7f8;
    pub const Greek_omega : Key = 0x7f9;
    pub const Greek_switch : Key = 0xff7e;
    pub const leftradical : Key = 0x8a1;
    pub const topleftradical : Key = 0x8a2;
    pub const horizconnector : Key = 0x8a3;
    pub const topintegral : Key = 0x8a4;
    pub const botintegral : Key = 0x8a5;
    pub const vertconnector : Key = 0x8a6;
    pub const topleftsqbracket : Key = 0x8a7;
    pub const botleftsqbracket : Key = 0x8a8;
    pub const toprightsqbracket : Key = 0x8a9;
    pub const botrightsqbracket : Key = 0x8aa;
    pub const topleftparens : Key = 0x8ab;
    pub const botleftparens : Key = 0x8ac;
    pub const toprightparens : Key = 0x8ad;
    pub const botrightparens : Key = 0x8ae;
    pub const leftmiddlecurlybrace : Key = 0x8af;
    pub const rightmiddlecurlybrace : Key = 0x8b0;
    pub const topleftsummation : Key = 0x8b1;
    pub const botleftsummation : Key = 0x8b2;
    pub const topvertsummationconnector : Key = 0x8b3;
    pub const botvertsummationconnector : Key = 0x8b4;
    pub const toprightsummation : Key = 0x8b5;
    pub const botrightsummation : Key = 0x8b6;
    pub const rightmiddlesummation : Key = 0x8b7;
    pub const lessthanequal : Key = 0x8bc;
    pub const notequal : Key = 0x8bd;
    pub const greaterthanequal : Key = 0x8be;
    pub const integral : Key = 0x8bf;
    pub const therefore : Key = 0x8c0;
    pub const variation : Key = 0x8c1;
    pub const infinity : Key = 0x8c2;
    pub const nabla : Key = 0x8c5;
    pub const approximate : Key = 0x8c8;
    pub const similarequal : Key = 0x8c9;
    pub const ifonlyif : Key = 0x8cd;
    pub const implies : Key = 0x8ce;
    pub const identical : Key = 0x8cf;
    pub const radical : Key = 0x8d6;
    pub const includedin : Key = 0x8da;
    pub const includes : Key = 0x8db;
    pub const intersection : Key = 0x8dc;
    pub const union : Key = 0x8dd;
    pub const logicaland : Key = 0x8de;
    pub const logicalor : Key = 0x8df;
    pub const partialderivative : Key = 0x8ef;
    pub const function : Key = 0x8f6;
    pub const leftarrow : Key = 0x8fb;
    pub const uparrow : Key = 0x8fc;
    pub const rightarrow : Key = 0x8fd;
    pub const downarrow : Key = 0x8fe;
    pub const blank : Key = 0x9df;
    pub const soliddiamond : Key = 0x9e0;
    pub const checkerboard : Key = 0x9e1;
    pub const ht : Key = 0x9e2;
    pub const ff : Key = 0x9e3;
    pub const cr : Key = 0x9e4;
    pub const lf : Key = 0x9e5;
    pub const nl : Key = 0x9e8;
    pub const vt : Key = 0x9e9;
    pub const lowrightcorner : Key = 0x9ea;
    pub const uprightcorner : Key = 0x9eb;
    pub const upleftcorner : Key = 0x9ec;
    pub const lowleftcorner : Key = 0x9ed;
    pub const crossinglines : Key = 0x9ee;
    pub const horizlinescan1 : Key = 0x9ef;
    pub const horizlinescan3 : Key = 0x9f0;
    pub const horizlinescan5 : Key = 0x9f1;
    pub const horizlinescan7 : Key = 0x9f2;
    pub const horizlinescan9 : Key = 0x9f3;
    pub const leftt : Key = 0x9f4;
    pub const rightt : Key = 0x9f5;
    pub const bott : Key = 0x9f6;
    pub const topt : Key = 0x9f7;
    pub const vertbar : Key = 0x9f8;
    pub const emspace : Key = 0xaa1;
    pub const enspace : Key = 0xaa2;
    pub const em3space : Key = 0xaa3;
    pub const em4space : Key = 0xaa4;
    pub const digitspace : Key = 0xaa5;
    pub const punctspace : Key = 0xaa6;
    pub const thinspace : Key = 0xaa7;
    pub const hairspace : Key = 0xaa8;
    pub const emdash : Key = 0xaa9;
    pub const endash : Key = 0xaaa;
    pub const signifblank : Key = 0xaac;
    pub const ellipsis : Key = 0xaae;
    pub const doubbaselinedot : Key = 0xaaf;
    pub const onethird : Key = 0xab0;
    pub const twothirds : Key = 0xab1;
    pub const onefifth : Key = 0xab2;
    pub const twofifths : Key = 0xab3;
    pub const threefifths : Key = 0xab4;
    pub const fourfifths : Key = 0xab5;
    pub const onesixth : Key = 0xab6;
    pub const fivesixths : Key = 0xab7;
    pub const careof : Key = 0xab8;
    pub const figdash : Key = 0xabb;
    pub const leftanglebracket : Key = 0xabc;
    pub const decimalpoint : Key = 0xabd;
    pub const rightanglebracket : Key = 0xabe;
    pub const marker : Key = 0xabf;
    pub const oneeighth : Key = 0xac3;
    pub const threeeighths : Key = 0xac4;
    pub const fiveeighths : Key = 0xac5;
    pub const seveneighths : Key = 0xac6;
    pub const trademark : Key = 0xac9;
    pub const signaturemark : Key = 0xaca;
    pub const trademarkincircle : Key = 0xacb;
    pub const leftopentriangle : Key = 0xacc;
    pub const rightopentriangle : Key = 0xacd;
    pub const emopencircle : Key = 0xace;
    pub const emopenrectangle : Key = 0xacf;
    pub const leftsinglequotemark : Key = 0xad0;
    pub const rightsinglequotemark : Key = 0xad1;
    pub const leftdoublequotemark : Key = 0xad2;
    pub const rightdoublequotemark : Key = 0xad3;
    pub const prescription : Key = 0xad4;
    pub const permille : Key = 0xad5;
    pub const minutes : Key = 0xad6;
    pub const seconds : Key = 0xad7;
    pub const latincross : Key = 0xad9;
    pub const hexagram : Key = 0xada;
    pub const filledrectbullet : Key = 0xadb;
    pub const filledlefttribullet : Key = 0xadc;
    pub const filledrighttribullet : Key = 0xadd;
    pub const emfilledcircle : Key = 0xade;
    pub const emfilledrect : Key = 0xadf;
    pub const enopencircbullet : Key = 0xae0;
    pub const enopensquarebullet : Key = 0xae1;
    pub const openrectbullet : Key = 0xae2;
    pub const opentribulletup : Key = 0xae3;
    pub const opentribulletdown : Key = 0xae4;
    pub const openstar : Key = 0xae5;
    pub const enfilledcircbullet : Key = 0xae6;
    pub const enfilledsqbullet : Key = 0xae7;
    pub const filledtribulletup : Key = 0xae8;
    pub const filledtribulletdown : Key = 0xae9;
    pub const leftpointer : Key = 0xaea;
    pub const rightpointer : Key = 0xaeb;
    pub const club : Key = 0xaec;
    pub const diamond : Key = 0xaed;
    pub const heart : Key = 0xaee;
    pub const maltesecross : Key = 0xaf0;
    pub const dagger : Key = 0xaf1;
    pub const doubledagger : Key = 0xaf2;
    pub const checkmark : Key = 0xaf3;
    pub const ballotcross : Key = 0xaf4;
    pub const musicalsharp : Key = 0xaf5;
    pub const musicalflat : Key = 0xaf6;
    pub const malesymbol : Key = 0xaf7;
    pub const femalesymbol : Key = 0xaf8;
    pub const telephone : Key = 0xaf9;
    pub const telephonerecorder : Key = 0xafa;
    pub const phonographcopyright : Key = 0xafb;
    pub const caret : Key = 0xafc;
    pub const singlelowquotemark : Key = 0xafd;
    pub const doublelowquotemark : Key = 0xafe;
    pub const cursor : Key = 0xaff;
    pub const leftcaret : Key = 0xba3;
    pub const rightcaret : Key = 0xba6;
    pub const downcaret : Key = 0xba8;
    pub const upcaret : Key = 0xba9;
    pub const overbar : Key = 0xbc0;
    pub const downtack : Key = 0xbc2;
    pub const upshoe : Key = 0xbc3;
    pub const downstile : Key = 0xbc4;
    pub const underbar : Key = 0xbc6;
    pub const jot : Key = 0xbca;
    pub const quad : Key = 0xbcc;
    pub const uptack : Key = 0xbce;
    pub const circle : Key = 0xbcf;
    pub const upstile : Key = 0xbd3;
    pub const downshoe : Key = 0xbd6;
    pub const rightshoe : Key = 0xbd8;
    pub const leftshoe : Key = 0xbda;
    pub const lefttack : Key = 0xbdc;
    pub const righttack : Key = 0xbfc;
    pub const hebrew_doublelowline : Key = 0xcdf;
    pub const hebrew_aleph : Key = 0xce0;
    pub const hebrew_bet : Key = 0xce1;
    pub const hebrew_beth : Key = 0xce1;
    pub const hebrew_gimel : Key = 0xce2;
    pub const hebrew_gimmel : Key = 0xce2;
    pub const hebrew_dalet : Key = 0xce3;
    pub const hebrew_daleth : Key = 0xce3;
    pub const hebrew_he : Key = 0xce4;
    pub const hebrew_waw : Key = 0xce5;
    pub const hebrew_zain : Key = 0xce6;
    pub const hebrew_zayin : Key = 0xce6;
    pub const hebrew_chet : Key = 0xce7;
    pub const hebrew_het : Key = 0xce7;
    pub const hebrew_tet : Key = 0xce8;
    pub const hebrew_teth : Key = 0xce8;
    pub const hebrew_yod : Key = 0xce9;
    pub const hebrew_finalkaph : Key = 0xcea;
    pub const hebrew_kaph : Key = 0xceb;
    pub const hebrew_lamed : Key = 0xcec;
    pub const hebrew_finalmem : Key = 0xced;
    pub const hebrew_mem : Key = 0xcee;
    pub const hebrew_finalnun : Key = 0xcef;
    pub const hebrew_nun : Key = 0xcf0;
    pub const hebrew_samech : Key = 0xcf1;
    pub const hebrew_samekh : Key = 0xcf1;
    pub const hebrew_ayin : Key = 0xcf2;
    pub const hebrew_finalpe : Key = 0xcf3;
    pub const hebrew_pe : Key = 0xcf4;
    pub const hebrew_finalzade : Key = 0xcf5;
    pub const hebrew_finalzadi : Key = 0xcf5;
    pub const hebrew_zade : Key = 0xcf6;
    pub const hebrew_zadi : Key = 0xcf6;
    pub const hebrew_qoph : Key = 0xcf7;
    pub const hebrew_kuf : Key = 0xcf7;
    pub const hebrew_resh : Key = 0xcf8;
    pub const hebrew_shin : Key = 0xcf9;
    pub const hebrew_taw : Key = 0xcfa;
    pub const hebrew_taf : Key = 0xcfa;
    pub const Hebrew_switch : Key = 0xff7e;
    pub const Thai_kokai : Key = 0xda1;
    pub const Thai_khokhai : Key = 0xda2;
    pub const Thai_khokhuat : Key = 0xda3;
    pub const Thai_khokhwai : Key = 0xda4;
    pub const Thai_khokhon : Key = 0xda5;
    pub const Thai_khorakhang : Key = 0xda6;
    pub const Thai_ngongu : Key = 0xda7;
    pub const Thai_chochan : Key = 0xda8;
    pub const Thai_choching : Key = 0xda9;
    pub const Thai_chochang : Key = 0xdaa;
    pub const Thai_soso : Key = 0xdab;
    pub const Thai_chochoe : Key = 0xdac;
    pub const Thai_yoying : Key = 0xdad;
    pub const Thai_dochada : Key = 0xdae;
    pub const Thai_topatak : Key = 0xdaf;
    pub const Thai_thothan : Key = 0xdb0;
    pub const Thai_thonangmontho : Key = 0xdb1;
    pub const Thai_thophuthao : Key = 0xdb2;
    pub const Thai_nonen : Key = 0xdb3;
    pub const Thai_dodek : Key = 0xdb4;
    pub const Thai_totao : Key = 0xdb5;
    pub const Thai_thothung : Key = 0xdb6;
    pub const Thai_thothahan : Key = 0xdb7;
    pub const Thai_thothong : Key = 0xdb8;
    pub const Thai_nonu : Key = 0xdb9;
    pub const Thai_bobaimai : Key = 0xdba;
    pub const Thai_popla : Key = 0xdbb;
    pub const Thai_phophung : Key = 0xdbc;
    pub const Thai_fofa : Key = 0xdbd;
    pub const Thai_phophan : Key = 0xdbe;
    pub const Thai_fofan : Key = 0xdbf;
    pub const Thai_phosamphao : Key = 0xdc0;
    pub const Thai_moma : Key = 0xdc1;
    pub const Thai_yoyak : Key = 0xdc2;
    pub const Thai_rorua : Key = 0xdc3;
    pub const Thai_ru : Key = 0xdc4;
    pub const Thai_loling : Key = 0xdc5;
    pub const Thai_lu : Key = 0xdc6;
    pub const Thai_wowaen : Key = 0xdc7;
    pub const Thai_sosala : Key = 0xdc8;
    pub const Thai_sorusi : Key = 0xdc9;
    pub const Thai_sosua : Key = 0xdca;
    pub const Thai_hohip : Key = 0xdcb;
    pub const Thai_lochula : Key = 0xdcc;
    pub const Thai_oang : Key = 0xdcd;
    pub const Thai_honokhuk : Key = 0xdce;
    pub const Thai_paiyannoi : Key = 0xdcf;
    pub const Thai_saraa : Key = 0xdd0;
    pub const Thai_maihanakat : Key = 0xdd1;
    pub const Thai_saraaa : Key = 0xdd2;
    pub const Thai_saraam : Key = 0xdd3;
    pub const Thai_sarai : Key = 0xdd4;
    pub const Thai_saraii : Key = 0xdd5;
    pub const Thai_saraue : Key = 0xdd6;
    pub const Thai_sarauee : Key = 0xdd7;
    pub const Thai_sarau : Key = 0xdd8;
    pub const Thai_sarauu : Key = 0xdd9;
    pub const Thai_phinthu : Key = 0xdda;
    pub const Thai_maihanakat_maitho : Key = 0xdde;
    pub const Thai_baht : Key = 0xddf;
    pub const Thai_sarae : Key = 0xde0;
    pub const Thai_saraae : Key = 0xde1;
    pub const Thai_sarao : Key = 0xde2;
    pub const Thai_saraaimaimuan : Key = 0xde3;
    pub const Thai_saraaimaimalai : Key = 0xde4;
    pub const Thai_lakkhangyao : Key = 0xde5;
    pub const Thai_maiyamok : Key = 0xde6;
    pub const Thai_maitaikhu : Key = 0xde7;
    pub const Thai_maiek : Key = 0xde8;
    pub const Thai_maitho : Key = 0xde9;
    pub const Thai_maitri : Key = 0xdea;
    pub const Thai_maichattawa : Key = 0xdeb;
    pub const Thai_thanthakhat : Key = 0xdec;
    pub const Thai_nikhahit : Key = 0xded;
    pub const Thai_leksun : Key = 0xdf0;
    pub const Thai_leknung : Key = 0xdf1;
    pub const Thai_leksong : Key = 0xdf2;
    pub const Thai_leksam : Key = 0xdf3;
    pub const Thai_leksi : Key = 0xdf4;
    pub const Thai_lekha : Key = 0xdf5;
    pub const Thai_lekhok : Key = 0xdf6;
    pub const Thai_lekchet : Key = 0xdf7;
    pub const Thai_lekpaet : Key = 0xdf8;
    pub const Thai_lekkao : Key = 0xdf9;
    pub const Hangul : Key = 0xff31;
    pub const Hangul_Start : Key = 0xff32;
    pub const Hangul_End : Key = 0xff33;
    pub const Hangul_Hanja : Key = 0xff34;
    pub const Hangul_Jamo : Key = 0xff35;
    pub const Hangul_Romaja : Key = 0xff36;
    pub const Hangul_Codeinput : Key = 0xff37;
    pub const Hangul_Jeonja : Key = 0xff38;
    pub const Hangul_Banja : Key = 0xff39;
    pub const Hangul_PreHanja : Key = 0xff3a;
    pub const Hangul_PostHanja : Key = 0xff3b;
    pub const Hangul_SingleCandidate : Key = 0xff3c;
    pub const Hangul_MultipleCandidate : Key = 0xff3d;
    pub const Hangul_PreviousCandidate : Key = 0xff3e;
    pub const Hangul_Special : Key = 0xff3f;
    pub const Hangul_switch : Key = 0xff7e;
    pub const Hangul_Kiyeog : Key = 0xea1;
    pub const Hangul_SsangKiyeog : Key = 0xea2;
    pub const Hangul_KiyeogSios : Key = 0xea3;
    pub const Hangul_Nieun : Key = 0xea4;
    pub const Hangul_NieunJieuj : Key = 0xea5;
    pub const Hangul_NieunHieuh : Key = 0xea6;
    pub const Hangul_Dikeud : Key = 0xea7;
    pub const Hangul_SsangDikeud : Key = 0xea8;
    pub const Hangul_Rieul : Key = 0xea9;
    pub const Hangul_RieulKiyeog : Key = 0xeaa;
    pub const Hangul_RieulMieum : Key = 0xeab;
    pub const Hangul_RieulPieub : Key = 0xeac;
    pub const Hangul_RieulSios : Key = 0xead;
    pub const Hangul_RieulTieut : Key = 0xeae;
    pub const Hangul_RieulPhieuf : Key = 0xeaf;
    pub const Hangul_RieulHieuh : Key = 0xeb0;
    pub const Hangul_Mieum : Key = 0xeb1;
    pub const Hangul_Pieub : Key = 0xeb2;
    pub const Hangul_SsangPieub : Key = 0xeb3;
    pub const Hangul_PieubSios : Key = 0xeb4;
    pub const Hangul_Sios : Key = 0xeb5;
    pub const Hangul_SsangSios : Key = 0xeb6;
    pub const Hangul_Ieung : Key = 0xeb7;
    pub const Hangul_Jieuj : Key = 0xeb8;
    pub const Hangul_SsangJieuj : Key = 0xeb9;
    pub const Hangul_Cieuc : Key = 0xeba;
    pub const Hangul_Khieuq : Key = 0xebb;
    pub const Hangul_Tieut : Key = 0xebc;
    pub const Hangul_Phieuf : Key = 0xebd;
    pub const Hangul_Hieuh : Key = 0xebe;
    pub const Hangul_A : Key = 0xebf;
    pub const Hangul_AE : Key = 0xec0;
    pub const Hangul_YA : Key = 0xec1;
    pub const Hangul_YAE : Key = 0xec2;
    pub const Hangul_EO : Key = 0xec3;
    pub const Hangul_E : Key = 0xec4;
    pub const Hangul_YEO : Key = 0xec5;
    pub const Hangul_YE : Key = 0xec6;
    pub const Hangul_O : Key = 0xec7;
    pub const Hangul_WA : Key = 0xec8;
    pub const Hangul_WAE : Key = 0xec9;
    pub const Hangul_OE : Key = 0xeca;
    pub const Hangul_YO : Key = 0xecb;
    pub const Hangul_U : Key = 0xecc;
    pub const Hangul_WEO : Key = 0xecd;
    pub const Hangul_WE : Key = 0xece;
    pub const Hangul_WI : Key = 0xecf;
    pub const Hangul_YU : Key = 0xed0;
    pub const Hangul_EU : Key = 0xed1;
    pub const Hangul_YI : Key = 0xed2;
    pub const Hangul_I : Key = 0xed3;
    pub const Hangul_J_Kiyeog : Key = 0xed4;
    pub const Hangul_J_SsangKiyeog : Key = 0xed5;
    pub const Hangul_J_KiyeogSios : Key = 0xed6;
    pub const Hangul_J_Nieun : Key = 0xed7;
    pub const Hangul_J_NieunJieuj : Key = 0xed8;
    pub const Hangul_J_NieunHieuh : Key = 0xed9;
    pub const Hangul_J_Dikeud : Key = 0xeda;
    pub const Hangul_J_Rieul : Key = 0xedb;
    pub const Hangul_J_RieulKiyeog : Key = 0xedc;
    pub const Hangul_J_RieulMieum : Key = 0xedd;
    pub const Hangul_J_RieulPieub : Key = 0xede;
    pub const Hangul_J_RieulSios : Key = 0xedf;
    pub const Hangul_J_RieulTieut : Key = 0xee0;
    pub const Hangul_J_RieulPhieuf : Key = 0xee1;
    pub const Hangul_J_RieulHieuh : Key = 0xee2;
    pub const Hangul_J_Mieum : Key = 0xee3;
    pub const Hangul_J_Pieub : Key = 0xee4;
    pub const Hangul_J_PieubSios : Key = 0xee5;
    pub const Hangul_J_Sios : Key = 0xee6;
    pub const Hangul_J_SsangSios : Key = 0xee7;
    pub const Hangul_J_Ieung : Key = 0xee8;
    pub const Hangul_J_Jieuj : Key = 0xee9;
    pub const Hangul_J_Cieuc : Key = 0xeea;
    pub const Hangul_J_Khieuq : Key = 0xeeb;
    pub const Hangul_J_Tieut : Key = 0xeec;
    pub const Hangul_J_Phieuf : Key = 0xeed;
    pub const Hangul_J_Hieuh : Key = 0xeee;
    pub const Hangul_RieulYeorinHieuh : Key = 0xeef;
    pub const Hangul_SunkyeongeumMieum : Key = 0xef0;
    pub const Hangul_SunkyeongeumPieub : Key = 0xef1;
    pub const Hangul_PanSios : Key = 0xef2;
    pub const Hangul_KkogjiDalrinIeung : Key = 0xef3;
    pub const Hangul_SunkyeongeumPhieuf : Key = 0xef4;
    pub const Hangul_YeorinHieuh : Key = 0xef5;
    pub const Hangul_AraeA : Key = 0xef6;
    pub const Hangul_AraeAE : Key = 0xef7;
    pub const Hangul_J_PanSios : Key = 0xef8;
    pub const Hangul_J_KkogjiDalrinIeung : Key = 0xef9;
    pub const Hangul_J_YeorinHieuh : Key = 0xefa;
    pub const Korean_Won : Key = 0xeff;
    pub const Armenian_ligature_ew : Key = 0x1000587;
    pub const Armenian_full_stop : Key = 0x1000589;
    pub const Armenian_verjaket : Key = 0x1000589;
    pub const Armenian_separation_mark : Key = 0x100055d;
    pub const Armenian_but : Key = 0x100055d;
    pub const Armenian_hyphen : Key = 0x100058a;
    pub const Armenian_yentamna : Key = 0x100058a;
    pub const Armenian_exclam : Key = 0x100055c;
    pub const Armenian_amanak : Key = 0x100055c;
    pub const Armenian_accent : Key = 0x100055b;
    pub const Armenian_shesht : Key = 0x100055b;
    pub const Armenian_question : Key = 0x100055e;
    pub const Armenian_paruyk : Key = 0x100055e;
    pub const Armenian_AYB : Key = 0x1000531;
    pub const Armenian_ayb : Key = 0x1000561;
    pub const Armenian_BEN : Key = 0x1000532;
    pub const Armenian_ben : Key = 0x1000562;
    pub const Armenian_GIM : Key = 0x1000533;
    pub const Armenian_gim : Key = 0x1000563;
    pub const Armenian_DA : Key = 0x1000534;
    pub const Armenian_da : Key = 0x1000564;
    pub const Armenian_YECH : Key = 0x1000535;
    pub const Armenian_yech : Key = 0x1000565;
    pub const Armenian_ZA : Key = 0x1000536;
    pub const Armenian_za : Key = 0x1000566;
    pub const Armenian_E : Key = 0x1000537;
    pub const Armenian_e : Key = 0x1000567;
    pub const Armenian_AT : Key = 0x1000538;
    pub const Armenian_at : Key = 0x1000568;
    pub const Armenian_TO : Key = 0x1000539;
    pub const Armenian_to : Key = 0x1000569;
    pub const Armenian_ZHE : Key = 0x100053a;
    pub const Armenian_zhe : Key = 0x100056a;
    pub const Armenian_INI : Key = 0x100053b;
    pub const Armenian_ini : Key = 0x100056b;
    pub const Armenian_LYUN : Key = 0x100053c;
    pub const Armenian_lyun : Key = 0x100056c;
    pub const Armenian_KHE : Key = 0x100053d;
    pub const Armenian_khe : Key = 0x100056d;
    pub const Armenian_TSA : Key = 0x100053e;
    pub const Armenian_tsa : Key = 0x100056e;
    pub const Armenian_KEN : Key = 0x100053f;
    pub const Armenian_ken : Key = 0x100056f;
    pub const Armenian_HO : Key = 0x1000540;
    pub const Armenian_ho : Key = 0x1000570;
    pub const Armenian_DZA : Key = 0x1000541;
    pub const Armenian_dza : Key = 0x1000571;
    pub const Armenian_GHAT : Key = 0x1000542;
    pub const Armenian_ghat : Key = 0x1000572;
    pub const Armenian_TCHE : Key = 0x1000543;
    pub const Armenian_tche : Key = 0x1000573;
    pub const Armenian_MEN : Key = 0x1000544;
    pub const Armenian_men : Key = 0x1000574;
    pub const Armenian_HI : Key = 0x1000545;
    pub const Armenian_hi : Key = 0x1000575;
    pub const Armenian_NU : Key = 0x1000546;
    pub const Armenian_nu : Key = 0x1000576;
    pub const Armenian_SHA : Key = 0x1000547;
    pub const Armenian_sha : Key = 0x1000577;
    pub const Armenian_VO : Key = 0x1000548;
    pub const Armenian_vo : Key = 0x1000578;
    pub const Armenian_CHA : Key = 0x1000549;
    pub const Armenian_cha : Key = 0x1000579;
    pub const Armenian_PE : Key = 0x100054a;
    pub const Armenian_pe : Key = 0x100057a;
    pub const Armenian_JE : Key = 0x100054b;
    pub const Armenian_je : Key = 0x100057b;
    pub const Armenian_RA : Key = 0x100054c;
    pub const Armenian_ra : Key = 0x100057c;
    pub const Armenian_SE : Key = 0x100054d;
    pub const Armenian_se : Key = 0x100057d;
    pub const Armenian_VEV : Key = 0x100054e;
    pub const Armenian_vev : Key = 0x100057e;
    pub const Armenian_TYUN : Key = 0x100054f;
    pub const Armenian_tyun : Key = 0x100057f;
    pub const Armenian_RE : Key = 0x1000550;
    pub const Armenian_re : Key = 0x1000580;
    pub const Armenian_TSO : Key = 0x1000551;
    pub const Armenian_tso : Key = 0x1000581;
    pub const Armenian_VYUN : Key = 0x1000552;
    pub const Armenian_vyun : Key = 0x1000582;
    pub const Armenian_PYUR : Key = 0x1000553;
    pub const Armenian_pyur : Key = 0x1000583;
    pub const Armenian_KE : Key = 0x1000554;
    pub const Armenian_ke : Key = 0x1000584;
    pub const Armenian_O : Key = 0x1000555;
    pub const Armenian_o : Key = 0x1000585;
    pub const Armenian_FE : Key = 0x1000556;
    pub const Armenian_fe : Key = 0x1000586;
    pub const Armenian_apostrophe : Key = 0x100055a;
    pub const Georgian_an : Key = 0x10010d0;
    pub const Georgian_ban : Key = 0x10010d1;
    pub const Georgian_gan : Key = 0x10010d2;
    pub const Georgian_don : Key = 0x10010d3;
    pub const Georgian_en : Key = 0x10010d4;
    pub const Georgian_vin : Key = 0x10010d5;
    pub const Georgian_zen : Key = 0x10010d6;
    pub const Georgian_tan : Key = 0x10010d7;
    pub const Georgian_in : Key = 0x10010d8;
    pub const Georgian_kan : Key = 0x10010d9;
    pub const Georgian_las : Key = 0x10010da;
    pub const Georgian_man : Key = 0x10010db;
    pub const Georgian_nar : Key = 0x10010dc;
    pub const Georgian_on : Key = 0x10010dd;
    pub const Georgian_par : Key = 0x10010de;
    pub const Georgian_zhar : Key = 0x10010df;
    pub const Georgian_rae : Key = 0x10010e0;
    pub const Georgian_san : Key = 0x10010e1;
    pub const Georgian_tar : Key = 0x10010e2;
    pub const Georgian_un : Key = 0x10010e3;
    pub const Georgian_phar : Key = 0x10010e4;
    pub const Georgian_khar : Key = 0x10010e5;
    pub const Georgian_ghan : Key = 0x10010e6;
    pub const Georgian_qar : Key = 0x10010e7;
    pub const Georgian_shin : Key = 0x10010e8;
    pub const Georgian_chin : Key = 0x10010e9;
    pub const Georgian_can : Key = 0x10010ea;
    pub const Georgian_jil : Key = 0x10010eb;
    pub const Georgian_cil : Key = 0x10010ec;
    pub const Georgian_char : Key = 0x10010ed;
    pub const Georgian_xan : Key = 0x10010ee;
    pub const Georgian_jhan : Key = 0x10010ef;
    pub const Georgian_hae : Key = 0x10010f0;
    pub const Georgian_he : Key = 0x10010f1;
    pub const Georgian_hie : Key = 0x10010f2;
    pub const Georgian_we : Key = 0x10010f3;
    pub const Georgian_har : Key = 0x10010f4;
    pub const Georgian_hoe : Key = 0x10010f5;
    pub const Georgian_fi : Key = 0x10010f6;
    pub const Xabovedot : Key = 0x1001e8a;
    pub const Ibreve : Key = 0x100012c;
    pub const Zstroke : Key = 0x10001b5;
    pub const Gcaron : Key = 0x10001e6;
    pub const Ocaron : Key = 0x10001d1;
    pub const Obarred : Key = 0x100019f;
    pub const xabovedot : Key = 0x1001e8b;
    pub const ibreve : Key = 0x100012d;
    pub const zstroke : Key = 0x10001b6;
    pub const gcaron : Key = 0x10001e7;
    pub const ocaron : Key = 0x10001d2;
    pub const obarred : Key = 0x1000275;
    pub const SCHWA : Key = 0x100018f;
    pub const schwa : Key = 0x1000259;
    pub const EZH : Key = 0x10001b7;
    pub const ezh : Key = 0x1000292;
    pub const Lbelowdot : Key = 0x1001e36;
    pub const lbelowdot : Key = 0x1001e37;
    pub const Abelowdot : Key = 0x1001ea0;
    pub const abelowdot : Key = 0x1001ea1;
    pub const Ahook : Key = 0x1001ea2;
    pub const ahook : Key = 0x1001ea3;
    pub const Acircumflexacute : Key = 0x1001ea4;
    pub const acircumflexacute : Key = 0x1001ea5;
    pub const Acircumflexgrave : Key = 0x1001ea6;
    pub const acircumflexgrave : Key = 0x1001ea7;
    pub const Acircumflexhook : Key = 0x1001ea8;
    pub const acircumflexhook : Key = 0x1001ea9;
    pub const Acircumflextilde : Key = 0x1001eaa;
    pub const acircumflextilde : Key = 0x1001eab;
    pub const Acircumflexbelowdot : Key = 0x1001eac;
    pub const acircumflexbelowdot : Key = 0x1001ead;
    pub const Abreveacute : Key = 0x1001eae;
    pub const abreveacute : Key = 0x1001eaf;
    pub const Abrevegrave : Key = 0x1001eb0;
    pub const abrevegrave : Key = 0x1001eb1;
    pub const Abrevehook : Key = 0x1001eb2;
    pub const abrevehook : Key = 0x1001eb3;
    pub const Abrevetilde : Key = 0x1001eb4;
    pub const abrevetilde : Key = 0x1001eb5;
    pub const Abrevebelowdot : Key = 0x1001eb6;
    pub const abrevebelowdot : Key = 0x1001eb7;
    pub const Ebelowdot : Key = 0x1001eb8;
    pub const ebelowdot : Key = 0x1001eb9;
    pub const Ehook : Key = 0x1001eba;
    pub const ehook : Key = 0x1001ebb;
    pub const Etilde : Key = 0x1001ebc;
    pub const etilde : Key = 0x1001ebd;
    pub const Ecircumflexacute : Key = 0x1001ebe;
    pub const ecircumflexacute : Key = 0x1001ebf;
    pub const Ecircumflexgrave : Key = 0x1001ec0;
    pub const ecircumflexgrave : Key = 0x1001ec1;
    pub const Ecircumflexhook : Key = 0x1001ec2;
    pub const ecircumflexhook : Key = 0x1001ec3;
    pub const Ecircumflextilde : Key = 0x1001ec4;
    pub const ecircumflextilde : Key = 0x1001ec5;
    pub const Ecircumflexbelowdot : Key = 0x1001ec6;
    pub const ecircumflexbelowdot : Key = 0x1001ec7;
    pub const Ihook : Key = 0x1001ec8;
    pub const ihook : Key = 0x1001ec9;
    pub const Ibelowdot : Key = 0x1001eca;
    pub const ibelowdot : Key = 0x1001ecb;
    pub const Obelowdot : Key = 0x1001ecc;
    pub const obelowdot : Key = 0x1001ecd;
    pub const Ohook : Key = 0x1001ece;
    pub const ohook : Key = 0x1001ecf;
    pub const Ocircumflexacute : Key = 0x1001ed0;
    pub const ocircumflexacute : Key = 0x1001ed1;
    pub const Ocircumflexgrave : Key = 0x1001ed2;
    pub const ocircumflexgrave : Key = 0x1001ed3;
    pub const Ocircumflexhook : Key = 0x1001ed4;
    pub const ocircumflexhook : Key = 0x1001ed5;
    pub const Ocircumflextilde : Key = 0x1001ed6;
    pub const ocircumflextilde : Key = 0x1001ed7;
    pub const Ocircumflexbelowdot : Key = 0x1001ed8;
    pub const ocircumflexbelowdot : Key = 0x1001ed9;
    pub const Ohornacute : Key = 0x1001eda;
    pub const ohornacute : Key = 0x1001edb;
    pub const Ohorngrave : Key = 0x1001edc;
    pub const ohorngrave : Key = 0x1001edd;
    pub const Ohornhook : Key = 0x1001ede;
    pub const ohornhook : Key = 0x1001edf;
    pub const Ohorntilde : Key = 0x1001ee0;
    pub const ohorntilde : Key = 0x1001ee1;
    pub const Ohornbelowdot : Key = 0x1001ee2;
    pub const ohornbelowdot : Key = 0x1001ee3;
    pub const Ubelowdot : Key = 0x1001ee4;
    pub const ubelowdot : Key = 0x1001ee5;
    pub const Uhook : Key = 0x1001ee6;
    pub const uhook : Key = 0x1001ee7;
    pub const Uhornacute : Key = 0x1001ee8;
    pub const uhornacute : Key = 0x1001ee9;
    pub const Uhorngrave : Key = 0x1001eea;
    pub const uhorngrave : Key = 0x1001eeb;
    pub const Uhornhook : Key = 0x1001eec;
    pub const uhornhook : Key = 0x1001eed;
    pub const Uhorntilde : Key = 0x1001eee;
    pub const uhorntilde : Key = 0x1001eef;
    pub const Uhornbelowdot : Key = 0x1001ef0;
    pub const uhornbelowdot : Key = 0x1001ef1;
    pub const Ybelowdot : Key = 0x1001ef4;
    pub const ybelowdot : Key = 0x1001ef5;
    pub const Yhook : Key = 0x1001ef6;
    pub const yhook : Key = 0x1001ef7;
    pub const Ytilde : Key = 0x1001ef8;
    pub const ytilde : Key = 0x1001ef9;
    pub const Ohorn : Key = 0x10001a0;
    pub const ohorn : Key = 0x10001a1;
    pub const Uhorn : Key = 0x10001af;
    pub const uhorn : Key = 0x10001b0;
    pub const EcuSign : Key = 0x10020a0;
    pub const ColonSign : Key = 0x10020a1;
    pub const CruzeiroSign : Key = 0x10020a2;
    pub const FFrancSign : Key = 0x10020a3;
    pub const LiraSign : Key = 0x10020a4;
    pub const MillSign : Key = 0x10020a5;
    pub const NairaSign : Key = 0x10020a6;
    pub const PesetaSign : Key = 0x10020a7;
    pub const RupeeSign : Key = 0x10020a8;
    pub const WonSign : Key = 0x10020a9;
    pub const NewSheqelSign : Key = 0x10020aa;
    pub const DongSign : Key = 0x10020ab;
    pub const EuroSign : Key = 0x20ac;
    pub const zerosuperior : Key = 0x1002070;
    pub const foursuperior : Key = 0x1002074;
    pub const fivesuperior : Key = 0x1002075;
    pub const sixsuperior : Key = 0x1002076;
    pub const sevensuperior : Key = 0x1002077;
    pub const eightsuperior : Key = 0x1002078;
    pub const ninesuperior : Key = 0x1002079;
    pub const zerosubscript : Key = 0x1002080;
    pub const onesubscript : Key = 0x1002081;
    pub const twosubscript : Key = 0x1002082;
    pub const threesubscript : Key = 0x1002083;
    pub const foursubscript : Key = 0x1002084;
    pub const fivesubscript : Key = 0x1002085;
    pub const sixsubscript : Key = 0x1002086;
    pub const sevensubscript : Key = 0x1002087;
    pub const eightsubscript : Key = 0x1002088;
    pub const ninesubscript : Key = 0x1002089;
    pub const partdifferential : Key = 0x1002202;
    pub const emptyset : Key = 0x1002205;
    pub const elementof : Key = 0x1002208;
    pub const notelementof : Key = 0x1002209;
    pub const containsas : Key = 0x100220b;
    pub const squareroot : Key = 0x100221a;
    pub const cuberoot : Key = 0x100221b;
    pub const fourthroot : Key = 0x100221c;
    pub const dintegral : Key = 0x100222c;
    pub const tintegral : Key = 0x100222d;
    pub const because : Key = 0x1002235;
    pub const approxeq : Key = 0x1002248;
    pub const notapproxeq : Key = 0x1002247;
    pub const notidentical : Key = 0x1002262;
    pub const stricteq : Key = 0x1002263;
    pub const braille_dot_1 : Key = 0xfff1;
    pub const braille_dot_2 : Key = 0xfff2;
    pub const braille_dot_3 : Key = 0xfff3;
    pub const braille_dot_4 : Key = 0xfff4;
    pub const braille_dot_5 : Key = 0xfff5;
    pub const braille_dot_6 : Key = 0xfff6;
    pub const braille_dot_7 : Key = 0xfff7;
    pub const braille_dot_8 : Key = 0xfff8;
    pub const braille_dot_9 : Key = 0xfff9;
    pub const braille_dot_10 : Key = 0xfffa;
    pub const braille_blank : Key = 0x1002800;
    pub const braille_dots_1 : Key = 0x1002801;
    pub const braille_dots_2 : Key = 0x1002802;
    pub const braille_dots_12 : Key = 0x1002803;
    pub const braille_dots_3 : Key = 0x1002804;
    pub const braille_dots_13 : Key = 0x1002805;
    pub const braille_dots_23 : Key = 0x1002806;
    pub const braille_dots_123 : Key = 0x1002807;
    pub const braille_dots_4 : Key = 0x1002808;
    pub const braille_dots_14 : Key = 0x1002809;
    pub const braille_dots_24 : Key = 0x100280a;
    pub const braille_dots_124 : Key = 0x100280b;
    pub const braille_dots_34 : Key = 0x100280c;
    pub const braille_dots_134 : Key = 0x100280d;
    pub const braille_dots_234 : Key = 0x100280e;
    pub const braille_dots_1234 : Key = 0x100280f;
    pub const braille_dots_5 : Key = 0x1002810;
    pub const braille_dots_15 : Key = 0x1002811;
    pub const braille_dots_25 : Key = 0x1002812;
    pub const braille_dots_125 : Key = 0x1002813;
    pub const braille_dots_35 : Key = 0x1002814;
    pub const braille_dots_135 : Key = 0x1002815;
    pub const braille_dots_235 : Key = 0x1002816;
    pub const braille_dots_1235 : Key = 0x1002817;
    pub const braille_dots_45 : Key = 0x1002818;
    pub const braille_dots_145 : Key = 0x1002819;
    pub const braille_dots_245 : Key = 0x100281a;
    pub const braille_dots_1245 : Key = 0x100281b;
    pub const braille_dots_345 : Key = 0x100281c;
    pub const braille_dots_1345 : Key = 0x100281d;
    pub const braille_dots_2345 : Key = 0x100281e;
    pub const braille_dots_12345 : Key = 0x100281f;
    pub const braille_dots_6 : Key = 0x1002820;
    pub const braille_dots_16 : Key = 0x1002821;
    pub const braille_dots_26 : Key = 0x1002822;
    pub const braille_dots_126 : Key = 0x1002823;
    pub const braille_dots_36 : Key = 0x1002824;
    pub const braille_dots_136 : Key = 0x1002825;
    pub const braille_dots_236 : Key = 0x1002826;
    pub const braille_dots_1236 : Key = 0x1002827;
    pub const braille_dots_46 : Key = 0x1002828;
    pub const braille_dots_146 : Key = 0x1002829;
    pub const braille_dots_246 : Key = 0x100282a;
    pub const braille_dots_1246 : Key = 0x100282b;
    pub const braille_dots_346 : Key = 0x100282c;
    pub const braille_dots_1346 : Key = 0x100282d;
    pub const braille_dots_2346 : Key = 0x100282e;
    pub const braille_dots_12346 : Key = 0x100282f;
    pub const braille_dots_56 : Key = 0x1002830;
    pub const braille_dots_156 : Key = 0x1002831;
    pub const braille_dots_256 : Key = 0x1002832;
    pub const braille_dots_1256 : Key = 0x1002833;
    pub const braille_dots_356 : Key = 0x1002834;
    pub const braille_dots_1356 : Key = 0x1002835;
    pub const braille_dots_2356 : Key = 0x1002836;
    pub const braille_dots_12356 : Key = 0x1002837;
    pub const braille_dots_456 : Key = 0x1002838;
    pub const braille_dots_1456 : Key = 0x1002839;
    pub const braille_dots_2456 : Key = 0x100283a;
    pub const braille_dots_12456 : Key = 0x100283b;
    pub const braille_dots_3456 : Key = 0x100283c;
    pub const braille_dots_13456 : Key = 0x100283d;
    pub const braille_dots_23456 : Key = 0x100283e;
    pub const braille_dots_123456 : Key = 0x100283f;
    pub const braille_dots_7 : Key = 0x1002840;
    pub const braille_dots_17 : Key = 0x1002841;
    pub const braille_dots_27 : Key = 0x1002842;
    pub const braille_dots_127 : Key = 0x1002843;
    pub const braille_dots_37 : Key = 0x1002844;
    pub const braille_dots_137 : Key = 0x1002845;
    pub const braille_dots_237 : Key = 0x1002846;
    pub const braille_dots_1237 : Key = 0x1002847;
    pub const braille_dots_47 : Key = 0x1002848;
    pub const braille_dots_147 : Key = 0x1002849;
    pub const braille_dots_247 : Key = 0x100284a;
    pub const braille_dots_1247 : Key = 0x100284b;
    pub const braille_dots_347 : Key = 0x100284c;
    pub const braille_dots_1347 : Key = 0x100284d;
    pub const braille_dots_2347 : Key = 0x100284e;
    pub const braille_dots_12347 : Key = 0x100284f;
    pub const braille_dots_57 : Key = 0x1002850;
    pub const braille_dots_157 : Key = 0x1002851;
    pub const braille_dots_257 : Key = 0x1002852;
    pub const braille_dots_1257 : Key = 0x1002853;
    pub const braille_dots_357 : Key = 0x1002854;
    pub const braille_dots_1357 : Key = 0x1002855;
    pub const braille_dots_2357 : Key = 0x1002856;
    pub const braille_dots_12357 : Key = 0x1002857;
    pub const braille_dots_457 : Key = 0x1002858;
    pub const braille_dots_1457 : Key = 0x1002859;
    pub const braille_dots_2457 : Key = 0x100285a;
    pub const braille_dots_12457 : Key = 0x100285b;
    pub const braille_dots_3457 : Key = 0x100285c;
    pub const braille_dots_13457 : Key = 0x100285d;
    pub const braille_dots_23457 : Key = 0x100285e;
    pub const braille_dots_123457 : Key = 0x100285f;
    pub const braille_dots_67 : Key = 0x1002860;
    pub const braille_dots_167 : Key = 0x1002861;
    pub const braille_dots_267 : Key = 0x1002862;
    pub const braille_dots_1267 : Key = 0x1002863;
    pub const braille_dots_367 : Key = 0x1002864;
    pub const braille_dots_1367 : Key = 0x1002865;
    pub const braille_dots_2367 : Key = 0x1002866;
    pub const braille_dots_12367 : Key = 0x1002867;
    pub const braille_dots_467 : Key = 0x1002868;
    pub const braille_dots_1467 : Key = 0x1002869;
    pub const braille_dots_2467 : Key = 0x100286a;
    pub const braille_dots_12467 : Key = 0x100286b;
    pub const braille_dots_3467 : Key = 0x100286c;
    pub const braille_dots_13467 : Key = 0x100286d;
    pub const braille_dots_23467 : Key = 0x100286e;
    pub const braille_dots_123467 : Key = 0x100286f;
    pub const braille_dots_567 : Key = 0x1002870;
    pub const braille_dots_1567 : Key = 0x1002871;
    pub const braille_dots_2567 : Key = 0x1002872;
    pub const braille_dots_12567 : Key = 0x1002873;
    pub const braille_dots_3567 : Key = 0x1002874;
    pub const braille_dots_13567 : Key = 0x1002875;
    pub const braille_dots_23567 : Key = 0x1002876;
    pub const braille_dots_123567 : Key = 0x1002877;
    pub const braille_dots_4567 : Key = 0x1002878;
    pub const braille_dots_14567 : Key = 0x1002879;
    pub const braille_dots_24567 : Key = 0x100287a;
    pub const braille_dots_124567 : Key = 0x100287b;
    pub const braille_dots_34567 : Key = 0x100287c;
    pub const braille_dots_134567 : Key = 0x100287d;
    pub const braille_dots_234567 : Key = 0x100287e;
    pub const braille_dots_1234567 : Key = 0x100287f;
    pub const braille_dots_8 : Key = 0x1002880;
    pub const braille_dots_18 : Key = 0x1002881;
    pub const braille_dots_28 : Key = 0x1002882;
    pub const braille_dots_128 : Key = 0x1002883;
    pub const braille_dots_38 : Key = 0x1002884;
    pub const braille_dots_138 : Key = 0x1002885;
    pub const braille_dots_238 : Key = 0x1002886;
    pub const braille_dots_1238 : Key = 0x1002887;
    pub const braille_dots_48 : Key = 0x1002888;
    pub const braille_dots_148 : Key = 0x1002889;
    pub const braille_dots_248 : Key = 0x100288a;
    pub const braille_dots_1248 : Key = 0x100288b;
    pub const braille_dots_348 : Key = 0x100288c;
    pub const braille_dots_1348 : Key = 0x100288d;
    pub const braille_dots_2348 : Key = 0x100288e;
    pub const braille_dots_12348 : Key = 0x100288f;
    pub const braille_dots_58 : Key = 0x1002890;
    pub const braille_dots_158 : Key = 0x1002891;
    pub const braille_dots_258 : Key = 0x1002892;
    pub const braille_dots_1258 : Key = 0x1002893;
    pub const braille_dots_358 : Key = 0x1002894;
    pub const braille_dots_1358 : Key = 0x1002895;
    pub const braille_dots_2358 : Key = 0x1002896;
    pub const braille_dots_12358 : Key = 0x1002897;
    pub const braille_dots_458 : Key = 0x1002898;
    pub const braille_dots_1458 : Key = 0x1002899;
    pub const braille_dots_2458 : Key = 0x100289a;
    pub const braille_dots_12458 : Key = 0x100289b;
    pub const braille_dots_3458 : Key = 0x100289c;
    pub const braille_dots_13458 : Key = 0x100289d;
    pub const braille_dots_23458 : Key = 0x100289e;
    pub const braille_dots_123458 : Key = 0x100289f;
    pub const braille_dots_68 : Key = 0x10028a0;
    pub const braille_dots_168 : Key = 0x10028a1;
    pub const braille_dots_268 : Key = 0x10028a2;
    pub const braille_dots_1268 : Key = 0x10028a3;
    pub const braille_dots_368 : Key = 0x10028a4;
    pub const braille_dots_1368 : Key = 0x10028a5;
    pub const braille_dots_2368 : Key = 0x10028a6;
    pub const braille_dots_12368 : Key = 0x10028a7;
    pub const braille_dots_468 : Key = 0x10028a8;
    pub const braille_dots_1468 : Key = 0x10028a9;
    pub const braille_dots_2468 : Key = 0x10028aa;
    pub const braille_dots_12468 : Key = 0x10028ab;
    pub const braille_dots_3468 : Key = 0x10028ac;
    pub const braille_dots_13468 : Key = 0x10028ad;
    pub const braille_dots_23468 : Key = 0x10028ae;
    pub const braille_dots_123468 : Key = 0x10028af;
    pub const braille_dots_568 : Key = 0x10028b0;
    pub const braille_dots_1568 : Key = 0x10028b1;
    pub const braille_dots_2568 : Key = 0x10028b2;
    pub const braille_dots_12568 : Key = 0x10028b3;
    pub const braille_dots_3568 : Key = 0x10028b4;
    pub const braille_dots_13568 : Key = 0x10028b5;
    pub const braille_dots_23568 : Key = 0x10028b6;
    pub const braille_dots_123568 : Key = 0x10028b7;
    pub const braille_dots_4568 : Key = 0x10028b8;
    pub const braille_dots_14568 : Key = 0x10028b9;
    pub const braille_dots_24568 : Key = 0x10028ba;
    pub const braille_dots_124568 : Key = 0x10028bb;
    pub const braille_dots_34568 : Key = 0x10028bc;
    pub const braille_dots_134568 : Key = 0x10028bd;
    pub const braille_dots_234568 : Key = 0x10028be;
    pub const braille_dots_1234568 : Key = 0x10028bf;
    pub const braille_dots_78 : Key = 0x10028c0;
    pub const braille_dots_178 : Key = 0x10028c1;
    pub const braille_dots_278 : Key = 0x10028c2;
    pub const braille_dots_1278 : Key = 0x10028c3;
    pub const braille_dots_378 : Key = 0x10028c4;
    pub const braille_dots_1378 : Key = 0x10028c5;
    pub const braille_dots_2378 : Key = 0x10028c6;
    pub const braille_dots_12378 : Key = 0x10028c7;
    pub const braille_dots_478 : Key = 0x10028c8;
    pub const braille_dots_1478 : Key = 0x10028c9;
    pub const braille_dots_2478 : Key = 0x10028ca;
    pub const braille_dots_12478 : Key = 0x10028cb;
    pub const braille_dots_3478 : Key = 0x10028cc;
    pub const braille_dots_13478 : Key = 0x10028cd;
    pub const braille_dots_23478 : Key = 0x10028ce;
    pub const braille_dots_123478 : Key = 0x10028cf;
    pub const braille_dots_578 : Key = 0x10028d0;
    pub const braille_dots_1578 : Key = 0x10028d1;
    pub const braille_dots_2578 : Key = 0x10028d2;
    pub const braille_dots_12578 : Key = 0x10028d3;
    pub const braille_dots_3578 : Key = 0x10028d4;
    pub const braille_dots_13578 : Key = 0x10028d5;
    pub const braille_dots_23578 : Key = 0x10028d6;
    pub const braille_dots_123578 : Key = 0x10028d7;
    pub const braille_dots_4578 : Key = 0x10028d8;
    pub const braille_dots_14578 : Key = 0x10028d9;
    pub const braille_dots_24578 : Key = 0x10028da;
    pub const braille_dots_124578 : Key = 0x10028db;
    pub const braille_dots_34578 : Key = 0x10028dc;
    pub const braille_dots_134578 : Key = 0x10028dd;
    pub const braille_dots_234578 : Key = 0x10028de;
    pub const braille_dots_1234578 : Key = 0x10028df;
    pub const braille_dots_678 : Key = 0x10028e0;
    pub const braille_dots_1678 : Key = 0x10028e1;
    pub const braille_dots_2678 : Key = 0x10028e2;
    pub const braille_dots_12678 : Key = 0x10028e3;
    pub const braille_dots_3678 : Key = 0x10028e4;
    pub const braille_dots_13678 : Key = 0x10028e5;
    pub const braille_dots_23678 : Key = 0x10028e6;
    pub const braille_dots_123678 : Key = 0x10028e7;
    pub const braille_dots_4678 : Key = 0x10028e8;
    pub const braille_dots_14678 : Key = 0x10028e9;
    pub const braille_dots_24678 : Key = 0x10028ea;
    pub const braille_dots_124678 : Key = 0x10028eb;
    pub const braille_dots_34678 : Key = 0x10028ec;
    pub const braille_dots_134678 : Key = 0x10028ed;
    pub const braille_dots_234678 : Key = 0x10028ee;
    pub const braille_dots_1234678 : Key = 0x10028ef;
    pub const braille_dots_5678 : Key = 0x10028f0;
    pub const braille_dots_15678 : Key = 0x10028f1;
    pub const braille_dots_25678 : Key = 0x10028f2;
    pub const braille_dots_125678 : Key = 0x10028f3;
    pub const braille_dots_35678 : Key = 0x10028f4;
    pub const braille_dots_135678 : Key = 0x10028f5;
    pub const braille_dots_235678 : Key = 0x10028f6;
    pub const braille_dots_1235678 : Key = 0x10028f7;
    pub const braille_dots_45678 : Key = 0x10028f8;
    pub const braille_dots_145678 : Key = 0x10028f9;
    pub const braille_dots_245678 : Key = 0x10028fa;
    pub const braille_dots_1245678 : Key = 0x10028fb;
    pub const braille_dots_345678 : Key = 0x10028fc;
    pub const braille_dots_1345678 : Key = 0x10028fd;
    pub const braille_dots_2345678 : Key = 0x10028fe;
    pub const braille_dots_12345678 : Key = 0x10028ff;
    pub const Sinh_ng : Key = 0x1000d82;
    pub const Sinh_h2 : Key = 0x1000d83;
    pub const Sinh_a : Key = 0x1000d85;
    pub const Sinh_aa : Key = 0x1000d86;
    pub const Sinh_ae : Key = 0x1000d87;
    pub const Sinh_aee : Key = 0x1000d88;
    pub const Sinh_i : Key = 0x1000d89;
    pub const Sinh_ii : Key = 0x1000d8a;
    pub const Sinh_u : Key = 0x1000d8b;
    pub const Sinh_uu : Key = 0x1000d8c;
    pub const Sinh_ri : Key = 0x1000d8d;
    pub const Sinh_rii : Key = 0x1000d8e;
    pub const Sinh_lu : Key = 0x1000d8f;
    pub const Sinh_luu : Key = 0x1000d90;
    pub const Sinh_e : Key = 0x1000d91;
    pub const Sinh_ee : Key = 0x1000d92;
    pub const Sinh_ai : Key = 0x1000d93;
    pub const Sinh_o : Key = 0x1000d94;
    pub const Sinh_oo : Key = 0x1000d95;
    pub const Sinh_au : Key = 0x1000d96;
    pub const Sinh_ka : Key = 0x1000d9a;
    pub const Sinh_kha : Key = 0x1000d9b;
    pub const Sinh_ga : Key = 0x1000d9c;
    pub const Sinh_gha : Key = 0x1000d9d;
    pub const Sinh_ng2 : Key = 0x1000d9e;
    pub const Sinh_nga : Key = 0x1000d9f;
    pub const Sinh_ca : Key = 0x1000da0;
    pub const Sinh_cha : Key = 0x1000da1;
    pub const Sinh_ja : Key = 0x1000da2;
    pub const Sinh_jha : Key = 0x1000da3;
    pub const Sinh_nya : Key = 0x1000da4;
    pub const Sinh_jnya : Key = 0x1000da5;
    pub const Sinh_nja : Key = 0x1000da6;
    pub const Sinh_tta : Key = 0x1000da7;
    pub const Sinh_ttha : Key = 0x1000da8;
    pub const Sinh_dda : Key = 0x1000da9;
    pub const Sinh_ddha : Key = 0x1000daa;
    pub const Sinh_nna : Key = 0x1000dab;
    pub const Sinh_ndda : Key = 0x1000dac;
    pub const Sinh_tha : Key = 0x1000dad;
    pub const Sinh_thha : Key = 0x1000dae;
    pub const Sinh_dha : Key = 0x1000daf;
    pub const Sinh_dhha : Key = 0x1000db0;
    pub const Sinh_na : Key = 0x1000db1;
    pub const Sinh_ndha : Key = 0x1000db3;
    pub const Sinh_pa : Key = 0x1000db4;
    pub const Sinh_pha : Key = 0x1000db5;
    pub const Sinh_ba : Key = 0x1000db6;
    pub const Sinh_bha : Key = 0x1000db7;
    pub const Sinh_ma : Key = 0x1000db8;
    pub const Sinh_mba : Key = 0x1000db9;
    pub const Sinh_ya : Key = 0x1000dba;
    pub const Sinh_ra : Key = 0x1000dbb;
    pub const Sinh_la : Key = 0x1000dbd;
    pub const Sinh_va : Key = 0x1000dc0;
    pub const Sinh_sha : Key = 0x1000dc1;
    pub const Sinh_ssha : Key = 0x1000dc2;
    pub const Sinh_sa : Key = 0x1000dc3;
    pub const Sinh_ha : Key = 0x1000dc4;
    pub const Sinh_lla : Key = 0x1000dc5;
    pub const Sinh_fa : Key = 0x1000dc6;
    pub const Sinh_al : Key = 0x1000dca;
    pub const Sinh_aa2 : Key = 0x1000dcf;
    pub const Sinh_ae2 : Key = 0x1000dd0;
    pub const Sinh_aee2 : Key = 0x1000dd1;
    pub const Sinh_i2 : Key = 0x1000dd2;
    pub const Sinh_ii2 : Key = 0x1000dd3;
    pub const Sinh_u2 : Key = 0x1000dd4;
    pub const Sinh_uu2 : Key = 0x1000dd6;
    pub const Sinh_ru2 : Key = 0x1000dd8;
    pub const Sinh_e2 : Key = 0x1000dd9;
    pub const Sinh_ee2 : Key = 0x1000dda;
    pub const Sinh_ai2 : Key = 0x1000ddb;
    pub const Sinh_o2 : Key = 0x1000ddc;
    pub const Sinh_oo2 : Key = 0x1000ddd;
    pub const Sinh_au2 : Key = 0x1000dde;
    pub const Sinh_lu2 : Key = 0x1000ddf;
    pub const Sinh_ruu2 : Key = 0x1000df2;
    pub const Sinh_luu2 : Key = 0x1000df3;
    pub const Sinh_kunddaliya : Key = 0x1000df4;
    pub const ModeLock : Key = 0x1008ff01;
    pub const MonBrightnessUp : Key = 0x1008ff02;
    pub const MonBrightnessDown : Key = 0x1008ff03;
    pub const KbdLightOnOff : Key = 0x1008ff04;
    pub const KbdBrightnessUp : Key = 0x1008ff05;
    pub const KbdBrightnessDown : Key = 0x1008ff06;
    pub const Standby : Key = 0x1008ff10;
    pub const AudioLowerVolume : Key = 0x1008ff11;
    pub const AudioMute : Key = 0x1008ff12;
    pub const AudioRaiseVolume : Key = 0x1008ff13;
    pub const AudioPlay : Key = 0x1008ff14;
    pub const AudioStop : Key = 0x1008ff15;
    pub const AudioPrev : Key = 0x1008ff16;
    pub const AudioNext : Key = 0x1008ff17;
    pub const HomePage : Key = 0x1008ff18;
    pub const Mail : Key = 0x1008ff19;
    pub const Start : Key = 0x1008ff1a;
    pub const Search : Key = 0x1008ff1b;
    pub const AudioRecord : Key = 0x1008ff1c;
    pub const Calculator : Key = 0x1008ff1d;
    pub const Memo : Key = 0x1008ff1e;
    pub const ToDoList : Key = 0x1008ff1f;
    pub const Calendar : Key = 0x1008ff20;
    pub const PowerDown : Key = 0x1008ff21;
    pub const ContrastAdjust : Key = 0x1008ff22;
    pub const RockerUp : Key = 0x1008ff23;
    pub const RockerDown : Key = 0x1008ff24;
    pub const RockerEnter : Key = 0x1008ff25;
    pub const Back : Key = 0x1008ff26;
    pub const Forward : Key = 0x1008ff27;
    pub const Stop : Key = 0x1008ff28;
    pub const Refresh : Key = 0x1008ff29;
    pub const PowerOff : Key = 0x1008ff2a;
    pub const WakeUp : Key = 0x1008ff2b;
    pub const Eject : Key = 0x1008ff2c;
    pub const ScreenSaver : Key = 0x1008ff2d;
    pub const WWW : Key = 0x1008ff2e;
    pub const Sleep : Key = 0x1008ff2f;
    pub const Favorites : Key = 0x1008ff30;
    pub const AudioPause : Key = 0x1008ff31;
    pub const AudioMedia : Key = 0x1008ff32;
    pub const MyComputer : Key = 0x1008ff33;
    pub const VendorHome : Key = 0x1008ff34;
    pub const LightBulb : Key = 0x1008ff35;
    pub const Shop : Key = 0x1008ff36;
    pub const History : Key = 0x1008ff37;
    pub const OpenURL : Key = 0x1008ff38;
    pub const AddFavorite : Key = 0x1008ff39;
    pub const HotLinks : Key = 0x1008ff3a;
    pub const BrightnessAdjust : Key = 0x1008ff3b;
    pub const Finance : Key = 0x1008ff3c;
    pub const Community : Key = 0x1008ff3d;
    pub const AudioRewind : Key = 0x1008ff3e;
    pub const BackForward : Key = 0x1008ff3f;
    pub const Launch0 : Key = 0x1008ff40;
    pub const Launch1 : Key = 0x1008ff41;
    pub const Launch2 : Key = 0x1008ff42;
    pub const Launch3 : Key = 0x1008ff43;
    pub const Launch4 : Key = 0x1008ff44;
    pub const Launch5 : Key = 0x1008ff45;
    pub const Launch6 : Key = 0x1008ff46;
    pub const Launch7 : Key = 0x1008ff47;
    pub const Launch8 : Key = 0x1008ff48;
    pub const Launch9 : Key = 0x1008ff49;
    pub const LaunchA : Key = 0x1008ff4a;
    pub const LaunchB : Key = 0x1008ff4b;
    pub const LaunchC : Key = 0x1008ff4c;
    pub const LaunchD : Key = 0x1008ff4d;
    pub const LaunchE : Key = 0x1008ff4e;
    pub const LaunchF : Key = 0x1008ff4f;
    pub const ApplicationLeft : Key = 0x1008ff50;
    pub const ApplicationRight : Key = 0x1008ff51;
    pub const Book : Key = 0x1008ff52;
    pub const CD : Key = 0x1008ff53;
    pub const WindowClear : Key = 0x1008ff55;
    pub const Close : Key = 0x1008ff56;
    pub const Copy : Key = 0x1008ff57;
    pub const Cut : Key = 0x1008ff58;
    pub const Display : Key = 0x1008ff59;
    pub const DOS : Key = 0x1008ff5a;
    pub const Documents : Key = 0x1008ff5b;
    pub const Excel : Key = 0x1008ff5c;
    pub const Explorer : Key = 0x1008ff5d;
    pub const Game : Key = 0x1008ff5e;
    pub const Go : Key = 0x1008ff5f;
    pub const iTouch : Key = 0x1008ff60;
    pub const LogOff : Key = 0x1008ff61;
    pub const Market : Key = 0x1008ff62;
    pub const Meeting : Key = 0x1008ff63;
    pub const MenuKB : Key = 0x1008ff65;
    pub const MenuPB : Key = 0x1008ff66;
    pub const MySites : Key = 0x1008ff67;
    pub const New : Key = 0x1008ff68;
    pub const News : Key = 0x1008ff69;
    pub const OfficeHome : Key = 0x1008ff6a;
    pub const Open : Key = 0x1008ff6b;
    pub const Option : Key = 0x1008ff6c;
    pub const Paste : Key = 0x1008ff6d;
    pub const Phone : Key = 0x1008ff6e;
    pub const Reply : Key = 0x1008ff72;
    pub const Reload : Key = 0x1008ff73;
    pub const RotateWindows : Key = 0x1008ff74;
    pub const RotationPB : Key = 0x1008ff75;
    pub const RotationKB : Key = 0x1008ff76;
    pub const Save : Key = 0x1008ff77;
    pub const ScrollUp : Key = 0x1008ff78;
    pub const ScrollDown : Key = 0x1008ff79;
    pub const ScrollClick : Key = 0x1008ff7a;
    pub const Send : Key = 0x1008ff7b;
    pub const Spell : Key = 0x1008ff7c;
    pub const SplitScreen : Key = 0x1008ff7d;
    pub const Support : Key = 0x1008ff7e;
    pub const TaskPane : Key = 0x1008ff7f;
    pub const Terminal : Key = 0x1008ff80;
    pub const Tools : Key = 0x1008ff81;
    pub const Travel : Key = 0x1008ff82;
    pub const UserPB : Key = 0x1008ff84;
    pub const User1KB : Key = 0x1008ff85;
    pub const User2KB : Key = 0x1008ff86;
    pub const Video : Key = 0x1008ff87;
    pub const WheelButton : Key = 0x1008ff88;
    pub const Word : Key = 0x1008ff89;
    pub const Xfer : Key = 0x1008ff8a;
    pub const ZoomIn : Key = 0x1008ff8b;
    pub const ZoomOut : Key = 0x1008ff8c;
    pub const Away : Key = 0x1008ff8d;
    pub const Messenger : Key = 0x1008ff8e;
    pub const WebCam : Key = 0x1008ff8f;
    pub const MailForward : Key = 0x1008ff90;
    pub const Pictures : Key = 0x1008ff91;
    pub const Music : Key = 0x1008ff92;
    pub const Battery : Key = 0x1008ff93;
    pub const Bluetooth : Key = 0x1008ff94;
    pub const WLAN : Key = 0x1008ff95;
    pub const UWB : Key = 0x1008ff96;
    pub const AudioForward : Key = 0x1008ff97;
    pub const AudioRepeat : Key = 0x1008ff98;
    pub const AudioRandomPlay : Key = 0x1008ff99;
    pub const Subtitle : Key = 0x1008ff9a;
    pub const AudioCycleTrack : Key = 0x1008ff9b;
    pub const CycleAngle : Key = 0x1008ff9c;
    pub const FrameBack : Key = 0x1008ff9d;
    pub const FrameForward : Key = 0x1008ff9e;
    pub const Time : Key = 0x1008ff9f;
    pub const SelectButton : Key = 0x1008ffa0;
    pub const View : Key = 0x1008ffa1;
    pub const TopMenu : Key = 0x1008ffa2;
    pub const Red : Key = 0x1008ffa3;
    pub const Green : Key = 0x1008ffa4;
    pub const Yellow : Key = 0x1008ffa5;
    pub const Blue : Key = 0x1008ffa6;
    pub const Suspend : Key = 0x1008ffa7;
    pub const Hibernate : Key = 0x1008ffa8;
    pub const TouchpadToggle : Key = 0x1008ffa9;
    pub const TouchpadOn : Key = 0x1008ffb0;
    pub const TouchpadOff : Key = 0x1008ffb1;
    pub const AudioMicMute : Key = 0x1008ffb2;
    pub const Switch_VT_1 : Key = 0x1008fe01;
    pub const Switch_VT_2 : Key = 0x1008fe02;
    pub const Switch_VT_3 : Key = 0x1008fe03;
    pub const Switch_VT_4 : Key = 0x1008fe04;
    pub const Switch_VT_5 : Key = 0x1008fe05;
    pub const Switch_VT_6 : Key = 0x1008fe06;
    pub const Switch_VT_7 : Key = 0x1008fe07;
    pub const Switch_VT_8 : Key = 0x1008fe08;
    pub const Switch_VT_9 : Key = 0x1008fe09;
    pub const Switch_VT_10 : Key = 0x1008fe0a;
    pub const Switch_VT_11 : Key = 0x1008fe0b;
    pub const Switch_VT_12 : Key = 0x1008fe0c;
    pub const Ungrab : Key = 0x1008fe20;
    pub const ClearGrab : Key = 0x1008fe21;
    pub const Next_VMode : Key = 0x1008fe22;
    pub const Prev_VMode : Key = 0x1008fe23;
    pub const LogWindowTree : Key = 0x1008fe24;
    pub const LogGrabInfo : Key = 0x1008fe25;
}
