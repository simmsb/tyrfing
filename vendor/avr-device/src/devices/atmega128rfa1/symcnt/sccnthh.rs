#[doc = "Register `SCCNTHH` reader"]
pub type R = crate::R<SCCNTHH_SPEC>;
#[doc = "Register `SCCNTHH` writer"]
pub type W = crate::W<SCCNTHH_SPEC>;
#[doc = "Field `SCCNTHH` reader - Symbol Counter Register HH-Byte"]
pub type SCCNTHH_R = crate::FieldReader;
#[doc = "Field `SCCNTHH` writer - Symbol Counter Register HH-Byte"]
pub type SCCNTHH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Register HH-Byte"]
    #[inline(always)]
    pub fn sccnthh(&self) -> SCCNTHH_R {
        SCCNTHH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Register HH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn sccnthh(&mut self) -> SCCNTHH_W<SCCNTHH_SPEC> {
        SCCNTHH_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Register HH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccnthh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccnthh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCCNTHH_SPEC;
impl crate::RegisterSpec for SCCNTHH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sccnthh::R`](R) reader structure"]
impl crate::Readable for SCCNTHH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sccnthh::W`](W) writer structure"]
impl crate::Writable for SCCNTHH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCNTHH to value 0"]
impl crate::Resettable for SCCNTHH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
