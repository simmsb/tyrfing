#[doc = "Register `OSC20MCALIBB` reader"]
pub type R = crate::R<OSC20MCALIBB_SPEC>;
#[doc = "Register `OSC20MCALIBB` writer"]
pub type W = crate::W<OSC20MCALIBB_SPEC>;
#[doc = "Field `TEMPCAL20M` reader - Oscillator temperature coefficient"]
pub type TEMPCAL20M_R = crate::FieldReader;
#[doc = "Field `TEMPCAL20M` writer - Oscillator temperature coefficient"]
pub type TEMPCAL20M_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `LOCK` reader - Lock"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Oscillator temperature coefficient"]
    #[inline(always)]
    pub fn tempcal20m(&self) -> TEMPCAL20M_R {
        TEMPCAL20M_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Oscillator temperature coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn tempcal20m(&mut self) -> TEMPCAL20M_W<OSC20MCALIBB_SPEC> {
        TEMPCAL20M_W::new(self, 0)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<OSC20MCALIBB_SPEC> {
        LOCK_W::new(self, 7)
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
#[doc = "OSC20M Calibration B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc20mcalibb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc20mcalibb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSC20MCALIBB_SPEC;
impl crate::RegisterSpec for OSC20MCALIBB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osc20mcalibb::R`](R) reader structure"]
impl crate::Readable for OSC20MCALIBB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`osc20mcalibb::W`](W) writer structure"]
impl crate::Writable for OSC20MCALIBB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC20MCALIBB to value 0"]
impl crate::Resettable for OSC20MCALIBB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
