#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `VLMS` reader - Voltage level monitor status"]
pub type VLMS_R = crate::BitReader<VLMS_A>;
#[doc = "Voltage level monitor status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VLMS_A {
    #[doc = "0: The voltage is above the VLM threshold level"]
    ABOVE = 0,
    #[doc = "1: The voltage is below the VLM threshold level"]
    BELOW = 1,
}
impl From<VLMS_A> for bool {
    #[inline(always)]
    fn from(variant: VLMS_A) -> Self {
        variant as u8 != 0
    }
}
impl VLMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VLMS_A {
        match self.bits {
            false => VLMS_A::ABOVE,
            true => VLMS_A::BELOW,
        }
    }
    #[doc = "The voltage is above the VLM threshold level"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == VLMS_A::ABOVE
    }
    #[doc = "The voltage is below the VLM threshold level"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == VLMS_A::BELOW
    }
}
impl R {
    #[doc = "Bit 0 - Voltage level monitor status"]
    #[inline(always)]
    pub fn vlms(&self) -> VLMS_R {
        VLMS_R::new((self.bits & 1) != 0)
    }
}
impl W {
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
#[doc = "Voltage level monitor status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
