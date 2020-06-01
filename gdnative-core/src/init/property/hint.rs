//! Strongly typed property hints.

use std::fmt::{self, Write};
use std::ops::RangeInclusive;

use crate::GodotString;
use crate::VariantType;

use super::ExportInfo;

/// Hints that an integer or float property should be within an inclusive range.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use gdnative_core::init::property::hint::RangeHint;
///
/// let hint: RangeHint<f64> = RangeHint::new(0.0, 20.0).or_greater();
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct RangeHint<T> {
    /// Minimal value, inclusive
    pub min: T,
    /// Maximal value, inclusive
    pub max: T,
    /// Optional step value for the slider
    pub step: Option<T>,
    /// Allow manual input above the `max` value
    pub or_greater: bool,
    /// Allow manual input below the `min` value
    pub or_lesser: bool,
}

impl<T> RangeHint<T>
where
    T: fmt::Display,
{
    /// Creates a new `RangeHint`.
    pub fn new(min: T, max: T) -> Self {
        RangeHint {
            min,
            max,
            step: None,
            or_greater: false,
            or_lesser: false,
        }
    }

    /// Builder-style method that returns `self` with the given step.
    pub fn with_step(mut self, step: T) -> Self {
        self.step.replace(step);
        self
    }

    /// Builder-style method that returns `self` with the `or_greater` hint.
    pub fn or_greater(mut self) -> Self {
        self.or_greater = true;
        self
    }

    /// Builder-style method that returns `self` with the `or_lesser` hint.
    pub fn or_lesser(mut self) -> Self {
        self.or_lesser = true;
        self
    }

    /// Formats the hint as a Godot hint string.
    pub fn to_godot_hint_string(&self) -> GodotString {
        let mut s = String::new();

        write!(s, "{},{}", self.min, self.max).unwrap();
        if let Some(step) = &self.step {
            write!(s, ",{}", step).unwrap();
        }

        if self.or_greater {
            s.push_str(",or_greater");
        }
        if self.or_lesser {
            s.push_str(",or_lesser");
        }

        s.into()
    }
}

impl<T> From<RangeInclusive<T>> for RangeHint<T>
where
    T: fmt::Display,
{
    fn from(range: RangeInclusive<T>) -> Self {
        let (min, max) = range.into_inner();
        RangeHint::new(min, max)
    }
}

/// Hints that an integer, float or string property is an enumerated value to pick in a list.
///
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use gdnative_core::init::property::hint::EnumHint;
///
/// let hint = EnumHint::new(vec!["Foo".into(), "Bar".into(), "Baz".into()]);
/// ```
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct EnumHint {
    values: Vec<String>,
}

impl EnumHint {
    pub fn new(values: Vec<String>) -> Self {
        EnumHint { values }
    }

    /// Formats the hint as a Godot hint string.
    pub fn to_godot_hint_string(&self) -> GodotString {
        let mut s = String::new();

        let mut iter = self.values.iter();

        if let Some(first) = iter.next() {
            write!(s, "{}", first).unwrap();
        }

        for rest in iter {
            write!(s, ",{}", rest).unwrap();
        }

        s.into()
    }
}

/// Possible hints for integers.
#[derive(Clone, Debug)]
pub enum IntHint<T> {
    /// Hints that an integer or float property should be within a range.
    Range(RangeHint<T>),
    /// Hints that an integer or float property should be within an exponential range.
    ExpRange(RangeHint<T>),
    /// Hints that an integer, float or string property is an enumerated value to pick in a list.
    Enum(EnumHint),
    /// Hints that an integer property is a bitmask with named bit flags.
    Flags(EnumHint),
    /// Hints that an integer property is a bitmask using the optionally named 2D render layers.
    Layers2DRender,
    /// Hints that an integer property is a bitmask using the optionally named 2D physics layers.
    Layers2DPhysics,
    /// Hints that an integer property is a bitmask using the optionally named 3D render layers.
    Layers3DRender,
    /// Hints that an integer property is a bitmask using the optionally named 3D physics layers.
    Layers3DPhysics,
}

