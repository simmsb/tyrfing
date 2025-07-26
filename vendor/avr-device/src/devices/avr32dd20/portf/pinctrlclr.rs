#[doc = "Register `PINCTRLCLR` reader"]
pub type R = crate::R<PINCTRLCLR_SPEC>;
#[doc = "Register `PINCTRLCLR` writer"]
pub type W = crate::W<PINCTRLCLR_SPEC>;
#[doc = "Field `PINCTRLCLR` reader - Pin control clear mask"]
pub type PINCTRLCLR_R = crate::FieldReader;
#[doc = "Field `PINCTRLCLR` writer - Pin control clear mask"]
pub type PINCTRLCLR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Pin control clear mask"]
    #[inline(always)]
    pub fn pinctrlclr(&self) -> PINCTRLCLR_R {
        PINCTRLCLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pin control clear mask"]
    #[inline(always)]
    #[must_use]
    pub fn pinctrlclr(&mut self) -> PINCTRLCLR_W<PINCTRLCLR_SPEC> {
        PINCTRLCLR_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Pin Control Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinctrlclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinctrlclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINCTRLCLR_SPEC;
impl crate::RegisterSpec for PINCTRLCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pinctrlclr::R`](R) reader structure"]
impl crate::Readable for PINCTRLCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinctrlclr::W`](W) writer structure"]
impl crate::Writable for PINCTRLCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINCTRLCLR to value 0"]
impl crate::Resettable for PINCTRLCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
