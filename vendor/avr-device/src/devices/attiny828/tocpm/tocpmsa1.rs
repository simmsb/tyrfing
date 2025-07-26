#[doc = "Register `TOCPMSA1` reader"]
pub type R = crate::R<TOCPMSA1_SPEC>;
#[doc = "Register `TOCPMSA1` writer"]
pub type W = crate::W<TOCPMSA1_SPEC>;
#[doc = "Field `TOCC4S` reader - Timer Output Compare Channel 4 Selection Bits"]
pub type TOCC4S_R = crate::FieldReader;
#[doc = "Field `TOCC4S` writer - Timer Output Compare Channel 4 Selection Bits"]
pub type TOCC4S_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `TOCC5S` reader - Timer Output Compare Channel 5 Selection Bits"]
pub type TOCC5S_R = crate::FieldReader;
#[doc = "Field `TOCC5S` writer - Timer Output Compare Channel 5 Selection Bits"]
pub type TOCC5S_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `TOCC6S` reader - Timer Output Compare Channel 6 Selection Bits"]
pub type TOCC6S_R = crate::FieldReader;
#[doc = "Field `TOCC6S` writer - Timer Output Compare Channel 6 Selection Bits"]
pub type TOCC6S_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `TOCC7S` reader - Timer Output Compare Channel 7 Selection Bits"]
pub type TOCC7S_R = crate::FieldReader;
#[doc = "Field `TOCC7S` writer - Timer Output Compare Channel 7 Selection Bits"]
pub type TOCC7S_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Timer Output Compare Channel 4 Selection Bits"]
    #[inline(always)]
    pub fn tocc4s(&self) -> TOCC4S_R {
        TOCC4S_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Timer Output Compare Channel 5 Selection Bits"]
    #[inline(always)]
    pub fn tocc5s(&self) -> TOCC5S_R {
        TOCC5S_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Timer Output Compare Channel 6 Selection Bits"]
    #[inline(always)]
    pub fn tocc6s(&self) -> TOCC6S_R {
        TOCC6S_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Timer Output Compare Channel 7 Selection Bits"]
    #[inline(always)]
    pub fn tocc7s(&self) -> TOCC7S_R {
        TOCC7S_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Output Compare Channel 4 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc4s(&mut self) -> TOCC4S_W<TOCPMSA1_SPEC> {
        TOCC4S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Timer Output Compare Channel 5 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc5s(&mut self) -> TOCC5S_W<TOCPMSA1_SPEC> {
        TOCC5S_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Timer Output Compare Channel 6 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc6s(&mut self) -> TOCC6S_W<TOCPMSA1_SPEC> {
        TOCC6S_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Timer Output Compare Channel 7 Selection Bits"]
    #[inline(always)]
    #[must_use]
    pub fn tocc7s(&mut self) -> TOCC7S_W<TOCPMSA1_SPEC> {
        TOCC7S_W::new(self, 6)
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
#[doc = "Timer Output Compare Pin Mux Selection 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocpmsa1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocpmsa1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOCPMSA1_SPEC;
impl crate::RegisterSpec for TOCPMSA1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tocpmsa1::R`](R) reader structure"]
impl crate::Readable for TOCPMSA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tocpmsa1::W`](W) writer structure"]
impl crate::Writable for TOCPMSA1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOCPMSA1 to value 0"]
impl crate::Resettable for TOCPMSA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
