#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `CROSSIF` reader - ZCD Interrupt Flag"]
pub type CROSSIF_R = crate::BitReader;
#[doc = "Field `CROSSIF` writer - ZCD Interrupt Flag"]
pub type CROSSIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATE` reader - ZCD State"]
pub type STATE_R = crate::BitReader<STATE_A>;
#[doc = "ZCD State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STATE_A {
    #[doc = "0: Output is 0"]
    LOW = 0,
    #[doc = "1: Output is 1"]
    HIGH = 1,
}
impl From<STATE_A> for bool {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STATE_A {
        match self.bits {
            false => STATE_A::LOW,
            true => STATE_A::HIGH,
        }
    }
    #[doc = "Output is 0"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == STATE_A::LOW
    }
    #[doc = "Output is 1"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == STATE_A::HIGH
    }
}
impl R {
    #[doc = "Bit 0 - ZCD Interrupt Flag"]
    #[inline(always)]
    pub fn crossif(&self) -> CROSSIF_R {
        CROSSIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ZCD State"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ZCD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn crossif(&mut self) -> CROSSIF_W<STATUS_SPEC> {
        CROSSIF_W::new(self, 0)
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
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
