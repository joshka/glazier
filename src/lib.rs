// Copyright 2018 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Platform abstraction for druid toolkit.
//!
//! `glazier` is an abstraction around a given platform UI & application
//! framework. It provides common types, which then defer to a platform-defined
//! implementation.
//!
//! # Env
//!
//! For testing and debugging, `glazier` can change its behavior based on environment
//! variables. Here is a list of environment variables that `glazier` supports:
//!
//! - `GLAZIER_OVERRIDE_SCALE`: If this is set and `glazier` is using the `x11` backend,
//! it will use this `f64` value as the scaling factor for the display. This is intended
//! to be used by developers to debug HiDPI issues without having to change their
//! system settings.

#![warn(rustdoc::broken_intra_doc_links)]
#![allow(clippy::new_without_default)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/linebender/druid/screenshots/images/doc_logo.png"
)]

pub use kurbo;

// Reexport the version of `raw_window_handle` we are using.
pub use raw_window_handle;

#[macro_use]
mod util;

mod application;
mod backend;
mod clipboard;
mod common_util;
mod dialog;
mod error;
mod hotkey;
mod keyboard;
mod menu;
mod mouse;
mod pointer;
mod region;
mod scale;
mod screen;
mod window;

pub mod platform;
pub mod text;

pub use application::{AppHandle, AppHandler, Application};
pub use clipboard::{Clipboard, ClipboardFormat, FormatId};
pub use common_util::Counter;
pub use dialog::{FileDialogOptions, FileInfo, FileSpec};
pub use error::Error;
pub use hotkey::{HotKey, RawMods, SysMods};
pub use keyboard::{Code, IntoKey, KbKey, KeyEvent, KeyState, Location, Modifiers, ModifiersExt};
pub use menu::Menu;
pub use mouse::{Cursor, CursorDesc};
pub use pointer::{
    MouseInfo, PenInclination, PenInfo, PointerButton, PointerButtons, PointerEvent, PointerId,
    PointerType, TouchInfo,
};
pub use region::Region;
pub use scale::{Scalable, Scale, ScaledArea};
pub use screen::{Monitor, Screen};
pub use window::{
    FileDialogToken, IdleHandle, IdleToken, TextFieldToken, TimerToken, WinHandler, WindowBuilder,
    WindowHandle, WindowLevel, WindowState,
};

pub use keyboard_types;