impl<T> IntHint<T>
where
    T: fmt::Display,
{
    pub fn export_info(self) -> ExportInfo {
        use IntHint as IH;

        let hint_kind = match &self {
            IH::Range(_) => sys::godot_property_hint_GODOT_PROPERTY_HINT_RANGE,
            IH::ExpRange(_) => sys::godot_property_hint_GODOT_PROPERTY_HINT_EXP_RANGE,
            IH::Enum(_) => sys::godot_property_hint_GODOT_PROPERTY_HINT_ENUM,
            IH::Flags(_) => sys::godot_property_hint_GODOT_PROPERTY_HINT_FLAGS,
            IH::Layers2DRender => sys::godot_property_hint_GODOT_PROPERTY_HINT_LAYERS_2D_RENDER,
            IH::Layers2DPhysics => sys::godot_property_hint_GODOT_PROPERTY_HINT_LAYERS_2D_PHYSICS,
            IH::Layers3DRender => sys::godot_property_hint_GODOT_PROPERTY_HINT_LAYERS_3D_RENDER,
            IH::Layers3DPhysics => sys::godot_property_hint_GODOT_PROPERTY_HINT_LAYERS_3D_PHYSICS,
        };

        let hint_string = match self {
            IH::Range(range) | IH::ExpRange(range) => range.to_godot_hint_string(),
            IH::Enum(e) | IH::Flags(e) => e.to_godot_hint_string(),
            _ => GodotString::new(),
        };

        ExportInfo {
            variant_type: VariantType::I64,
            hint_kind,
            hint_string,
        }
    }
}

impl<T> From<RangeHint<T>> for IntHint<T>
where
    T: fmt::Display,
{
    fn from(hint: RangeHint<T>) -> Self {
        Self::Range(hint)
    }
}

impl<T> From<RangeInclusive<T>> for IntHint<T>
where
    T: fmt::Display,
{
    fn from(range: RangeInclusive<T>) -> Self {
        Self::Range(range.into())
    }
}

impl<T> From<EnumHint> for IntHint<T> {
    fn from(hint: EnumHint) -> Self {
        Self::Enum(hint)
    }
}

/// Hints that a float property should be edited via an exponential easing function.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct ExpEasingHint {
    /// Flip the curve horizontally.
    pub is_attenuation: bool,
    /// Also include in/out easing.
    pub is_in_out: bool,
}

impl ExpEasingHint {
    pub fn new() -> Self {
        Self::default()
    }

    /// Formats the hint as a Godot hint string.
    pub fn to_godot_hint_string(&self) -> GodotString {
        let mut s = String::new();

        if self.is_attenuation {
            s.push_str("attenuation");
        }

        if self.is_in_out {
            if self.is_attenuation {
                s.push(',');
            }
            s.push_str("inout");
        }

        s.into()
    }
}

/// Possible hints for floats.
#[derive(Clone, Debug)]
pub enum FloatHint<T> {
    /// Hints that an integer or float property should be within a range.
    Range(RangeHint<T>),
    /// Hints that an integer or float property should be within an exponential range.
    ExpRange(RangeHint<T>),
    /// Hints that an integer, float or string property is an enumerated value to pick in a list.
    Enum(EnumHint),
    /// Hints that a float property should be edited via an exponential easing function.
    ExpEasing(ExpEasingHint),
}

