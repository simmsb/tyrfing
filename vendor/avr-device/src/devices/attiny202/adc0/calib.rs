#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CALIB_SPEC>;
#[doc = "Register `CALIB` writer"]
pub type W = crate::W<CALIB_SPEC>;
#[doc = "Field `DUTYCYC` reader - Duty Cycle"]
pub type DUTYCYC_R = crate::BitReader<DUTYCYC_A>;
#[doc = "Duty Cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUTYCYC_A {
    #[doc = "0: 50% Duty cycle"]
    DUTY50 = 0,
    #[doc = "1: 25% Duty cycle"]
    DUTY25 = 1,
}
impl From<DUTYCYC_A> for bool {
    #[inline(always)]
    fn from(variant: DUTYCYC_A) -> Self {
        variant as u8 != 0
    }
}
impl DUTYCYC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DUTYCYC_A {
        match self.bits {
            false => DUTYCYC_A::DUTY50,
            true => DUTYCYC_A::DUTY25,
        }
    }
    #[doc = "50% Duty cycle"]
    #[inline(always)]
    pub fn is_duty50(&self) -> bool {
        *self == DUTYCYC_A::DUTY50
    }
    #[doc = "25% Duty cycle"]
    #[inline(always)]
    pub fn is_duty25(&self) -> bool {
        *self == DUTYCYC_A::DUTY25
    }
}
#[doc = "Field `DUTYCYC` writer - Duty Cycle"]
pub type DUTYCYC_W<'a, REG> = crate::BitWriter<'a, REG, DUTYCYC_A>;
impl<'a, REG> DUTYCYC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "50% Duty cycle"]
    #[inline(always)]
    pub fn duty50(self) -> &'a mut crate::W<REG> {
        self.variant(DUTYCYC_A::DUTY50)
    }
    #[doc = "25% Duty cycle"]
    #[inline(always)]
    pub fn duty25(self) -> &'a mut crate::W<REG> {
        self.variant(DUTYCYC_A::DUTY25)
    }
}
impl R {
    #[doc = "Bit 0 - Duty Cycle"]
    #[inline(always)]
    pub fn dutycyc(&self) -> DUTYCYC_R {
        DUTYCYC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Duty Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn dutycyc(&mut self) -> DUTYCYC_W<CALIB_SPEC> {
        DUTYCYC_W::new(self, 0)
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
#[doc = "Calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calib::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calib::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`calib::R`](R) reader structure"]
impl crate::Readable for CALIB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calib::W`](W) writer structure"]
impl crate::Writable for CALIB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
