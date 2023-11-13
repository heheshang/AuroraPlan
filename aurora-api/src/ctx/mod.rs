#![allow(unused)]
// region:    --- Modules

use aurora_common::{core_error::error::Error, core_results::results::Result};

// endregion: --- Modules

#[derive(Clone, Debug)]
pub struct Ctx {
    pub(crate) user_id: i32,
}

// Constructors.
impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 }
    }

    pub fn new(user_id: i32) -> Result<Self> {
        if user_id == 0 {
            Err(Error::UserNotExist)
        } else {
            Ok(Self { user_id })
        }
    }
}

// Property Accessors.
impl Ctx {
    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}
