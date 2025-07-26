#[doc = "Register `HIGH` reader"]
pub type R = crate::R<HIGH_SPEC>;
#[doc = "Register `HIGH` writer"]
pub type W = crate::W<HIGH_SPEC>;
#[doc = "Field `RSTDISBL` reader - Reset Disabled (Enable PB5 as i/o pin)"]
pub type RSTDISBL_R = crate::BitReader;
#[doc = "Field `RSTDISBL` writer - Reset Disabled (Enable PB5 as i/o pin)"]
pub type RSTDISBL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODLEVEL` reader - Enable BOD and select level"]
pub type BODLEVEL_R = crate::FieldReader<BODLEVEL_A>;
#[doc = "Enable BOD and select level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BODLEVEL_A {
    #[doc = "0: Brown-out detection at VCC=4.3 V"]
    _4V3 = 0,
    #[doc = "1: Brown-out detection at VCC=2.7 V"]
    _2V7 = 1,
    #[doc = "2: Brown-out detection at VCC=1.8 V"]
    _1V8 = 2,
    #[doc = "3: Brown-out detection disabled"]
    DISABLED = 3,
}
impl From<BODLEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BODLEVEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BODLEVEL_A {
    type Ux = u8;
}
impl BODLEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BODLEVEL_A {
        match self.bits {
            0 => BODLEVEL_A::_4V3,
            1 => BODLEVEL_A::_2V7,
            2 => BODLEVEL_A::_1V8,
            3 => BODLEVEL_A::DISABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Brown-out detection at VCC=4.3 V"]
    #[inline(always)]
    pub fn is_4v3(&self) -> bool {
        *self == BODLEVEL_A::_4V3
    }
    #[doc = "Brown-out detection at VCC=2.7 V"]
    #[inline(always)]
    pub fn is_2v7(&self) -> bool {
        *self == BODLEVEL_A::_2V7
    }
    #[doc = "Brown-out detection at VCC=1.8 V"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        *self == BODLEVEL_A::_1V8
    }
    #[doc = "Brown-out detection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BODLEVEL_A::DISABLED
    }
}
#[doc = "Field `BODLEVEL` writer - Enable BOD and select level"]
pub type BODLEVEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BODLEVEL_A>;
impl<'a, REG> BODLEVEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Brown-out detection at VCC=4.3 V"]
    #[inline(always)]
    pub fn _4v3(self) -> &'a mut crate::W<REG> {
        self.variant(BODLEVEL_A::_4V3)
    }
    #[doc = "Brown-out detection at VCC=2.7 V"]
    #[inline(always)]
    pub fn _2v7(self) -> &'a mut crate::W<REG> {
        self.variant(BODLEVEL_A::_2V7)
    }
    #[doc = "Brown-out detection at VCC=1.8 V"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut crate::W<REG> {
        self.variant(BODLEVEL_A::_1V8)
    }
    #[doc = "Brown-out detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BODLEVEL_A::DISABLED)
    }
}
#[doc = "Field `DWEN` reader - Debug Wire enable"]
pub type DWEN_R = crate::BitReader;
#[doc = "Field `DWEN` writer - Debug Wire enable"]
pub type DWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELFPRGEN` reader - Self Programming enable"]
pub type SELFPRGEN_R = crate::BitReader;
#[doc = "Field `SELFPRGEN` writer - Self Programming enable"]
pub type SELFPRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset Disabled (Enable PB5 as i/o pin)"]
    #[inline(always)]
    pub fn rstdisbl(&self) -> RSTDISBL_R {
        RSTDISBL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enable BOD and select level"]
    #[inline(always)]
    pub fn bodlevel(&self) -> BODLEVEL_R {
        BODLEVEL_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Debug Wire enable"]
    #[inline(always)]
    pub fn dwen(&self) -> DWEN_R {
        DWEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Self Programming enable"]
    #[inline(always)]
    pub fn selfprgen(&self) -> SELFPRGEN_R {
        SELFPRGEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Disabled (Enable PB5 as i/o pin)"]
    #[inline(always)]
    #[must_use]
    pub fn rstdisbl(&mut self) -> RSTDISBL_W<HIGH_SPEC> {
        RSTDISBL_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Enable BOD and select level"]
    #[inline(always)]
    #[must_use]
    pub fn bodlevel(&mut self) -> BODLEVEL_W<HIGH_SPEC> {
        BODLEVEL_W::new(self, 1)
    }
    #[doc = "Bit 3 - Debug Wire enable"]
    #[inline(always)]
    #[must_use]
    pub fn dwen(&mut self) -> DWEN_W<HIGH_SPEC> {
        DWEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Self Programming enable"]
    #[inline(always)]
    #[must_use]
    pub fn selfprgen(&mut self) -> SELFPRGEN_W<HIGH_SPEC> {
        SELFPRGEN_W::new(self, 4)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIGH_SPEC;
impl crate::RegisterSpec for HIGH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`high::R`](R) reader structure"]
impl crate::Readable for HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`high::W`](W) writer structure"]
impl crate::Writable for HIGH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIGH to value 0"]
impl crate::Resettable for HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
