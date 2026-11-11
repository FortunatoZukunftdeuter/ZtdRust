#![allow(unused_imports)]

pub use std::format as f;

pub use crate::bayern::{
    /* Traits */
    UnwrapOrBayern,
    UnwrapOrByeOption,
    UnwrapOrByePossible,
    UnwrapOrByeResult
};

pub use crate::{
    /* Macros */
    bye_fnc,

    bye_msg,
    bye_msgd,
    bye_msgln,
    bye_msgdln,

    bye_expl,
    bye_expld,
    bye_explln,
    bye_expldln
};

use crate::{
    printd,
    prints,
    printds,
    printdln,

    printj,
    printsj,
    printdj,
    printdsj,
    printlnj,
    printdlnj
};

pub use crate::variants::{
    Possible,

    ResultToPossible,
    OptionToPossible
};
pub use crate::vfs::{
    ZtdHardlink
};

pub use crate::Debugger;


pub use crate::amplified::{
    str::traits::*,
    prcs::traits::*
};
