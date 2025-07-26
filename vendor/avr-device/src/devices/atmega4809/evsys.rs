#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    strobe: STROBE,
    _reserved1: [u8; 0x0f],
    channel0: CHANNEL0,
    channel1: CHANNEL1,
    channel2: CHANNEL2,
    channel3: CHANNEL3,
    channel4: CHANNEL4,
    channel5: CHANNEL5,
    channel6: CHANNEL6,
    channel7: CHANNEL7,
    _reserved9: [u8; 0x08],
    userccllut0a: USERCCLLUT0A,
    userccllut0b: USERCCLLUT0B,
    userccllut1a: USERCCLLUT1A,
    userccllut1b: USERCCLLUT1B,
    userccllut2a: USERCCLLUT2A,
    userccllut2b: USERCCLLUT2B,
    userccllut3a: USERCCLLUT3A,
    userccllut3b: USERCCLLUT3B,
    useradc0: USERADC0,
    userevouta: USEREVOUTA,
    userevoutb: USEREVOUTB,
    userevoutc: USEREVOUTC,
    userevoutd: USEREVOUTD,
    userevoute: USEREVOUTE,
    userevoutf: USEREVOUTF,
    userusart0: USERUSART0,
    userusart1: USERUSART1,
    userusart2: USERUSART2,
    userusart3: USERUSART3,
    usertca0: USERTCA0,
    usertcb0: USERTCB0,
    usertcb1: USERTCB1,
    usertcb2: USERTCB2,
    usertcb3: USERTCB3,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel Strobe"]
    #[inline(always)]
    pub const fn strobe(&self) -> &STROBE {
        &self.strobe
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
    #[doc = "0x16 - Multiplexer Channel 6"]
    #[inline(always)]
    pub const fn channel6(&self) -> &CHANNEL6 {
        &self.channel6
    }
    #[doc = "0x17 - Multiplexer Channel 7"]
    #[inline(always)]
    pub const fn channel7(&self) -> &CHANNEL7 {
        &self.channel7
    }
    #[doc = "0x20 - User CCL LUT0 Event A"]
    #[inline(always)]
    pub const fn userccllut0a(&self) -> &USERCCLLUT0A {
        &self.userccllut0a
    }
    #[doc = "0x21 - User CCL LUT0 Event B"]
    #[inline(always)]
    pub const fn userccllut0b(&self) -> &USERCCLLUT0B {
        &self.userccllut0b
    }
    #[doc = "0x22 - User CCL LUT1 Event A"]
    #[inline(always)]
    pub const fn userccllut1a(&self) -> &USERCCLLUT1A {
        &self.userccllut1a
    }
    #[doc = "0x23 - User CCL LUT1 Event B"]
    #[inline(always)]
    pub const fn userccllut1b(&self) -> &USERCCLLUT1B {
        &self.userccllut1b
    }
    #[doc = "0x24 - User CCL LUT2 Event A"]
    #[inline(always)]
    pub const fn userccllut2a(&self) -> &USERCCLLUT2A {
        &self.userccllut2a
    }
    #[doc = "0x25 - User CCL LUT2 Event B"]
    #[inline(always)]
    pub const fn userccllut2b(&self) -> &USERCCLLUT2B {
        &self.userccllut2b
    }
    #[doc = "0x26 - User CCL LUT3 Event A"]
    #[inline(always)]
    pub const fn userccllut3a(&self) -> &USERCCLLUT3A {
        &self.userccllut3a
    }
    #[doc = "0x27 - User CCL LUT3 Event B"]
    #[inline(always)]
    pub const fn userccllut3b(&self) -> &USERCCLLUT3B {
        &self.userccllut3b
    }
    #[doc = "0x28 - User ADC0"]
    #[inline(always)]
    pub const fn useradc0(&self) -> &USERADC0 {
        &self.useradc0
    }
    #[doc = "0x29 - User EVOUT Port A"]
    #[inline(always)]
    pub const fn userevouta(&self) -> &USEREVOUTA {
        &self.userevouta
    }
    #[doc = "0x2a - User EVOUT Port B"]
    #[inline(always)]
    pub const fn userevoutb(&self) -> &USEREVOUTB {
        &self.userevoutb
    }
    #[doc = "0x2b - User EVOUT Port C"]
    #[inline(always)]
    pub const fn userevoutc(&self) -> &USEREVOUTC {
        &self.userevoutc
    }
    #[doc = "0x2c - User EVOUT Port D"]
    #[inline(always)]
    pub const fn userevoutd(&self) -> &USEREVOUTD {
        &self.userevoutd
    }
    #[doc = "0x2d - User EVOUT Port E"]
    #[inline(always)]
    pub const fn userevoute(&self) -> &USEREVOUTE {
        &self.userevoute
    }
    #[doc = "0x2e - User EVOUT Port F"]
    #[inline(always)]
    pub const fn userevoutf(&self) -> &USEREVOUTF {
        &self.userevoutf
    }
    #[doc = "0x2f - User USART0"]
    #[inline(always)]
    pub const fn userusart0(&self) -> &USERUSART0 {
        &self.userusart0
    }
    #[doc = "0x30 - User USART1"]
    #[inline(always)]
    pub const fn userusart1(&self) -> &USERUSART1 {
        &self.userusart1
    }
    #[doc = "0x31 - User USART2"]
    #[inline(always)]
    pub const fn userusart2(&self) -> &USERUSART2 {
        &self.userusart2
    }
    #[doc = "0x32 - User USART3"]
    #[inline(always)]
    pub const fn userusart3(&self) -> &USERUSART3 {
        &self.userusart3
    }
    #[doc = "0x33 - User TCA0"]
    #[inline(always)]
    pub const fn usertca0(&self) -> &USERTCA0 {
        &self.usertca0
    }
    #[doc = "0x34 - User TCB0"]
    #[inline(always)]
    pub const fn usertcb0(&self) -> &USERTCB0 {
        &self.usertcb0
    }
    #[doc = "0x35 - User TCB1"]
    #[inline(always)]
    pub const fn usertcb1(&self) -> &USERTCB1 {
        &self.usertcb1
    }
    #[doc = "0x36 - User TCB2"]
    #[inline(always)]
    pub const fn usertcb2(&self) -> &USERTCB2 {
        &self.usertcb2
    }
    #[doc = "0x37 - User TCB3"]
    #[inline(always)]
    pub const fn usertcb3(&self) -> &USERTCB3 {
        &self.usertcb3
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
#[doc = "CHANNEL6 (rw) register accessor: Multiplexer Channel 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel6`]
module"]
pub type CHANNEL6 = crate::Reg<channel6::CHANNEL6_SPEC>;
#[doc = "Multiplexer Channel 6"]
pub mod channel6;
#[doc = "CHANNEL7 (rw) register accessor: Multiplexer Channel 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel7`]
module"]
pub type CHANNEL7 = crate::Reg<channel7::CHANNEL7_SPEC>;
#[doc = "Multiplexer Channel 7"]
pub mod channel7;
#[doc = "STROBE (w) register accessor: Channel Strobe\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`strobe::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strobe`]
module"]
pub type STROBE = crate::Reg<strobe::STROBE_SPEC>;
#[doc = "Channel Strobe"]
pub mod strobe;
#[doc = "USERADC0 (rw) register accessor: User ADC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`useradc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`useradc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@useradc0`]
module"]
pub type USERADC0 = crate::Reg<useradc0::USERADC0_SPEC>;
#[doc = "User ADC0"]
pub mod useradc0;
#[doc = "USERCCLLUT0A (rw) register accessor: User CCL LUT0 Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut0a`]
module"]
pub type USERCCLLUT0A = crate::Reg<userccllut0a::USERCCLLUT0A_SPEC>;
#[doc = "User CCL LUT0 Event A"]
pub mod userccllut0a;
#[doc = "USERCCLLUT0B (rw) register accessor: User CCL LUT0 Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut0b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut0b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut0b`]
module"]
pub type USERCCLLUT0B = crate::Reg<userccllut0b::USERCCLLUT0B_SPEC>;
#[doc = "User CCL LUT0 Event B"]
pub mod userccllut0b;
#[doc = "USERCCLLUT1A (rw) register accessor: User CCL LUT1 Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut1a`]
module"]
pub type USERCCLLUT1A = crate::Reg<userccllut1a::USERCCLLUT1A_SPEC>;
#[doc = "User CCL LUT1 Event A"]
pub mod userccllut1a;
#[doc = "USERCCLLUT1B (rw) register accessor: User CCL LUT1 Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut1b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut1b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut1b`]
module"]
pub type USERCCLLUT1B = crate::Reg<userccllut1b::USERCCLLUT1B_SPEC>;
#[doc = "User CCL LUT1 Event B"]
pub mod userccllut1b;
#[doc = "USERCCLLUT2A (rw) register accessor: User CCL LUT2 Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut2a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut2a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut2a`]
module"]
pub type USERCCLLUT2A = crate::Reg<userccllut2a::USERCCLLUT2A_SPEC>;
#[doc = "User CCL LUT2 Event A"]
pub mod userccllut2a;
#[doc = "USERCCLLUT2B (rw) register accessor: User CCL LUT2 Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut2b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut2b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut2b`]
module"]
pub type USERCCLLUT2B = crate::Reg<userccllut2b::USERCCLLUT2B_SPEC>;
#[doc = "User CCL LUT2 Event B"]
pub mod userccllut2b;
#[doc = "USERCCLLUT3A (rw) register accessor: User CCL LUT3 Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut3a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut3a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut3a`]
module"]
pub type USERCCLLUT3A = crate::Reg<userccllut3a::USERCCLLUT3A_SPEC>;
#[doc = "User CCL LUT3 Event A"]
pub mod userccllut3a;
#[doc = "USERCCLLUT3B (rw) register accessor: User CCL LUT3 Event B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userccllut3b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userccllut3b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userccllut3b`]
module"]
pub type USERCCLLUT3B = crate::Reg<userccllut3b::USERCCLLUT3B_SPEC>;
#[doc = "User CCL LUT3 Event B"]
pub mod userccllut3b;
#[doc = "USEREVOUTA (rw) register accessor: User EVOUT Port A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userevouta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userevouta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userevouta`]
module"]
pub type USEREVOUTA = crate::Reg<userevouta::USEREVOUTA_SPEC>;
#[doc = "User EVOUT Port A"]
pub mod userevouta;
#[doc = "USEREVOUTB (rw) register accessor: User EVOUT Port B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userevoutb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userevoutb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userevoutb`]
module"]
pub type USEREVOUTB = crate::Reg<userevoutb::USEREVOUTB_SPEC>;
#[doc = "User EVOUT Port B"]
pub mod userevoutb;
#[doc = "USEREVOUTC (rw) register accessor: User EVOUT Port C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userevoutc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userevoutc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userevoutc`]
module"]
pub type USEREVOUTC = crate::Reg<userevoutc::USEREVOUTC_SPEC>;
#[doc = "User EVOUT Port C"]
pub mod userevoutc;
#[doc = "USEREVOUTD (rw) register accessor: User EVOUT Port D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userevoutd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userevoutd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userevoutd`]
module"]
pub type USEREVOUTD = crate::Reg<userevoutd::USEREVOUTD_SPEC>;
#[doc = "User EVOUT Port D"]
pub mod userevoutd;
#[doc = "USEREVOUTE (rw) register accessor: User EVOUT Port E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userevoute::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userevoute::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userevoute`]
module"]
pub type USEREVOUTE = crate::Reg<userevoute::USEREVOUTE_SPEC>;
#[doc = "User EVOUT Port E"]
pub mod userevoute;
#[doc = "USEREVOUTF (rw) register accessor: User EVOUT Port F\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userevoutf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userevoutf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userevoutf`]
module"]
pub type USEREVOUTF = crate::Reg<userevoutf::USEREVOUTF_SPEC>;
#[doc = "User EVOUT Port F"]
pub mod userevoutf;
#[doc = "USERTCA0 (rw) register accessor: User TCA0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertca0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertca0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertca0`]
module"]
pub type USERTCA0 = crate::Reg<usertca0::USERTCA0_SPEC>;
#[doc = "User TCA0"]
pub mod usertca0;
#[doc = "USERTCB0 (rw) register accessor: User TCB0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertcb0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertcb0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertcb0`]
module"]
pub type USERTCB0 = crate::Reg<usertcb0::USERTCB0_SPEC>;
#[doc = "User TCB0"]
pub mod usertcb0;
#[doc = "USERTCB1 (rw) register accessor: User TCB1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertcb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertcb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertcb1`]
module"]
pub type USERTCB1 = crate::Reg<usertcb1::USERTCB1_SPEC>;
#[doc = "User TCB1"]
pub mod usertcb1;
#[doc = "USERTCB2 (rw) register accessor: User TCB2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertcb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertcb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertcb2`]
module"]
pub type USERTCB2 = crate::Reg<usertcb2::USERTCB2_SPEC>;
#[doc = "User TCB2"]
pub mod usertcb2;
#[doc = "USERTCB3 (rw) register accessor: User TCB3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertcb3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertcb3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usertcb3`]
module"]
pub type USERTCB3 = crate::Reg<usertcb3::USERTCB3_SPEC>;
#[doc = "User TCB3"]
pub mod usertcb3;
#[doc = "USERUSART0 (rw) register accessor: User USART0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userusart0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userusart0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userusart0`]
module"]
pub type USERUSART0 = crate::Reg<userusart0::USERUSART0_SPEC>;
#[doc = "User USART0"]
pub mod userusart0;
#[doc = "USERUSART1 (rw) register accessor: User USART1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userusart1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userusart1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userusart1`]
module"]
pub type USERUSART1 = crate::Reg<userusart1::USERUSART1_SPEC>;
#[doc = "User USART1"]
pub mod userusart1;
#[doc = "USERUSART2 (rw) register accessor: User USART2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userusart2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userusart2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userusart2`]
module"]
pub type USERUSART2 = crate::Reg<userusart2::USERUSART2_SPEC>;
#[doc = "User USART2"]
pub mod userusart2;
#[doc = "USERUSART3 (rw) register accessor: User USART3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userusart3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userusart3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userusart3`]
module"]
pub type USERUSART3 = crate::Reg<userusart3::USERUSART3_SPEC>;
#[doc = "User USART3"]
pub mod userusart3;
