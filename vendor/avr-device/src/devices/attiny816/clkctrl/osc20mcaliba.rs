#[doc = "Register `OSC20MCALIBA` reader"]
pub type R = crate::R<OSC20MCALIBA_SPEC>;
#[doc = "Register `OSC20MCALIBA` writer"]
pub type W = crate::W<OSC20MCALIBA_SPEC>;
#[doc = "Field `CAL20M` reader - Calibration"]
pub type CAL20M_R = crate::FieldReader;
#[doc = "Field `CAL20M` writer - Calibration"]
pub type CAL20M_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Calibration"]
    #[inline(always)]
    pub fn cal20m(&self) -> CAL20M_R {
        CAL20M_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cal20m(&mut self) -> CAL20M_W<OSC20MCALIBA_SPEC> {
        CAL20M_W::new(self, 0)
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
#[doc = "OSC20M Calibration A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc20mcaliba::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc20mcaliba::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSC20MCALIBA_SPEC;
impl crate::RegisterSpec for OSC20MCALIBA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osc20mcaliba::R`](R) reader structure"]
impl crate::Readable for OSC20MCALIBA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osc20mcaliba::W`](W) writer structure"]
impl crate::Writable for OSC20MCALIBA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC20MCALIBA to value 0"]
impl crate::Resettable for OSC20MCALIBA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
