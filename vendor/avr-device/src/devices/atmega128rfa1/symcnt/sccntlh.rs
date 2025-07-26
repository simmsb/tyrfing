#[doc = "Register `SCCNTLH` reader"]
pub type R = crate::R<SCCNTLH_SPEC>;
#[doc = "Register `SCCNTLH` writer"]
pub type W = crate::W<SCCNTLH_SPEC>;
#[doc = "Field `SCCNTLH` reader - Symbol Counter Register LH-Byte"]
pub type SCCNTLH_R = crate::FieldReader;
#[doc = "Field `SCCNTLH` writer - Symbol Counter Register LH-Byte"]
pub type SCCNTLH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Register LH-Byte"]
    #[inline(always)]
    pub fn sccntlh(&self) -> SCCNTLH_R {
        SCCNTLH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Register LH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn sccntlh(&mut self) -> SCCNTLH_W<SCCNTLH_SPEC> {
        SCCNTLH_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Register LH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sccntlh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sccntlh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCCNTLH_SPEC;
impl crate::RegisterSpec for SCCNTLH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sccntlh::R`](R) reader structure"]
impl crate::Readable for SCCNTLH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sccntlh::W`](W) writer structure"]
impl crate::Writable for SCCNTLH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCNTLH to value 0"]
impl crate::Resettable for SCCNTLH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
