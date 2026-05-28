use noansi::Noansi;

use crate::util::ffi::BashStatus;
use std::fmt::{Display, Formatter};

pub trait BuiltinError: Display {
    fn status(&self) -> BashStatus {
        BashStatus::FAILURE
    }

    fn print(&self, strip_ansi: bool, f: &mut Formatter<'_>) -> std::fmt::Result {
        if strip_ansi {
            let ansi_text = format!("{self}");
            write!(f, "{}", Noansi(ansi_text))
        } else {
            write!(f, "{}", self)
        }
    }
}

pub struct BashFailure;

impl Display for BashFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unspecified Error.")
    }
}

impl BuiltinError for &'static str {}
impl BuiltinError for String {}
impl BuiltinError for Box<str> {}

impl BuiltinError for BashFailure {
    fn print(&self, strip_ansi: bool, f: &mut Formatter<'_>) -> std::fmt::Result {
        if strip_ansi {
            write!(f, "Unspecified Error.")
        } else {
            write!(f, "\x1b[38;2;255;0;0mUnspecified Error.\x1b[39m")
        }
    }
}
