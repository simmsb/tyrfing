#[doc = "Register `PLLFRQ` reader"]
pub type R = crate::R<PLLFRQ_SPEC>;
#[doc = "Register `PLLFRQ` writer"]
pub type W = crate::W<PLLFRQ_SPEC>;
#[doc = "Field `PDIV` reader - PLL Lock Frequency"]
pub type PDIV_R = crate::FieldReader<PDIV_A>;
#[doc = "PLL Lock Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PDIV_A {
    #[doc = "3: 40 MHz"]
    MHZ40 = 3,
    #[doc = "4: 48 MHz"]
    MHZ48 = 4,
    #[doc = "5: 56 MHz"]
    MHZ56 = 5,
    #[doc = "7: 72 MHz"]
    MHZ72 = 7,
    #[doc = "8: 80 MHz"]
    MHZ80 = 8,
    #[doc = "9: 88 MHz"]
    MHZ88 = 9,
    #[doc = "10: 96 MHz"]
    MHZ96 = 10,
}
impl From<PDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PDIV_A {
    type Ux = u8;
}
impl PDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PDIV_A> {
        match self.bits {
            3 => Some(PDIV_A::MHZ40),
            4 => Some(PDIV_A::MHZ48),
            5 => Some(PDIV_A::MHZ56),
            7 => Some(PDIV_A::MHZ72),
            8 => Some(PDIV_A::MHZ80),
            9 => Some(PDIV_A::MHZ88),
            10 => Some(PDIV_A::MHZ96),
            _ => None,
        }
    }
    #[doc = "40 MHz"]
    #[inline(always)]
    pub fn is_mhz40(&self) -> bool {
        *self == PDIV_A::MHZ40
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn is_mhz48(&self) -> bool {
        *self == PDIV_A::MHZ48
    }
    #[doc = "56 MHz"]
    #[inline(always)]
    pub fn is_mhz56(&self) -> bool {
        *self == PDIV_A::MHZ56
    }
    #[doc = "72 MHz"]
    #[inline(always)]
    pub fn is_mhz72(&self) -> bool {
        *self == PDIV_A::MHZ72
    }
    #[doc = "80 MHz"]
    #[inline(always)]
    pub fn is_mhz80(&self) -> bool {
        *self == PDIV_A::MHZ80
    }
    #[doc = "88 MHz"]
    #[inline(always)]
    pub fn is_mhz88(&self) -> bool {
        *self == PDIV_A::MHZ88
    }
    #[doc = "96 MHz"]
    #[inline(always)]
    pub fn is_mhz96(&self) -> bool {
        *self == PDIV_A::MHZ96
    }
}
#[doc = "Field `PDIV` writer - PLL Lock Frequency"]
pub type PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PDIV_A>;
impl<'a, REG> PDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "40 MHz"]
    #[inline(always)]
    pub fn mhz40(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::MHZ40)
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn mhz48(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::MHZ48)
    }
    #[doc = "56 MHz"]
    #[inline(always)]
    pub fn mhz56(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::MHZ56)
    }
    #[doc = "72 MHz"]
    #[inline(always)]
    pub fn mhz72(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::MHZ72)
    }
    #[doc = "80 MHz"]
    #[inline(always)]
    pub fn mhz80(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::MHZ80)
    }
    #[doc = "88 MHz"]
    #[inline(always)]
    pub fn mhz88(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::MHZ88)
    }
    #[doc = "96 MHz"]
    #[inline(always)]
    pub fn mhz96(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::MHZ96)
    }
}
#[doc = "Field `PLLTM` reader - PLL Postscaler for High Speed Timer"]
pub type PLLTM_R = crate::FieldReader<PLLTM_A>;
#[doc = "PLL Postscaler for High Speed Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLTM_A {
    #[doc = "0: 0 (Disconnected)"]
    DISCONNECTED = 0,
    #[doc = "1: 1"]
    FACTOR_1 = 1,
    #[doc = "2: 1.5"]
    FACTOR_15 = 2,
    #[doc = "3: 2"]
    FACTOR_2 = 3,
}
impl From<PLLTM_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLTM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLTM_A {
    type Ux = u8;
}
impl PLLTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLTM_A {
        match self.bits {
            0 => PLLTM_A::DISCONNECTED,
            1 => PLLTM_A::FACTOR_1,
            2 => PLLTM_A::FACTOR_15,
            3 => PLLTM_A::FACTOR_2,
            _ => unreachable!(),
        }
    }
    #[doc = "0 (Disconnected)"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PLLTM_A::DISCONNECTED
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn is_factor_1(&self) -> bool {
        *self == PLLTM_A::FACTOR_1
    }
    #[doc = "1.5"]
    #[inline(always)]
    pub fn is_factor_15(&self) -> bool {
        *self == PLLTM_A::FACTOR_15
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_factor_2(&self) -> bool {
        *self == PLLTM_A::FACTOR_2
    }
}
#[doc = "Field `PLLTM` writer - PLL Postscaler for High Speed Timer"]
pub type PLLTM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PLLTM_A>;
impl<'a, REG> PLLTM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 (Disconnected)"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(PLLTM_A::DISCONNECTED)
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn factor_1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLTM_A::FACTOR_1)
    }
    #[doc = "1.5"]
    #[inline(always)]
    pub fn factor_15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLTM_A::FACTOR_15)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn factor_2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLTM_A::FACTOR_2)
    }
}
#[doc = "Field `PLLUSB` reader - PLL Postscaler for USB Peripheral"]
pub type PLLUSB_R = crate::BitReader;
#[doc = "Field `PLLUSB` writer - PLL Postscaler for USB Peripheral"]
pub type PLLUSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINMUX` reader - PLL Input Multiplexer"]
pub type PINMUX_R = crate::BitReader;
#[doc = "Field `PINMUX` writer - PLL Input Multiplexer"]
pub type PINMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - PLL Lock Frequency"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - PLL Postscaler for High Speed Timer"]
    #[inline(always)]
    pub fn plltm(&self) -> PLLTM_R {
        PLLTM_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - PLL Postscaler for USB Peripheral"]
    #[inline(always)]
    pub fn pllusb(&self) -> PLLUSB_R {
        PLLUSB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL Input Multiplexer"]
    #[inline(always)]
    pub fn pinmux(&self) -> PINMUX_R {
        PINMUX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PLL Lock Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PDIV_W<PLLFRQ_SPEC> {
        PDIV_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - PLL Postscaler for High Speed Timer"]
    #[inline(always)]
    #[must_use]
    pub fn plltm(&mut self) -> PLLTM_W<PLLFRQ_SPEC> {
        PLLTM_W::new(self, 4)
    }
    #[doc = "Bit 6 - PLL Postscaler for USB Peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn pllusb(&mut self) -> PLLUSB_W<PLLFRQ_SPEC> {
        PLLUSB_W::new(self, 6)
    }
    #[doc = "Bit 7 - PLL Input Multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn pinmux(&mut self) -> PINMUX_W<PLLFRQ_SPEC> {
        PINMUX_W::new(self, 7)
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
#[doc = "PLL Frequency Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllfrq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllfrq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLFRQ_SPEC;
impl crate::RegisterSpec for PLLFRQ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pllfrq::R`](R) reader structure"]
impl crate::Readable for PLLFRQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllfrq::W`](W) writer structure"]
impl crate::Writable for PLLFRQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLFRQ to value 0"]
impl crate::Resettable for PLLFRQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
