#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    sweventa: SWEVENTA,
    sweventb: SWEVENTB,
    _reserved2: [u8; 0x0e],
    channel0: CHANNEL0,
    channel1: CHANNEL1,
    channel2: CHANNEL2,
    channel3: CHANNEL3,
    channel4: CHANNEL4,
    channel5: CHANNEL5,
    _reserved8: [u8; 0x0a],
    userccllut0a: USERCCLLUT0A,
    userccllut0b: USERCCLLUT0B,
    userccllut1a: USERCCLLUT1A,
    userccllut1b: USERCCLLUT1B,
    userccllut2a: USERCCLLUT2A,
    userccllut2b: USERCCLLUT2B,
    userccllut3a: USERCCLLUT3A,
    userccllut3b: USERCCLLUT3B,
    useradc0start: USERADC0START,
    userevsysevouta: USEREVSYSEVOUTA,
    userevsysevoutc: USEREVSYSEVOUTC,
    userevsysevoutd: USEREVSYSEVOUTD,
    userevsysevoutf: USEREVSYSEVOUTF,
    userusart0irda: USERUSART0IRDA,
    userusart1irda: USERUSART1IRDA,
    usertca0cnta: USERTCA0CNTA,
    usertca0cntb: USERTCA0CNTB,
    usertcb0capt: USERTCB0CAPT,
    usertcb0count: USERTCB0COUNT,
    usertcb1capt: USERTCB1CAPT,
    usertcb1count: USERTCB1COUNT,
    usertcb2capt: USERTCB2CAPT,
    usertcb2count: USERTCB2COUNT,
    usertcd0inputa: USERTCD0INPUTA,
    usertcd0inputb: USERTCD0INPUTB,
}
impl RegisterBlock {
    #[doc = "0x00 - Software Event A"]
    #[inline(always)]
    pub const fn sweventa(&self) -> &SWEVENTA {
        &self.sweventa
    }
    #[doc = "0x01 - Software Event B"]
    #[inline(always)]
    pub const fn sweventb(&self) -> &SWEVENTB {
        &self.sweventb
    }
    #[doc = "0x10 - Multiplexer Channel 0"]
    #[inline(always)]
    pub const fn channel0(&self) -> &CHANNEL0 {
        &self.channel0
    }
    #[doc = "0x11 - Multiplexer Channel 1"]
    #[inline(always)]
    pub const fn channel1(&self) -> &CHANNEL1 {
        &self.channel1
    }
    #[doc = "0x12 - Multiplexer Channel 2"]
    #[inline(always)]
    pub const fn channel2(&self) -> &CHANNEL2 {
        &self.channel2
    }
    #[doc = "0x13 - Multiplexer Channel 3"]
    #[inline(always)]
    pub const fn channel3(&self) -> &CHANNEL3 {
        &self.channel3
    }
    #[doc = "0x14 - Multiplexer Channel 4"]
    #[inline(always)]
    pub const fn channel4(&self) -> &CHANNEL4 {
        &self.channel4
    }
    #[doc = "0x15 - Multiplexer Channel 5"]
    #[inline(always)]
    pub const fn channel5(&self) -> &CHANNEL5 {
        &self.channel5
    }
    #[doc = "0x20 - User 0 - CCL0 Event A"]
    #[inline(always)]
    pub const fn userccllut0a(&self) -> &USERCCLLUT0A {
        &self.userccllut0a
    }
    #[doc = "0x21 - User 1 - CCL0 Event B"]
    #[inline(always)]
    pub const fn userccllut0b(&self) -> &USERCCLLUT0B {
        &self.userccllut0b
    }
    #[doc = "0x22 - User 2 - CCL1 Event A"]
    #[inline(always)]
    pub const fn userccllut1a(&self) -> &USERCCLLUT1A {
        &self.userccllut1a
    }
    #[doc = "0x23 - User 3 - CCL1 Event B"]
    #[inline(always)]
    pub const fn userccllut1b(&self) -> &USERCCLLUT1B {
        &self.userccllut1b
    }
    #[doc = "0x24 - User 4 - CCL2 Event A"]
    #[inline(always)]
    pub const fn userccllut2a(&self) -> &USERCCLLUT2A {
        &self.userccllut2a
    }
    #[doc = "0x25 - User 5 - CCL2 Event B"]
    #[inline(always)]
    pub const fn userccllut2b(&self) -> &USERCCLLUT2B {
        &self.userccllut2b
    }
    #[doc = "0x26 - User 6 - CCL3 Event A"]
    #[inline(always)]
    pub const fn userccllut3a(&self) -> &USERCCLLUT3A {
        &self.userccllut3a
    }
    #[doc = "0x27 - User 7 - CCL3 Event B"]
    #[inline(always)]
    pub const fn userccllut3b(&self) -> &USERCCLLUT3B {
        &self.userccllut3b
    }
    #[doc = "0x28 - User 12 - ADC0"]
    #[inline(always)]
    pub const fn useradc0start(&self) -> &USERADC0START {
        &self.useradc0start
    }
    #[doc = "0x29 - User 13 - EVOUTA"]
    #[inline(always)]
    pub const fn userevsysevouta(&self) -> &USEREVSYSEVOUTA {
        &self.userevsysevouta
    }
    #[doc = "0x2a - User 15 - EVOUTC"]
    #[inline(always)]
    pub const fn userevsysevoutc(&self) -> &USEREVSYSEVOUTC {
        &self.userevsysevoutc
    }
    #[doc = "0x2b - User 16 - EVOUTD"]
    #[inline(always)]
    pub const fn userevsysevoutd(&self) -> &USEREVSYSEVOUTD {
        &self.userevsysevoutd
    }
    #[doc = "0x2c - User 18 - EVOUTF"]
    #[inline(always)]
    pub const fn userevsysevoutf(&self) -> &USEREVSYSEVOUTF {
        &self.userevsysevoutf
    }
    #[doc = "0x2d - User 20 - USART0"]
    #[inline(always)]
    pub const fn userusart0irda(&self) -> &USERUSART0IRDA {
        &self.userusart0irda
    }
    #[doc = "0x2e - User 21 - USART1"]
    #[inline(always)]
    pub const fn userusart1irda(&self) -> &USERUSART1IRDA {
        &self.userusart1irda
    }
    #[doc = "0x2f - User 26 - TCA0 Event A"]
    #[inline(always)]
    pub const fn usertca0cnta(&self) -> &USERTCA0CNTA {
        &self.usertca0cnta
    }
    #[doc = "0x30 - User 27 - TCA0 Event B"]
    #[inline(always)]
    pub const fn usertca0cntb(&self) -> &USERTCA0CNTB {
        &self.usertca0cntb
    }
    #[doc = "0x31 - User 30 - TCB0 Event A"]
    #[inline(always)]
    pub const fn usertcb0capt(&self) -> &USERTCB0CAPT {
        &self.usertcb0capt
    }
    #[doc = "0x32 - User 31 - TCB0 Event B"]
    #[inline(always)]
    pub const fn usertcb0count(&self) -> &USERTCB0COUNT {
        &self.usertcb0count
    }
    #[doc = "0x33 - User 32 - TCB1 Event A"]
    #[inline(always)]
    pub const fn usertcb1capt(&self) -> &USERTCB1CAPT {
        &self.usertcb1capt
    }
    #[doc = "0x34 - User 33 - TCB1 Event B"]
    #[inline(always)]
    pub const fn usertcb1count(&self) -> &USERTCB1COUNT {
        &self.usertcb1count
    }
    #[doc = "0x35 - User 34 - TCB2 Event A"]
    #[inline(always)]
    pub const fn usertcb2capt(&self) -> &USERTCB2CAPT {
        &self.usertcb2capt
    }
    #[doc = "0x36 - User 35 - TCB2 Event B"]
    #[inline(always)]
    pub const fn usertcb2count(&self) -> &USERTCB2COUNT {
        &self.usertcb2count
    }
    #[doc = "0x37 - User 40 - TCD0 Event A"]
    #[inline(always)]
    pub const fn usertcd0inputa(&self) -> &USERTCD0INPUTA {
        &self.usertcd0inputa
    }
    #[doc = "0x38 - User 41 - TCD0 Event B"]
    #[inline(always)]
    pub const fn usertcd0inputb(&self) -> &USERTCD0INPUTB {
        &self.usertcd0inputb
    }
}
#[doc = "CHANNEL0 (rw) register accessor: Multiplexer Channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel0`]
module"]
pub type CHANNEL0 = crate::Reg<channel0::CHANNEL0_SPEC>;
#[doc = "Multiplexer Channel 0"]
pub mod channel0;
#[doc = "CHANNEL1 (rw) register accessor: Multiplexer Channel 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel1`]
module"]
pub type CHANNEL1 = crate::Reg<channel1::CHANNEL1_SPEC>;
#[doc = "Multiplexer Channel 1"]
pub mod channel1;
#[doc = "CHANNEL2 (rw) register accessor: Multiplexer Channel 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel2`]
module"]
pub type CHANNEL2 = crate::Reg<channel2::CHANNEL2_SPEC>;
#[doc = "Multiplexer Channel 2"]
pub mod channel2;
#[doc = "CHANNEL3 (rw) register accessor: Multiplexer Channel 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel3`]
module"]
pub type CHANNEL3 = crate::Reg<channel3::CHANNEL3_SPEC>;
#[doc = "Multiplexer Channel 3"]
pub mod channel3;
#[doc = "CHANNEL4 (rw) register accessor: Multiplexer Channel 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel4`]
module"]
pub type CHANNEL4 = crate::Reg<channel4::CHANNEL4_SPEC>;
#[doc = "Multiplexer Channel 4"]
pub mod channel4;
#[doc = "CHANNEL5 (rw) register accessor: Multiplexer Channel 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel5`]
module"]
pub type CHANNEL5 = crate::Reg<channel5::CHANNEL5_SPEC>;
#[doc = "Multiplexer Channel 5"]
pub mod channel5;
#[doc = "SWEVENTA (rw) register accessor: Software Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sweventa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sweventa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sweventa`]
module"]
pub type SWEVENTA = crate::Reg<sweventa::SWEVENTA_SPEC>;
#[doc = "Software Event A"]
pub mod sweventa;
#[doc = "SWEVENTB (rw) register accessor: Software Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sweventb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sweventb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sweventb`]
module"]
pub type SWEVENTB = crate::Reg<sweventb::SWEVENTB_SPEC>;
#[doc = "Software Event B"]
pub mod sweventb;
#[doc = "USERADC0START (rw) register accessor: User 12 - ADC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`useradc0start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`useradc0start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@useradc0start`]
module"]
pub type USERADC0START = crate::Reg<useradc0start::USERADC0START_SPEC>;
#[doc = "User 12 - ADC0"]
pub mod useradc0start;
#[doc = "USERCCLLUT0A (rw) register accessor: User 0 - CCL0 Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut0a`]
module"]
pub type USERCCLLUT0A = crate::Reg<userccllut0a::USERCCLLUT0A_SPEC>;
#[doc = "User 0 - CCL0 Event A"]
pub mod userccllut0a;
#[doc = "USERCCLLUT0B (rw) register accessor: User 1 - CCL0 Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut0b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut0b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut0b`]
module"]
pub type USERCCLLUT0B = crate::Reg<userccllut0b::USERCCLLUT0B_SPEC>;
#[doc = "User 1 - CCL0 Event B"]
pub mod userccllut0b;
#[doc = "USERCCLLUT1A (rw) register accessor: User 2 - CCL1 Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut1a`]
module"]
pub type USERCCLLUT1A = crate::Reg<userccllut1a::USERCCLLUT1A_SPEC>;
#[doc = "User 2 - CCL1 Event A"]
pub mod userccllut1a;
#[doc = "USERCCLLUT1B (rw) register accessor: User 3 - CCL1 Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut1b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut1b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut1b`]
module"]
pub type USERCCLLUT1B = crate::Reg<userccllut1b::USERCCLLUT1B_SPEC>;
#[doc = "User 3 - CCL1 Event B"]
pub mod userccllut1b;
#[doc = "USERCCLLUT2A (rw) register accessor: User 4 - CCL2 Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut2a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut2a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut2a`]
module"]
pub type USERCCLLUT2A = crate::Reg<userccllut2a::USERCCLLUT2A_SPEC>;
#[doc = "User 4 - CCL2 Event A"]
pub mod userccllut2a;
#[doc = "USERCCLLUT2B (rw) register accessor: User 5 - CCL2 Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut2b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut2b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut2b`]
module"]
pub type USERCCLLUT2B = crate::Reg<userccllut2b::USERCCLLUT2B_SPEC>;
#[doc = "User 5 - CCL2 Event B"]
pub mod userccllut2b;
#[doc = "USERCCLLUT3A (rw) register accessor: User 6 - CCL3 Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut3a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut3a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut3a`]
module"]
pub type USERCCLLUT3A = crate::Reg<userccllut3a::USERCCLLUT3A_SPEC>;
#[doc = "User 6 - CCL3 Event A"]
pub mod userccllut3a;
#[doc = "USERCCLLUT3B (rw) register accessor: User 7 - CCL3 Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut3b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut3b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut3b`]
module"]
pub type USERCCLLUT3B = crate::Reg<userccllut3b::USERCCLLUT3B_SPEC>;
#[doc = "User 7 - CCL3 Event B"]
pub mod userccllut3b;
#[doc = "USEREVSYSEVOUTA (rw) register accessor: User 13 - EVOUTA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userevsysevouta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userevsysevouta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userevsysevouta`]
module"]
pub type USEREVSYSEVOUTA = crate::Reg<userevsysevouta::USEREVSYSEVOUTA_SPEC>;
#[doc = "User 13 - EVOUTA"]
pub mod userevsysevouta;
#[doc = "USEREVSYSEVOUTC (rw) register accessor: User 15 - EVOUTC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userevsysevoutc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userevsysevoutc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userevsysevoutc`]
module"]
pub type USEREVSYSEVOUTC = crate::Reg<userevsysevoutc::USEREVSYSEVOUTC_SPEC>;
#[doc = "User 15 - EVOUTC"]
pub mod userevsysevoutc;
#[doc = "USEREVSYSEVOUTD (rw) register accessor: User 16 - EVOUTD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userevsysevoutd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userevsysevoutd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userevsysevoutd`]
module"]
pub type USEREVSYSEVOUTD = crate::Reg<userevsysevoutd::USEREVSYSEVOUTD_SPEC>;
#[doc = "User 16 - EVOUTD"]
pub mod userevsysevoutd;
#[doc = "USEREVSYSEVOUTF (rw) register accessor: User 18 - EVOUTF\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userevsysevoutf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userevsysevoutf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userevsysevoutf`]
module"]
pub type USEREVSYSEVOUTF = crate::Reg<userevsysevoutf::USEREVSYSEVOUTF_SPEC>;
#[doc = "User 18 - EVOUTF"]
pub mod userevsysevoutf;
#[doc = "USERTCA0CNTA (rw) register accessor: User 26 - TCA0 Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertca0cnta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertca0cnta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertca0cnta`]
module"]
pub type USERTCA0CNTA = crate::Reg<usertca0cnta::USERTCA0CNTA_SPEC>;
#[doc = "User 26 - TCA0 Event A"]
pub mod usertca0cnta;
#[doc = "USERTCA0CNTB (rw) register accessor: User 27 - TCA0 Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertca0cntb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertca0cntb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertca0cntb`]
module"]
pub type USERTCA0CNTB = crate::Reg<usertca0cntb::USERTCA0CNTB_SPEC>;
#[doc = "User 27 - TCA0 Event B"]
pub mod usertca0cntb;
#[doc = "USERTCB0CAPT (rw) register accessor: User 30 - TCB0 Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertcb0capt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertcb0capt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertcb0capt`]
module"]
pub type USERTCB0CAPT = crate::Reg<usertcb0capt::USERTCB0CAPT_SPEC>;
#[doc = "User 30 - TCB0 Event A"]
pub mod usertcb0capt;
#[doc = "USERTCB0COUNT (rw) register accessor: User 31 - TCB0 Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertcb0count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertcb0count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertcb0count`]
module"]
pub type USERTCB0COUNT = crate::Reg<usertcb0count::USERTCB0COUNT_SPEC>;
#[doc = "User 31 - TCB0 Event B"]
pub mod usertcb0count;
#[doc = "USERTCB1CAPT (rw) register accessor: User 32 - TCB1 Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertcb1capt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertcb1capt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertcb1capt`]
module"]
pub type USERTCB1CAPT = crate::Reg<usertcb1capt::USERTCB1CAPT_SPEC>;
#[doc = "User 32 - TCB1 Event A"]
pub mod usertcb1capt;
#[doc = "USERTCB1COUNT (rw) register accessor: User 33 - TCB1 Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertcb1count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertcb1count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertcb1count`]
module"]
pub type USERTCB1COUNT = crate::Reg<usertcb1count::USERTCB1COUNT_SPEC>;
#[doc = "User 33 - TCB1 Event B"]
pub mod usertcb1count;
#[doc = "USERTCB2CAPT (rw) register accessor: User 34 - TCB2 Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertcb2capt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertcb2capt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertcb2capt`]
module"]
pub type USERTCB2CAPT = crate::Reg<usertcb2capt::USERTCB2CAPT_SPEC>;
#[doc = "User 34 - TCB2 Event A"]
pub mod usertcb2capt;
#[doc = "USERTCB2COUNT (rw) register accessor: User 35 - TCB2 Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertcb2count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertcb2count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertcb2count`]
module"]
pub type USERTCB2COUNT = crate::Reg<usertcb2count::USERTCB2COUNT_SPEC>;
#[doc = "User 35 - TCB2 Event B"]
pub mod usertcb2count;
#[doc = "USERTCD0INPUTA (rw) register accessor: User 40 - TCD0 Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertcd0inputa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertcd0inputa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertcd0inputa`]
module"]
pub type USERTCD0INPUTA = crate::Reg<usertcd0inputa::USERTCD0INPUTA_SPEC>;
#[doc = "User 40 - TCD0 Event A"]
pub mod usertcd0inputa;
#[doc = "USERTCD0INPUTB (rw) register accessor: User 41 - TCD0 Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertcd0inputb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertcd0inputb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertcd0inputb`]
module"]
pub type USERTCD0INPUTB = crate::Reg<usertcd0inputb::USERTCD0INPUTB_SPEC>;
#[doc = "User 41 - TCD0 Event B"]
pub mod usertcd0inputb;
#[doc = "USERUSART0IRDA (rw) register accessor: User 20 - USART0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userusart0irda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userusart0irda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userusart0irda`]
module"]
pub type USERUSART0IRDA = crate::Reg<userusart0irda::USERUSART0IRDA_SPEC>;
#[doc = "User 20 - USART0"]
pub mod userusart0irda;
#[doc = "USERUSART1IRDA (rw) register accessor: User 21 - USART1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userusart1irda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userusart1irda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userusart1irda`]
module"]
pub type USERUSART1IRDA = crate::Reg<userusart1irda::USERUSART1IRDA_SPEC>;
#[doc = "User 21 - USART1"]
pub mod userusart1irda;
