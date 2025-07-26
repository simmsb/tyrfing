#[doc = "Register `PCIFR` reader"]
pub type R = crate::R<PCIFR_SPEC>;
#[doc = "Register `PCIFR` writer"]
pub type W = crate::W<PCIFR_SPEC>;
#[doc = "Field `PCIF` reader - Pin Change Interrupt Flags"]
pub type PCIF_R = crate::FieldReader;
#[doc = "Field `PCIF` writer - Pin Change Interrupt Flags"]
pub type PCIF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - Pin Change Interrupt Flags"]
    #[inline(always)]
    pub fn pcif(&self) -> PCIF_R {
        PCIF_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 3) & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pin Change Interrupt Flags"]
    #[inline(always)]
    #[must_use]
    pub fn pcif(&mut self) -> PCIF_W<PCIFR_SPEC> {
        PCIF_W::new(self, 0)
    }
    #[doc = "Bits 3:7 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<PCIFR_SPEC> {
        RES_W::new(self, 3)
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
#[doc = "Pin Change Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcifr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcifr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCIFR_SPEC;
impl crate::RegisterSpec for PCIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcifr::R`](R) reader structure"]
impl crate::Readable for PCIFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcifr::W`](W) writer structure"]
impl crate::Writable for PCIFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCIFR to value 0"]
impl crate::Resettable for PCIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
