#[doc = "Register `EICRA` reader"]
pub type R = crate::R<EICRA_SPEC>;
#[doc = "Register `EICRA` writer"]
pub type W = crate::W<EICRA_SPEC>;
#[doc = "Field `ISC0` reader - External Interrupt Sense Control Bit"]
pub type ISC0_R = crate::FieldReader;
#[doc = "Field `ISC0` writer - External Interrupt Sense Control Bit"]
pub type ISC0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `ISC1` reader - External Interrupt Sense Control Bit"]
pub type ISC1_R = crate::FieldReader;
#[doc = "Field `ISC1` writer - External Interrupt Sense Control Bit"]
pub type ISC1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `ISC2` reader - External Interrupt Sense Control Bit"]
pub type ISC2_R = crate::FieldReader;
#[doc = "Field `ISC2` writer - External Interrupt Sense Control Bit"]
pub type ISC2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `ISC3` reader - External Interrupt Sense Control Bit"]
pub type ISC3_R = crate::FieldReader;
#[doc = "Field `ISC3` writer - External Interrupt Sense Control Bit"]
pub type ISC3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    pub fn isc0(&self) -> ISC0_R {
        ISC0_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    pub fn isc1(&self) -> ISC1_R {
        ISC1_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    pub fn isc2(&self) -> ISC2_R {
        ISC2_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    pub fn isc3(&self) -> ISC3_R {
        ISC3_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc0(&mut self) -> ISC0_W<EICRA_SPEC> {
        ISC0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc1(&mut self) -> ISC1_W<EICRA_SPEC> {
        ISC1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc2(&mut self) -> ISC2_W<EICRA_SPEC> {
        ISC2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - External Interrupt Sense Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn isc3(&mut self) -> ISC3_W<EICRA_SPEC> {
        ISC3_W::new(self, 6)
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
#[doc = "External Interrupt Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eicra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eicra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EICRA_SPEC;
impl crate::RegisterSpec for EICRA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eicra::R`](R) reader structure"]
impl crate::Readable for EICRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eicra::W`](W) writer structure"]
impl crate::Writable for EICRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EICRA to value 0"]
impl crate::Resettable for EICRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
