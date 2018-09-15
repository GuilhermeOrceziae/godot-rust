//! # Rust bindings for the Godot game engine
//!
//! This crate contains high-level wrappers around the Godot game engine's gdnaive API.
//! Some of the types were automatically generated from the engine's JSON API description,
//! and some other types are hand made wrappers around the core C types.
//!
//! ## Memory management
//!
//! ### Reference counting
//!
//! A lot of the types provided by the engine are internally reference counted and
//! allow mutable aliasing.
//! In rust parlance this means that a type such as `gdnative::ConcavePolygonShape2D`
//! is functionally equivalent to a `Rc<Cell<Something>>` rather than `Rc<Something>`.
//!
//! Since it is easy to expect containers and other types to allocate a copy of their
//! content when using the `Clone` trait, most of these types do not implement `Clone`
//! and instead provide a `new_ref(&self) -> Self` method to create references to the
//! same collection or object.
//!
//! ### Manually managed objects
//!
//! Some types are manually managed. This means that ownership can be passed to the
//! engine or the object must be carefully deallocated using the object's `free`  method.
//!

// TODO: document feature flags

// TODO: add logo using #![doc(html_logo_url = "https://<url>")]

// TODO: currently the generated classes are not showing in the the gdnative crate
// documentation, and are only appearing in the sub-crates. It would make the doc
// a lot easier to navigate if we could gather all classes here.

#[macro_use]
extern crate gdnative_core;
extern crate gdnative_common;
#[cfg(feature="graphics")] extern crate gdnative_graphics;
#[cfg(feature="physics")] extern crate gdnative_physics;
#[cfg(feature="network")] extern crate gdnative_network;
#[cfg(feature="audio")] extern crate gdnative_audio;
#[cfg(feature="video")] extern crate gdnative_video;
#[cfg(feature="editor")] extern crate gdnative_editor;
#[cfg(feature="arvr")] extern crate gdnative_arvr;
#[cfg(feature="visual_script")] extern crate gdnative_visual_script;
#[cfg(feature="animation")] extern crate gdnative_animation;
#[cfg(feature="input")] extern crate gdnative_input;
#[cfg(feature="ui")] extern crate gdnative_ui;

#[doc(inline)] pub use gdnative_core::*;
#[doc(inline)] pub use gdnative_common::*;
#[doc(inline)] #[cfg(feature="graphics")] pub use gdnative_graphics::*;
#[doc(inline)] #[cfg(feature="physics")] pub use gdnative_physics::*;
#[doc(inline)] #[cfg(feature="network")] pub use gdnative_network::*;
#[doc(inline)] #[cfg(feature="audio")] pub use gdnative_audio::*;
#[doc(inline)] #[cfg(feature="video")] pub use gdnative_video::*;
#[doc(inline)] #[cfg(feature="editor")] pub use gdnative_editor::*;
#[doc(inline)] #[cfg(feature="arvr")] pub use gdnative_arvr::*;
#[doc(inline)] #[cfg(feature="visual_script")] pub use gdnative_visual_script::*;
#[doc(inline)] #[cfg(feature="animation")] pub use gdnative_animation::*;
#[doc(inline)] #[cfg(feature="input")] pub use gdnative_input::*;
#[doc(inline)] #[cfg(feature="ui")] pub use gdnative_ui::*;
