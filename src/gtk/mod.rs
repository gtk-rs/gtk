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

pub use self::rt::{
	init,
	main,
	main_quit,
	main_level,
	main_iteration,
	main_iteration_do
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
	MenuButton,
	ColorButton,
	LinkButton,
	Adjustment,
	ScaleButton,
	VolumeButton,
	Grid,
	EntryBuffer,
	Entry,
	SearchEntry,
	Switch,
	Scale,
	LevelBar,
	SearchBar,
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
	MenuToolButton
};

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
	MessageType
};

pub use self::types::{
	Tooltip,
	WidgetHelpType
};

pub mod cast;
pub mod version;
pub mod traits;
pub mod signals;

mod rt;
mod widgets;
mod enums;
mod types;