impl<T> FloatHint<T>
where
    T: fmt::Display,
{
    pub fn export_info(self) -> ExportInfo {
        use FloatHint as FH;

        let hint_kind = match &self {
            FH::Range(_) => sys::godot_property_hint_GODOT_PROPERTY_HINT_RANGE,
            FH::ExpRange(_) => sys::godot_property_hint_GODOT_PROPERTY_HINT_EXP_RANGE,
            FH::Enum(_) => sys::godot_property_hint_GODOT_PROPERTY_HINT_ENUM,
            FH::ExpEasing(_) => sys::godot_property_hint_GODOT_PROPERTY_HINT_EXP_EASING,
        };

        let hint_string = match self {
            FH::Range(range) | FH::ExpRange(range) => range.to_godot_hint_string(),
            FH::Enum(e) => e.to_godot_hint_string(),
            FH::ExpEasing(e) => e.to_godot_hint_string(),
        };

        ExportInfo {
            variant_type: VariantType::F64,
            hint_kind,
            hint_string,
        }
    }
}

impl<T> From<RangeHint<T>> for FloatHint<T>
where
    T: fmt::Display,
{
    fn from(hint: RangeHint<T>) -> Self {
        Self::Range(hint)
    }
}

impl<T> From<RangeInclusive<T>> for FloatHint<T>
where
    T: fmt::Display,
{
    fn from(range: RangeInclusive<T>) -> Self {
        Self::Range(range.into())
    }
}

impl<T> From<EnumHint> for FloatHint<T> {
    fn from(hint: EnumHint) -> Self {
        Self::Enum(hint)
    }
}

impl<T> From<ExpEasingHint> for FloatHint<T> {
    fn from(hint: ExpEasingHint) -> Self {
        Self::ExpEasing(hint)
    }
}

/// Possible hints for strings.
#[derive(Clone, Debug)]
pub enum StringHint {
    /// Hints that an integer, float or string property is an enumerated value to pick in a list.
    Enum(EnumHint),
    /// Hints that a string property is a path to a file.
    File(EnumHint),
    /// Hints that a string property is an absolute path to a file outside the project folder.
    GlobalFile(EnumHint),
    /// Hints that a string property is a path to a directory.
    Dir,
    /// Hints that a string property is an absolute path to a directory outside the project folder.
    GlobalDir,
    /// Hints that a string property is text with line breaks.
    Multiline,
    /// Hints that a string property should have a placeholder text visible on its input field, whenever the property is empty.
    Placeholder { placeholder: String },
}

impl StringHint {
    pub fn export_info(self) -> ExportInfo {
        use StringHint as SH;

        let hint_kind = match &self {
            SH::Enum(_) => sys::godot_property_hint_GODOT_PROPERTY_HINT_ENUM,
            SH::File(_) => sys::godot_property_hint_GODOT_PROPERTY_HINT_FILE,
            SH::GlobalFile(_) => sys::godot_property_hint_GODOT_PROPERTY_HINT_GLOBAL_FILE,
            SH::Dir => sys::godot_property_hint_GODOT_PROPERTY_HINT_DIR,
            SH::GlobalDir => sys::godot_property_hint_GODOT_PROPERTY_HINT_GLOBAL_DIR,
            SH::Multiline => sys::godot_property_hint_GODOT_PROPERTY_HINT_MULTILINE_TEXT,
            SH::Placeholder { .. } => sys::godot_property_hint_GODOT_PROPERTY_HINT_PLACEHOLDER_TEXT,
        };

        let hint_string = match self {
            SH::Enum(e) | SH::File(e) | SH::GlobalFile(e) => e.to_godot_hint_string(),
            SH::Placeholder { placeholder } => placeholder.into(),
            _ => GodotString::new(),
        };

        ExportInfo {
            variant_type: VariantType::GodotString,
            hint_kind,
            hint_string,
        }
    }
}

/// Possible hints for `Color`.
#[derive(Clone, Debug)]
pub enum ColorHint {
    /// Hints that a color property should be edited without changing its alpha component.
    NoAlpha,
}

impl ColorHint {
    pub fn export_info(self) -> ExportInfo {
        ExportInfo {
            variant_type: VariantType::Color,
            hint_kind: match self {
                ColorHint::NoAlpha => sys::godot_property_hint_GODOT_PROPERTY_HINT_COLOR_NO_ALPHA,
            },
            hint_string: GodotString::new(),
        }
    }
}
