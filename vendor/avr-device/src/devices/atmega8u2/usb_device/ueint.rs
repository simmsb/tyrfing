#[doc = "Register `UEINT` reader"]
pub type R = crate::R<UEINT_SPEC>;
#[doc = "Register `UEINT` writer"]
pub type W = crate::W<UEINT_SPEC>;
#[doc = "Field `EPINT` reader - Byte Count bits"]
pub type EPINT_R = crate::FieldReader;
#[doc = "Field `EPINT` writer - Byte Count bits"]
pub type EPINT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Byte Count bits"]
    #[inline(always)]
    pub fn epint(&self) -> EPINT_R {
        EPINT_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Byte Count bits"]
    #[inline(always)]
    #[must_use]
    pub fn epint(&mut self) -> EPINT_W<UEINT_SPEC> {
        EPINT_W::new(self, 0)
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
#[doc = "USB Endpoint Number Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ueint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ueint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UEINT_SPEC;
impl crate::RegisterSpec for UEINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ueint::R`](R) reader structure"]
impl crate::Readable for UEINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ueint::W`](W) writer structure"]
impl crate::Writable for UEINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UEINT to value 0"]
impl crate::Resettable for UEINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
