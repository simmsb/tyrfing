#[doc = "Register `OSCHFCTRLA` reader"]
pub type R = crate::R<OSCHFCTRLA_SPEC>;
#[doc = "Register `OSCHFCTRLA` writer"]
pub type W = crate::W<OSCHFCTRLA_SPEC>;
#[doc = "Field `AUTOTUNE` reader - Autotune"]
pub type AUTOTUNE_R = crate::BitReader;
#[doc = "Field `AUTOTUNE` writer - Autotune"]
pub type AUTOTUNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRQSEL` reader - Frequency select"]
pub type FRQSEL_R = crate::FieldReader<FRQSEL_A>;
#[doc = "Frequency select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRQSEL_A {
    #[doc = "0: 1 MHz system clock"]
    _1M = 0,
    #[doc = "1: 2 MHz system clock"]
    _2M = 1,
    #[doc = "2: 3 MHz system clock"]
    _3M = 2,
    #[doc = "3: 4 MHz system clock (default)"]
    _4M = 3,
    #[doc = "5: 8 MHz system clock"]
    _8M = 5,
    #[doc = "6: 12 MHz system clock"]
    _12M = 6,
    #[doc = "7: 16 MHz system clock"]
    _16M = 7,
    #[doc = "8: 20 MHz system clock"]
    _20M = 8,
    #[doc = "9: 24 MHz system clock"]
    _24M = 9,
}
impl From<FRQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FRQSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FRQSEL_A {
    type Ux = u8;
}
impl FRQSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FRQSEL_A> {
        match self.bits {
            0 => Some(FRQSEL_A::_1M),
            1 => Some(FRQSEL_A::_2M),
            2 => Some(FRQSEL_A::_3M),
            3 => Some(FRQSEL_A::_4M),
            5 => Some(FRQSEL_A::_8M),
            6 => Some(FRQSEL_A::_12M),
            7 => Some(FRQSEL_A::_16M),
            8 => Some(FRQSEL_A::_20M),
            9 => Some(FRQSEL_A::_24M),
            _ => None,
        }
    }
    #[doc = "1 MHz system clock"]
    #[inline(always)]
    pub fn is_1m(&self) -> bool {
        *self == FRQSEL_A::_1M
    }
    #[doc = "2 MHz system clock"]
    #[inline(always)]
    pub fn is_2m(&self) -> bool {
        *self == FRQSEL_A::_2M
    }
    #[doc = "3 MHz system clock"]
    #[inline(always)]
    pub fn is_3m(&self) -> bool {
        *self == FRQSEL_A::_3M
    }
    #[doc = "4 MHz system clock (default)"]
    #[inline(always)]
    pub fn is_4m(&self) -> bool {
        *self == FRQSEL_A::_4M
    }
    #[doc = "8 MHz system clock"]
    #[inline(always)]
    pub fn is_8m(&self) -> bool {
        *self == FRQSEL_A::_8M
    }
    #[doc = "12 MHz system clock"]
    #[inline(always)]
    pub fn is_12m(&self) -> bool {
        *self == FRQSEL_A::_12M
    }
    #[doc = "16 MHz system clock"]
    #[inline(always)]
    pub fn is_16m(&self) -> bool {
        *self == FRQSEL_A::_16M
    }
    #[doc = "20 MHz system clock"]
    #[inline(always)]
    pub fn is_20m(&self) -> bool {
        *self == FRQSEL_A::_20M
    }
    #[doc = "24 MHz system clock"]
    #[inline(always)]
    pub fn is_24m(&self) -> bool {
        *self == FRQSEL_A::_24M
    }
}
#[doc = "Field `FRQSEL` writer - Frequency select"]
pub type FRQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, FRQSEL_A>;
impl<'a, REG> FRQSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 MHz system clock"]
    #[inline(always)]
    pub fn _1m(self) -> &'a mut crate::W<REG> {
        self.variant(FRQSEL_A::_1M)
    }
    #[doc = "2 MHz system clock"]
    #[inline(always)]
    pub fn _2m(self) -> &'a mut crate::W<REG> {
        self.variant(FRQSEL_A::_2M)
    }
    #[doc = "3 MHz system clock"]
    #[inline(always)]
    pub fn _3m(self) -> &'a mut crate::W<REG> {
        self.variant(FRQSEL_A::_3M)
    }
    #[doc = "4 MHz system clock (default)"]
    #[inline(always)]
    pub fn _4m(self) -> &'a mut crate::W<REG> {
        self.variant(FRQSEL_A::_4M)
    }
    #[doc = "8 MHz system clock"]
    #[inline(always)]
    pub fn _8m(self) -> &'a mut crate::W<REG> {
        self.variant(FRQSEL_A::_8M)
    }
    #[doc = "12 MHz system clock"]
    #[inline(always)]
    pub fn _12m(self) -> &'a mut crate::W<REG> {
        self.variant(FRQSEL_A::_12M)
    }
    #[doc = "16 MHz system clock"]
    #[inline(always)]
    pub fn _16m(self) -> &'a mut crate::W<REG> {
        self.variant(FRQSEL_A::_16M)
    }
    #[doc = "20 MHz system clock"]
    #[inline(always)]
    pub fn _20m(self) -> &'a mut crate::W<REG> {
        self.variant(FRQSEL_A::_20M)
    }
    #[doc = "24 MHz system clock"]
    #[inline(always)]
    pub fn _24m(self) -> &'a mut crate::W<REG> {
        self.variant(FRQSEL_A::_24M)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run standby"]
pub type RUNSTDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Autotune"]
    #[inline(always)]
    pub fn autotune(&self) -> AUTOTUNE_R {
        AUTOTUNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:5 - Frequency select"]
    #[inline(always)]
    pub fn frqsel(&self) -> FRQSEL_R {
        FRQSEL_R::new((self.bits >> 2) & 0x0f)
    }
    #[doc = "Bit 7 - Run standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Autotune"]
    #[inline(always)]
    #[must_use]
    pub fn autotune(&mut self) -> AUTOTUNE_W<OSCHFCTRLA_SPEC> {
        AUTOTUNE_W::new(self, 0)
    }
    #[doc = "Bits 2:5 - Frequency select"]
    #[inline(always)]
    #[must_use]
    pub fn frqsel(&mut self) -> FRQSEL_W<OSCHFCTRLA_SPEC> {
        FRQSEL_W::new(self, 2)
    }
    #[doc = "Bit 7 - Run standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<OSCHFCTRLA_SPEC> {
        RUNSTDBY_W::new(self, 7)
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
#[doc = "OSCHF Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oschfctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oschfctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCHFCTRLA_SPEC;
impl crate::RegisterSpec for OSCHFCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`oschfctrla::R`](R) reader structure"]
impl crate::Readable for OSCHFCTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oschfctrla::W`](W) writer structure"]
impl crate::Writable for OSCHFCTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCHFCTRLA to value 0"]
impl crate::Resettable for OSCHFCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
