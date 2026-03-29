use crate::{
    macros::{
        cenum,
    },
};

// TODO: I have no idea what the documentation should be.
cenum!{
    pub enum MatchFlags {
        /// Match any.
        ANY         any         = 0x000,
        /// Match beginning, idk.
        BEGINNING   beginning   = 0x001,
        /// Match end, idk.
        END         end         = 0x002,
        /// Typemask. Duh.
        TYPE_MASK   type_mask   = 0x003,
        /// Glob replacement. I think.
        GLOBREP     globrep     = 0x010,
        /// Quoted.
        QUOTED      quoted      = 0x020,
        /// Assign RHS.
        ASSIGN_RHS  assign_rhs  = 0x040,
        /// Star substitution.
        STARSUB     starsub     = 0x080,
        /// For pattern substitution, expand replacement.
        EXPREP      exprep      = 0x100,
    }
}