use core::{
    ffi::{
        c_int,
    },
};
use crate::{
    ffi::word::Word, macros::cenum
};

#[repr(C)]
pub enum CommandType {
    For,
    Case,
    While,
    If,
    Simple,
    Select,
    Connection,
    FunctionDef,
    Until,
    Group,
    Arith,
    Cond,
    ArithFor,
    Subshell,
    Coproc,
}

cenum!{
    pub enum CommandFlags {
        /// User wants a subshell: ( command )
        WANT_SUBSHELL       want_subshell       = 0x0001,
        /// Shell needs to force a subshell.
        FORCE_SUBSHELL      force_subshell      = 0x0002,
        /// Invert the exit value.
        INVERT_RETURN       invert_return       = 0x0004,
        /// Ignore the exit value. For set -e.
        IGNORE_RETURN       ignore_return       = 0x0008,
        /// Ignore functions during command lookup.
        NO_FUNCTIONS        no_functions        = 0x0010,
        /// Do not expand the command words.
        INHIBIT_EXPANSION   inhibit_expansion   = 0x0020,
        /// Don't fork; just call execve.
        NO_FORK             no_fork             = 0x0040,
        /// Time a pipeline.
        TIME_PIPELINE       time_pipeline       = 0x0080,
        /// time -p; use POSIX.2 time output spec.
        TIME_POSIX          time_posix          = 0x0100,
        /// command &
        AMPERSAND           ampersand           = 0x0200,
        /// Async command needs implicit </dev/null
        STDIN_REDIR         stdin_redir         = 0x0400,
        /// Command executed by `command` builtin.
        COMMAND_BUILTIN     command_builtin     = 0x0800,
        /// COPROC_SUBSHELL!!!
        COPROC_SUBSHELL     coproc_subshell     = 0x1000,
        /// Last pipe.
        LAST_PIPE           last_pipe           = 0x2000,
        /// Use standard path for command lookup.
        STD_PATH            std_path            = 0x4000,
        /// Try to optimize this simple command.
        TRY_OPTIMIZING      try_optimizing      = 0x8000,
    }
}

#[repr(C)]
pub struct Redirectee {
    dest: c_int,
    word: Word,
}

#[repr(C)]
pub struct Redirect {

}

#[repr(C)]
pub struct Command {
    pub ty: CommandType,
    pub flags: CommandFlags,
    pub lineno: c_int,
    // TODO: find REDIRECT struct in command.h
    pub redirects: *const (),
    pub command: *const (),
}