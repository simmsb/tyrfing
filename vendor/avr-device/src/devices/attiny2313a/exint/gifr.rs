#[doc = "Register `GIFR` reader"]
pub type R = crate::R<GIFR_SPEC>;
#[doc = "Register `GIFR` writer"]
pub type W = crate::W<GIFR_SPEC>;
#[doc = "Field `PCIF` reader - Pin Change Interrupt Flags"]
pub type PCIF_R = crate::FieldReader;
#[doc = "Field `PCIF` writer - Pin Change Interrupt Flags"]
pub type PCIF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `INTF` reader - External Interrupt Flags"]
pub type INTF_R = crate::FieldReader;
#[doc = "Field `INTF` writer - External Interrupt Flags"]
pub type INTF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bits 3:5 - Pin Change Interrupt Flags"]
    #[inline(always)]
    pub fn pcif(&self) -> PCIF_R {
        PCIF_R::new((self.bits >> 3) & 7)
    }
    #[doc = "Bits 6:7 - External Interrupt Flags"]
    #[inline(always)]
    pub fn intf(&self) -> INTF_R {
        INTF_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 3:5 - Pin Change Interrupt Flags"]
    #[inline(always)]
    #[must_use]
    pub fn pcif(&mut self) -> PCIF_W<GIFR_SPEC> {
        PCIF_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - External Interrupt Flags"]
    #[inline(always)]
    #[must_use]
    pub fn intf(&mut self) -> INTF_W<GIFR_SPEC> {
        INTF_W::new(self, 6)
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
#[doc = "General Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gifr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gifr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GIFR_SPEC;
impl crate::RegisterSpec for GIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gifr::R`](R) reader structure"]
impl crate::Readable for GIFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gifr::W`](W) writer structure"]
impl crate::Writable for GIFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GIFR to value 0"]
impl crate::Resettable for GIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
