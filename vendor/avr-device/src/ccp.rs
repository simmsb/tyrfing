//! Configuration change protected (CCP) register definitions

pub use crate::generic::ProtectedWritable;

#[cfg(feature = "attiny1616")]
pub mod attiny1616 {
    use crate::generic::{UnlockRegister, Protected};

    // Mark the CPU.CCP register with the UnlockRegister trait so that it can be used to unlock the below defined registers
    impl UnlockRegister for crate::attiny1616::cpu::ccp::CCP_SPEC { const PTR: *mut u8 = 0x34 as *mut u8; }

    // Configuration change protected registers in NVMCTRL
    impl Protected for crate::attiny1616::nvmctrl::ctrla::CTRLA_SPEC { const MAGIC: u8 = 0x9D; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }
    impl Protected for crate::attiny1616::nvmctrl::ctrlb::CTRLB_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }

    // Configuration change protected registers in CLKCTRL
    impl Protected for crate::attiny1616::clkctrl::mclkctrlb::MCLKCTRLB_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }
    impl Protected for crate::attiny1616::clkctrl::mclklock::MCLKLOCK_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }
    impl Protected for crate::attiny1616::clkctrl::xosc32kctrla::XOSC32KCTRLA_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }
    impl Protected for crate::attiny1616::clkctrl::mclkctrla::MCLKCTRLA_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }
    impl Protected for crate::attiny1616::clkctrl::osc20mctrla::OSC20MCTRLA_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }
    impl Protected for crate::attiny1616::clkctrl::osc20mcaliba::OSC20MCALIBA_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }
    impl Protected for crate::attiny1616::clkctrl::osc20mcalibb::OSC20MCALIBB_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }
    impl Protected for crate::attiny1616::clkctrl::osc32kctrla::OSC32KCTRLA_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }

    // Configuration change protected registers in RSTCTRL
    impl Protected for crate::attiny1616::rstctrl::swrr::SWRR_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }

    // Configuration change protected registers in CPUINT
    impl Protected for crate::attiny1616::cpuint::ctrla::CTRLA_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }

    // Configuration change protected registers in BOD
    impl Protected for crate::attiny1616::bod::ctrla::CTRLA_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }

    // Configuration change protected registers in WDT
    impl Protected for crate::attiny1616::wdt::ctrla::CTRLA_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }
    impl Protected for crate::attiny1616::wdt::status::STATUS_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }

    // Configuration change protected registers in TCD0
    impl Protected for crate::attiny1616::tcd0::faultctrl::FAULTCTRL_SPEC { const MAGIC: u8 = 0xD8; type CcpReg = crate::attiny1616::cpu::ccp::CCP_SPEC; }
}
