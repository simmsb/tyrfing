#[doc = "Register `SCOCR3HH` reader"]
pub type R = crate::R<SCOCR3HH_SPEC>;
#[doc = "Register `SCOCR3HH` writer"]
pub type W = crate::W<SCOCR3HH_SPEC>;
#[doc = "Field `SCOCR3HH` reader - Symbol Counter Output Compare Register 3 HH-Byte"]
pub type SCOCR3HH_R = crate::FieldReader;
#[doc = "Field `SCOCR3HH` writer - Symbol Counter Output Compare Register 3 HH-Byte"]
pub type SCOCR3HH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 3 HH-Byte"]
    #[inline(always)]
    pub fn scocr3hh(&self) -> SCOCR3HH_R {
        SCOCR3HH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Symbol Counter Output Compare Register 3 HH-Byte"]
    #[inline(always)]
    #[must_use]
    pub fn scocr3hh(&mut self) -> SCOCR3HH_W<SCOCR3HH_SPEC> {
        SCOCR3HH_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Symbol Counter Output Compare Register 3 HH-Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scocr3hh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scocr3hh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCOCR3HH_SPEC;
impl crate::RegisterSpec for SCOCR3HH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scocr3hh::R`](R) reader structure"]
impl crate::Readable for SCOCR3HH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scocr3hh::W`](W) writer structure"]
impl crate::Writable for SCOCR3HH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCOCR3HH to value 0"]
impl crate::Resettable for SCOCR3HH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
