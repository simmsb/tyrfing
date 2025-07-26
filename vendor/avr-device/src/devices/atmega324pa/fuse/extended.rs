#[doc = "Register `EXTENDED` reader"]
pub type R = crate::R<EXTENDED_SPEC>;
#[doc = "Register `EXTENDED` writer"]
pub type W = crate::W<EXTENDED_SPEC>;
#[doc = "Field `BODLEVEL` reader - Brown-out Detector trigger level"]
pub type BODLEVEL_R = crate::FieldReader<BODLEVEL_A>;
#[doc = "Brown-out Detector trigger level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BODLEVEL_A {
    #[doc = "4: Brown-out detection at VCC=4.3 V"]
    _4V3 = 4,
    #[doc = "5: Brown-out detection at VCC=2.7 V"]
    _2V7 = 5,
    #[doc = "6: Brown-out detection at VCC=1.8 V"]
    _1V8 = 6,
    #[doc = "7: Brown-out detection disabled"]
    DISABLED = 7,
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
    pub const fn variant(&self) -> Option<BODLEVEL_A> {
        match self.bits {
            4 => Some(BODLEVEL_A::_4V3),
            5 => Some(BODLEVEL_A::_2V7),
            6 => Some(BODLEVEL_A::_1V8),
            7 => Some(BODLEVEL_A::DISABLED),
            _ => None,
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
#[doc = "Field `BODLEVEL` writer - Brown-out Detector trigger level"]
pub type BODLEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BODLEVEL_A>;
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
impl R {
    #[doc = "Bits 0:2 - Brown-out Detector trigger level"]
    #[inline(always)]
    pub fn bodlevel(&self) -> BODLEVEL_R {
        BODLEVEL_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Brown-out Detector trigger level"]
    #[inline(always)]
    #[must_use]
    pub fn bodlevel(&mut self) -> BODLEVEL_W<EXTENDED_SPEC> {
        BODLEVEL_W::new(self, 0)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extended::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extended::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTENDED_SPEC;
impl crate::RegisterSpec for EXTENDED_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`extended::R`](R) reader structure"]
impl crate::Readable for EXTENDED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extended::W`](W) writer structure"]
impl crate::Writable for EXTENDED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTENDED to value 0"]
impl crate::Resettable for EXTENDED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
