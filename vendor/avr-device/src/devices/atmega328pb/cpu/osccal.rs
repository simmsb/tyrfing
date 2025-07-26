#[doc = "Register `OSCCAL` reader"]
pub type R = crate::R<OSCCAL_SPEC>;
#[doc = "Register `OSCCAL` writer"]
pub type W = crate::W<OSCCAL_SPEC>;
#[doc = "Field `OSCCAL` reader - Oscillator Calibration"]
pub type OSCCAL_R = crate::FieldReader;
#[doc = "Field `OSCCAL` writer - Oscillator Calibration"]
pub type OSCCAL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Oscillator Calibration"]
    #[inline(always)]
    pub fn osccal(&self) -> OSCCAL_R {
        OSCCAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Oscillator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn osccal(&mut self) -> OSCCAL_W<OSCCAL_SPEC> {
        OSCCAL_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Oscillator Calibration Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osccal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCCAL_SPEC;
impl crate::RegisterSpec for OSCCAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osccal::R`](R) reader structure"]
impl crate::Readable for OSCCAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osccal::W`](W) writer structure"]
impl crate::Writable for OSCCAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCCAL to value 0"]
impl crate::Resettable for OSCCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
