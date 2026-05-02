use crate::cenum;


cenum! {
    pub enum EvalFlags {
        NONINT = 0x001,
        INTERACT = 0x002,
        NO_HIST = 0x004,
        NO_FREE = 0x008,
        RESET_LINE = 0x010,
        PARSE_ONLY = 0x020,
        NO_LONG_JUMP = 0x040,
        /// Allow only function definitions.
        FUNCDEF = 0x080,
        /// Allow only a single command.
        ONE_CMD = 0x100,
        /// Inhibit history expansion.
        NO_HIST_EXP = 0x200,
        /// Don't try to set optimization flags.
        NO_OPTIMIZE = 0x400,
        /// Wants job notifications.
        NOTIFY = 0x800,
    }
}