#[doc = "Register `SCOCR2HH` reader"]
pub type R = crate::R<SCOCR2HH_SPEC>;
#[doc = "Register `SCOCR2HH` writer"]
pub type W = crate::W<SCOCR2HH_SPEC>;
#[doc = "Field `SCOCR2HH` reader - Symbol Counter Output Compare Register 2 HH-Byte"]
pub type SCOCR2HH_R = crate::FieldReader;
#[doc = "Field `SCOCR2HH` writer - Symbol Counter Output Compare Register 2 HH-Byte"]
pub type SCOCR2HH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 2 HH-Byte"]
    #[inline(always)]
    pub fn scocr2hh(&self) -> SCOCR2HH_R {
        SCOCR2HH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 2 HH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scocr2hh(&mut self) -> SCOCR2HH_W<SCOCR2HH_SPEC> {
        SCOCR2HH_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Output Compare Register 2 HH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr2hh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr2hh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCOCR2HH_SPEC;
impl crate::RegisterSpec for SCOCR2HH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scocr2hh::R`](R) reader structure"]
impl crate::Readable for SCOCR2HH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scocr2hh::W`](W) writer structure"]
impl crate::Writable for SCOCR2HH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCOCR2HH to value 0"]
impl crate::Resettable for SCOCR2HH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
