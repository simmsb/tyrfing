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
    #[doc = "0: Brown-out detection at VCC=4.3 V"]
    _4V3 = 0,
    #[doc = "1: Brown-out detection at VCC=3.5 V"]
    _3V5 = 1,
    #[doc = "2: Brown-out detection at VCC=3.4 V"]
    _3V4 = 2,
    #[doc = "3: Brown-out detection at VCC=2.6 V"]
    _2V6 = 3,
    #[doc = "4: Brown-out detection at VCC=2.4 V"]
    _2V4 = 4,
    #[doc = "5: Brown-out detection at VCC=2.2 V"]
    _2V2 = 5,
    #[doc = "6: Brown-out detection at VCC=2.0 V"]
    _2V0 = 6,
    #[doc = "7: Brown-out detection disabled; \\[BODLEVEL=111\\]"]
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
    pub const fn variant(&self) -> BODLEVEL_A {
        match self.bits {
            0 => BODLEVEL_A::_4V3,
            1 => BODLEVEL_A::_3V5,
            2 => BODLEVEL_A::_3V4,
            3 => BODLEVEL_A::_2V6,
            4 => BODLEVEL_A::_2V4,
            5 => BODLEVEL_A::_2V2,
            6 => BODLEVEL_A::_2V0,
            7 => BODLEVEL_A::DISABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Brown-out detection at VCC=4.3 V"]
    #[inline(always)]
    pub fn is_4v3(&self) -> bool {
        *self == BODLEVEL_A::_4V3
    }
    #[doc = "Brown-out detection at VCC=3.5 V"]
    #[inline(always)]
    pub fn is_3v5(&self) -> bool {
        *self == BODLEVEL_A::_3V5
    }
    #[doc = "Brown-out detection at VCC=3.4 V"]
    #[inline(always)]
    pub fn is_3v4(&self) -> bool {
        *self == BODLEVEL_A::_3V4
    }
    #[doc = "Brown-out detection at VCC=2.6 V"]
    #[inline(always)]
    pub fn is_2v6(&self) -> bool {
        *self == BODLEVEL_A::_2V6
    }
    #[doc = "Brown-out detection at VCC=2.4 V"]
    #[inline(always)]
    pub fn is_2v4(&self) -> bool {
        *self == BODLEVEL_A::_2V4
    }
    #[doc = "Brown-out detection at VCC=2.2 V"]
    #[inline(always)]
    pub fn is_2v2(&self) -> bool {
        *self == BODLEVEL_A::_2V2
    }
    #[doc = "Brown-out detection at VCC=2.0 V"]
    #[inline(always)]
    pub fn is_2v0(&self) -> bool {
        *self == BODLEVEL_A::_2V0
    }
    #[doc = "Brown-out detection disabled; \\[BODLEVEL=111\\]"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BODLEVEL_A::DISABLED
    }
}
#[doc = "Field `BODLEVEL` writer - Brown-out Detector trigger level"]
pub type BODLEVEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, BODLEVEL_A>;
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
    #[doc = "Brown-out detection at VCC=3.5 V"]
    #[inline(always)]
    pub fn _3v5(self) -> &'a mut crate::W<REG> {
        self.variant(BODLEVEL_A::_3V5)
    }
    #[doc = "Brown-out detection at VCC=3.4 V"]
    #[inline(always)]
    pub fn _3v4(self) -> &'a mut crate::W<REG> {
        self.variant(BODLEVEL_A::_3V4)
    }
    #[doc = "Brown-out detection at VCC=2.6 V"]
    #[inline(always)]
    pub fn _2v6(self) -> &'a mut crate::W<REG> {
        self.variant(BODLEVEL_A::_2V6)
    }
    #[doc = "Brown-out detection at VCC=2.4 V"]
    #[inline(always)]
    pub fn _2v4(self) -> &'a mut crate::W<REG> {
        self.variant(BODLEVEL_A::_2V4)
    }
    #[doc = "Brown-out detection at VCC=2.2 V"]
    #[inline(always)]
    pub fn _2v2(self) -> &'a mut crate::W<REG> {
        self.variant(BODLEVEL_A::_2V2)
    }
    #[doc = "Brown-out detection at VCC=2.0 V"]
    #[inline(always)]
    pub fn _2v0(self) -> &'a mut crate::W<REG> {
        self.variant(BODLEVEL_A::_2V0)
    }
    #[doc = "Brown-out detection disabled; \\[BODLEVEL=111\\]"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BODLEVEL_A::DISABLED)
    }
}
#[doc = "Field `HWBE` reader - Hardware Boot Enable"]
pub type HWBE_R = crate::BitReader;
#[doc = "Field `HWBE` writer - Hardware Boot Enable"]
pub type HWBE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Brown-out Detector trigger level"]
    #[inline(always)]
    pub fn bodlevel(&self) -> BODLEVEL_R {
        BODLEVEL_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Hardware Boot Enable"]
    #[inline(always)]
    pub fn hwbe(&self) -> HWBE_R {
        HWBE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Brown-out Detector trigger level"]
    #[inline(always)]
    #[must_use]
    pub fn bodlevel(&mut self) -> BODLEVEL_W<EXTENDED_SPEC> {
        BODLEVEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - Hardware Boot Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hwbe(&mut self) -> HWBE_W<EXTENDED_SPEC> {
        HWBE_W::new(self, 3)
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
