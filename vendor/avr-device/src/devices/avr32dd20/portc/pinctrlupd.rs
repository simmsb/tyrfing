#[doc = "Register `PINCTRLUPD` reader"]
pub type R = crate::R<PINCTRLUPD_SPEC>;
#[doc = "Register `PINCTRLUPD` writer"]
pub type W = crate::W<PINCTRLUPD_SPEC>;
#[doc = "Field `PINCTRLUPD` reader - Pin control update mask"]
pub type PINCTRLUPD_R = crate::FieldReader;
#[doc = "Field `PINCTRLUPD` writer - Pin control update mask"]
pub type PINCTRLUPD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Pin control update mask"]
    #[inline(always)]
    pub fn pinctrlupd(&self) -> PINCTRLUPD_R {
        PINCTRLUPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pin control update mask"]
    #[inline(always)]
    #[must_use]
    pub fn pinctrlupd(&mut self) -> PINCTRLUPD_W<PINCTRLUPD_SPEC> {
        PINCTRLUPD_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Pin Control Update\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinctrlupd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinctrlupd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINCTRLUPD_SPEC;
impl crate::RegisterSpec for PINCTRLUPD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pinctrlupd::R`](R) reader structure"]
impl crate::Readable for PINCTRLUPD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinctrlupd::W`](W) writer structure"]
impl crate::Writable for PINCTRLUPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINCTRLUPD to value 0"]
impl crate::Resettable for PINCTRLUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
