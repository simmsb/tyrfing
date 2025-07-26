#[doc = "Register `PINCTRLSET` reader"]
pub type R = crate::R<PINCTRLSET_SPEC>;
#[doc = "Register `PINCTRLSET` writer"]
pub type W = crate::W<PINCTRLSET_SPEC>;
#[doc = "Field `PINCTRLSET` reader - Pin control set mask"]
pub type PINCTRLSET_R = crate::FieldReader;
#[doc = "Field `PINCTRLSET` writer - Pin control set mask"]
pub type PINCTRLSET_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Pin control set mask"]
    #[inline(always)]
    pub fn pinctrlset(&self) -> PINCTRLSET_R {
        PINCTRLSET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pin control set mask"]
    #[inline(always)]
    #[must_use]
    pub fn pinctrlset(&mut self) -> PINCTRLSET_W<PINCTRLSET_SPEC> {
        PINCTRLSET_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Pin Control Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinctrlset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinctrlset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINCTRLSET_SPEC;
impl crate::RegisterSpec for PINCTRLSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pinctrlset::R`](R) reader structure"]
impl crate::Readable for PINCTRLSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinctrlset::W`](W) writer structure"]
impl crate::Writable for PINCTRLSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINCTRLSET to value 0"]
impl crate::Resettable for PINCTRLSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
