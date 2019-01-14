//! Flash memory

use stm32l4::stm32l4x5::{flash, FLASH};

use crate::common::Constrain;

impl Constrain<Parts> for FLASH {
    fn constrain(self) -> Parts {
        Parts { acr: ACR(()) }
    }
}

/// Constrained FLASH peripheral
pub struct Parts {
    /// Opaque ACR register
    pub acr: ACR,
}

/// Opaque ACR register
pub struct ACR(());
impl ACR {
    pub fn acr(&mut self) -> &flash::ACR {
        unsafe { &(*FLASH::ptr()).acr }
    }
}
