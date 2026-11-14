//! Port of experience, enemy-party setup and trainer-battle switching
//! (`engine/battle/core.asm`, `engine/pokemon/`). See docs/engine.md.

#![allow(unused_imports)]

use super::battle::{Engine, Flow, PARTY_LEN, PP_MASK, percent};
use super::rom;
use super::rsym as r;
use super::sym::*;
use crate::defs::*;

impl Engine {}
