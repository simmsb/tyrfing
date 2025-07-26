#[doc = "Register `MCLKCTRLB` reader"]
pub type R = crate::R<MCLKCTRLB_SPEC>;
#[doc = "Register `MCLKCTRLB` writer"]
pub type W = crate::W<MCLKCTRLB_SPEC>;
#[doc = "Field `PEN` reader - Prescaler enable"]
pub type PEN_R = crate::BitReader;
#[doc = "Field `PEN` writer - Prescaler enable"]
pub type PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDIV` reader - Prescaler division"]
pub type PDIV_R = crate::FieldReader<PDIV_A>;
#[doc = "Prescaler division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PDIV_A {
    #[doc = "0: 2X"]
    _2X = 0,
    #[doc = "1: 4X"]
    _4X = 1,
    #[doc = "2: 8X"]
    _8X = 2,
    #[doc = "3: 16X"]
    _16X = 3,
    #[doc = "4: 32X"]
    _32X = 4,
    #[doc = "5: 64X"]
    _64X = 5,
    #[doc = "8: 6X"]
    _6X = 8,
    #[doc = "9: 10X"]
    _10X = 9,
    #[doc = "10: 12X"]
    _12X = 10,
    #[doc = "11: 24X"]
    _24X = 11,
    #[doc = "12: 48X"]
    _48X = 12,
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
            0 => Some(PDIV_A::_2X),
            1 => Some(PDIV_A::_4X),
            2 => Some(PDIV_A::_8X),
            3 => Some(PDIV_A::_16X),
            4 => Some(PDIV_A::_32X),
            5 => Some(PDIV_A::_64X),
            8 => Some(PDIV_A::_6X),
            9 => Some(PDIV_A::_10X),
            10 => Some(PDIV_A::_12X),
            11 => Some(PDIV_A::_24X),
            12 => Some(PDIV_A::_48X),
            _ => None,
        }
    }
    #[doc = "2X"]
    #[inline(always)]
    pub fn is_2x(&self) -> bool {
        *self == PDIV_A::_2X
    }
    #[doc = "4X"]
    #[inline(always)]
    pub fn is_4x(&self) -> bool {
        *self == PDIV_A::_4X
    }
    #[doc = "8X"]
    #[inline(always)]
    pub fn is_8x(&self) -> bool {
        *self == PDIV_A::_8X
    }
    #[doc = "16X"]
    #[inline(always)]
    pub fn is_16x(&self) -> bool {
        *self == PDIV_A::_16X
    }
    #[doc = "32X"]
    #[inline(always)]
    pub fn is_32x(&self) -> bool {
        *self == PDIV_A::_32X
    }
    #[doc = "64X"]
    #[inline(always)]
    pub fn is_64x(&self) -> bool {
        *self == PDIV_A::_64X
    }
    #[doc = "6X"]
    #[inline(always)]
    pub fn is_6x(&self) -> bool {
        *self == PDIV_A::_6X
    }
    #[doc = "10X"]
    #[inline(always)]
    pub fn is_10x(&self) -> bool {
        *self == PDIV_A::_10X
    }
    #[doc = "12X"]
    #[inline(always)]
    pub fn is_12x(&self) -> bool {
        *self == PDIV_A::_12X
    }
    #[doc = "24X"]
    #[inline(always)]
    pub fn is_24x(&self) -> bool {
        *self == PDIV_A::_24X
    }
    #[doc = "48X"]
    #[inline(always)]
    pub fn is_48x(&self) -> bool {
        *self == PDIV_A::_48X
    }
}
#[doc = "Field `PDIV` writer - Prescaler division"]
pub type PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PDIV_A>;
impl<'a, REG> PDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2X"]
    #[inline(always)]
    pub fn _2x(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::_2X)
    }
    #[doc = "4X"]
    #[inline(always)]
    pub fn _4x(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::_4X)
    }
    #[doc = "8X"]
    #[inline(always)]
    pub fn _8x(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::_8X)
    }
    #[doc = "16X"]
    #[inline(always)]
    pub fn _16x(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::_16X)
    }
    #[doc = "32X"]
    #[inline(always)]
    pub fn _32x(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::_32X)
    }
    #[doc = "64X"]
    #[inline(always)]
    pub fn _64x(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::_64X)
    }
    #[doc = "6X"]
    #[inline(always)]
    pub fn _6x(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::_6X)
    }
    #[doc = "10X"]
    #[inline(always)]
    pub fn _10x(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::_10X)
    }
    #[doc = "12X"]
    #[inline(always)]
    pub fn _12x(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::_12X)
    }
    #[doc = "24X"]
    #[inline(always)]
    pub fn _24x(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::_24X)
    }
    #[doc = "48X"]
    #[inline(always)]
    pub fn _48x(self) -> &'a mut crate::W<REG> {
        self.variant(PDIV_A::_48X)
    }
}
impl R {
    #[doc = "Bit 0 - Prescaler enable"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Prescaler division"]
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits >> 1) & 0x0f)
    }
}
impl W {
    #[doc = "Bit 0 - Prescaler enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<MCLKCTRLB_SPEC> {
        PEN_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Prescaler division"]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PDIV_W<MCLKCTRLB_SPEC> {
        PDIV_W::new(self, 1)
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
#[doc = "MCLK Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCLKCTRLB_SPEC;
impl crate::RegisterSpec for MCLKCTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mclkctrlb::R`](R) reader structure"]
impl crate::Readable for MCLKCTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mclkctrlb::W`](W) writer structure"]
impl crate::Writable for MCLKCTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKCTRLB to value 0"]
impl crate::Resettable for MCLKCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
