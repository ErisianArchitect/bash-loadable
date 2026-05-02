use crate::{
    macros::{
        cenum,
    },
};

// TODO: I have no idea what the documentation should be.
cenum!{
    pub enum MatchFlags {
        /// Match any.
        ANY           = 0x000,
        /// Match beginning, idk.
        BEGINNING = 0x001,
        /// Match end, idk.
        END           = 0x002,
        /// Typemask. Duh.
        TYPE_MASK     = 0x003,
        /// Glob replacement. I think.
        GLOBREP       = 0x010,
        /// Quoted.
        QUOTED        = 0x020,
        /// Assign RHS.
        ASSIGN_RHS    = 0x040,
        /// Star substitution.
        STARSUB       = 0x080,
        /// For pattern substitution, expand replacement.
        EXPREP        = 0x100,
    }
}