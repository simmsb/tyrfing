#[doc = "Register `USICR` reader"]
pub type R = crate::R<USICR_SPEC>;
#[doc = "Register `USICR` writer"]
pub type W = crate::W<USICR_SPEC>;
#[doc = "Field `USITC` writer - Toggle Clock Port Pin"]
pub type USITC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USICLK` writer - Clock Strobe"]
pub type USICLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USICS` reader - USI Clock Source Select Bits"]
pub type USICS_R = crate::FieldReader<USICS_A>;
#[doc = "USI Clock Source Select Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USICS_A {
    #[doc = "0: No Clock/Software clock strobe"]
    NO_CLOCK = 0,
    #[doc = "1: Timer/Counter0 Compare Match"]
    TC0 = 1,
    #[doc = "2: External, positive edge"]
    EXT_POS = 2,
    #[doc = "3: External, negative edge"]
    EXT_NEG = 3,
}
impl From<USICS_A> for u8 {
    #[inline(always)]
    fn from(variant: USICS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USICS_A {
    type Ux = u8;
}
impl USICS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USICS_A {
        match self.bits {
            0 => USICS_A::NO_CLOCK,
            1 => USICS_A::TC0,
            2 => USICS_A::EXT_POS,
            3 => USICS_A::EXT_NEG,
            _ => unreachable!(),
        }
    }
    #[doc = "No Clock/Software clock strobe"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == USICS_A::NO_CLOCK
    }
    #[doc = "Timer/Counter0 Compare Match"]
    #[inline(always)]
    pub fn is_tc0(&self) -> bool {
        *self == USICS_A::TC0
    }
    #[doc = "External, positive edge"]
    #[inline(always)]
    pub fn is_ext_pos(&self) -> bool {
        *self == USICS_A::EXT_POS
    }
    #[doc = "External, negative edge"]
    #[inline(always)]
    pub fn is_ext_neg(&self) -> bool {
        *self == USICS_A::EXT_NEG
    }
}
#[doc = "Field `USICS` writer - USI Clock Source Select Bits"]
pub type USICS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, USICS_A>;
impl<'a, REG> USICS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Clock/Software clock strobe"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(USICS_A::NO_CLOCK)
    }
    #[doc = "Timer/Counter0 Compare Match"]
    #[inline(always)]
    pub fn tc0(self) -> &'a mut crate::W<REG> {
        self.variant(USICS_A::TC0)
    }
    #[doc = "External, positive edge"]
    #[inline(always)]
    pub fn ext_pos(self) -> &'a mut crate::W<REG> {
        self.variant(USICS_A::EXT_POS)
    }
    #[doc = "External, negative edge"]
    #[inline(always)]
    pub fn ext_neg(self) -> &'a mut crate::W<REG> {
        self.variant(USICS_A::EXT_NEG)
    }
}
#[doc = "Field `USIWM` reader - USI Wire Mode Bits"]
pub type USIWM_R = crate::FieldReader<USIWM_A>;
#[doc = "USI Wire Mode Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USIWM_A {
    #[doc = "0: All detectors disabled. Port pins operates as normal."]
    DISABLED = 0,
    #[doc = "1: Three-wire mode. Uses DO, DI, and USCK pins."]
    THREE_WIRE = 1,
    #[doc = "2: Two-wire mode (Slave). Uses SDA (DI) and SCL (USCK) pins."]
    TWO_WIRE_SLAVE = 2,
    #[doc = "3: Two-wire mode (Master). Uses SDA and SCL pins."]
    TWO_WIRE_MASTER = 3,
}
impl From<USIWM_A> for u8 {
    #[inline(always)]
    fn from(variant: USIWM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USIWM_A {
    type Ux = u8;
}
impl USIWM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USIWM_A {
        match self.bits {
            0 => USIWM_A::DISABLED,
            1 => USIWM_A::THREE_WIRE,
            2 => USIWM_A::TWO_WIRE_SLAVE,
            3 => USIWM_A::TWO_WIRE_MASTER,
            _ => unreachable!(),
        }
    }
    #[doc = "All detectors disabled. Port pins operates as normal."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USIWM_A::DISABLED
    }
    #[doc = "Three-wire mode. Uses DO, DI, and USCK pins."]
    #[inline(always)]
    pub fn is_three_wire(&self) -> bool {
        *self == USIWM_A::THREE_WIRE
    }
    #[doc = "Two-wire mode (Slave). Uses SDA (DI) and SCL (USCK) pins."]
    #[inline(always)]
    pub fn is_two_wire_slave(&self) -> bool {
        *self == USIWM_A::TWO_WIRE_SLAVE
    }
    #[doc = "Two-wire mode (Master). Uses SDA and SCL pins."]
    #[inline(always)]
    pub fn is_two_wire_master(&self) -> bool {
        *self == USIWM_A::TWO_WIRE_MASTER
    }
}
#[doc = "Field `USIWM` writer - USI Wire Mode Bits"]
pub type USIWM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, USIWM_A>;
impl<'a, REG> USIWM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All detectors disabled. Port pins operates as normal."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(USIWM_A::DISABLED)
    }
    #[doc = "Three-wire mode. Uses DO, DI, and USCK pins."]
    #[inline(always)]
    pub fn three_wire(self) -> &'a mut crate::W<REG> {
        self.variant(USIWM_A::THREE_WIRE)
    }
    #[doc = "Two-wire mode (Slave). Uses SDA (DI) and SCL (USCK) pins."]
    #[inline(always)]
    pub fn two_wire_slave(self) -> &'a mut crate::W<REG> {
        self.variant(USIWM_A::TWO_WIRE_SLAVE)
    }
    #[doc = "Two-wire mode (Master). Uses SDA and SCL pins."]
    #[inline(always)]
    pub fn two_wire_master(self) -> &'a mut crate::W<REG> {
        self.variant(USIWM_A::TWO_WIRE_MASTER)
    }
}
#[doc = "Field `USIOIE` reader - Counter Overflow Interrupt Enable"]
pub type USIOIE_R = crate::BitReader;
#[doc = "Field `USIOIE` writer - Counter Overflow Interrupt Enable"]
pub type USIOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USISIE` reader - Start Condition Interrupt Enable"]
pub type USISIE_R = crate::BitReader;
#[doc = "Field `USISIE` writer - Start Condition Interrupt Enable"]
pub type USISIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:3 - USI Clock Source Select Bits"]
    #[inline(always)]
    pub fn usics(&self) -> USICS_R {
        USICS_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - USI Wire Mode Bits"]
    #[inline(always)]
    pub fn usiwm(&self) -> USIWM_R {
        USIWM_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Counter Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn usioie(&self) -> USIOIE_R {
        USIOIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Start Condition Interrupt Enable"]
    #[inline(always)]
    pub fn usisie(&self) -> USISIE_R {
        USISIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Toggle Clock Port Pin"]
    #[inline(always)]
    #[must_use]
    pub fn usitc(&mut self) -> USITC_W<USICR_SPEC> {
        USITC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Strobe"]
    #[inline(always)]
    #[must_use]
    pub fn usiclk(&mut self) -> USICLK_W<USICR_SPEC> {
        USICLK_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - USI Clock Source Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn usics(&mut self) -> USICS_W<USICR_SPEC> {
        USICS_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - USI Wire Mode Bits"]
    #[inline(always)]
    #[must_use]
    pub fn usiwm(&mut self) -> USIWM_W<USICR_SPEC> {
        USIWM_W::new(self, 4)
    }
    #[doc = "Bit 6 - Counter Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usioie(&mut self) -> USIOIE_W<USICR_SPEC> {
        USIOIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Start Condition Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usisie(&mut self) -> USISIE_W<USICR_SPEC> {
        USISIE_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USICR_SPEC;
impl crate::RegisterSpec for USICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usicr::R`](R) reader structure"]
impl crate::Readable for USICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usicr::W`](W) writer structure"]
impl crate::Writable for USICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USICR to value 0"]
impl crate::Resettable for USICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
