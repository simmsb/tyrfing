#[doc = "Register `TOCPMSA0` reader"]
pub type R = crate::R<TOCPMSA0_SPEC>;
#[doc = "Register `TOCPMSA0` writer"]
pub type W = crate::W<TOCPMSA0_SPEC>;
#[doc = "Field `TOCC0S` reader - Timer Output Compare Channel 0 Selection Bits"]
pub type TOCC0S_R = crate::FieldReader;
#[doc = "Field `TOCC0S` writer - Timer Output Compare Channel 0 Selection Bits"]
pub type TOCC0S_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `TOCC1S` reader - Timer Output Compare Channel 1 Selection Bits"]
pub type TOCC1S_R = crate::FieldReader;
#[doc = "Field `TOCC1S` writer - Timer Output Compare Channel 1 Selection Bits"]
pub type TOCC1S_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `TOCC2S` reader - Timer Output Compare Channel 2 Selection Bits"]
pub type TOCC2S_R = crate::FieldReader;
#[doc = "Field `TOCC2S` writer - Timer Output Compare Channel 2 Selection Bits"]
pub type TOCC2S_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `TOCC3S` reader - Timer Output Compare Channel 3 Selection Bits"]
pub type TOCC3S_R = crate::FieldReader;
#[doc = "Field `TOCC3S` writer - Timer Output Compare Channel 3 Selection Bits"]
pub type TOCC3S_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Timer Output Compare Channel 0 Selection Bits"]
    #[inline(always)]
    pub fn tocc0s(&self) -> TOCC0S_R {
        TOCC0S_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Timer Output Compare Channel 1 Selection Bits"]
    #[inline(always)]
    pub fn tocc1s(&self) -> TOCC1S_R {
        TOCC1S_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Timer Output Compare Channel 2 Selection Bits"]
    #[inline(always)]
    pub fn tocc2s(&self) -> TOCC2S_R {
        TOCC2S_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Timer Output Compare Channel 3 Selection Bits"]
    #[inline(always)]
    pub fn tocc3s(&self) -> TOCC3S_R {
        TOCC3S_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Output Compare Channel 0 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc0s(&mut self) -> TOCC0S_W<TOCPMSA0_SPEC> {
        TOCC0S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Timer Output Compare Channel 1 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc1s(&mut self) -> TOCC1S_W<TOCPMSA0_SPEC> {
        TOCC1S_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Timer Output Compare Channel 2 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc2s(&mut self) -> TOCC2S_W<TOCPMSA0_SPEC> {
        TOCC2S_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Timer Output Compare Channel 3 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc3s(&mut self) -> TOCC3S_W<TOCPMSA0_SPEC> {
        TOCC3S_W::new(self, 6)
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
#[doc = "Timer Output Compare Pin Mux Selection 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocpmsa0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocpmsa0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOCPMSA0_SPEC;
impl crate::RegisterSpec for TOCPMSA0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tocpmsa0::R`](R) reader structure"]
impl crate::Readable for TOCPMSA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tocpmsa0::W`](W) writer structure"]
impl crate::Writable for TOCPMSA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOCPMSA0 to value 0"]
impl crate::Resettable for TOCPMSA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
