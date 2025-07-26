#[doc = "Register `GIMSK` reader"]
pub type R = crate::R<GIMSK_SPEC>;
#[doc = "Register `GIMSK` writer"]
pub type W = crate::W<GIMSK_SPEC>;
#[doc = "Field `PCIE` reader - No Description."]
pub type PCIE_R = crate::BitReader;
#[doc = "Field `PCIE` writer - No Description."]
pub type PCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT` reader - External Interrupt Request 1 Enable"]
pub type INT_R = crate::FieldReader;
#[doc = "Field `INT` writer - External Interrupt Request 1 Enable"]
pub type INT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    pub fn pcie(&self) -> PCIE_R {
        PCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - External Interrupt Request 1 Enable"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 5 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn pcie(&mut self) -> PCIE_W<GIMSK_SPEC> {
        PCIE_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - External Interrupt Request 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<GIMSK_SPEC> {
        INT_W::new(self, 6)
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
#[doc = "General Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gimsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gimsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GIMSK_SPEC;
impl crate::RegisterSpec for GIMSK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gimsk::R`](R) reader structure"]
impl crate::Readable for GIMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gimsk::W`](W) writer structure"]
impl crate::Writable for GIMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GIMSK to value 0"]
impl crate::Resettable for GIMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
