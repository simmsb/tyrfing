#[doc = "Register `FTN_CTRL` reader"]
pub type R = crate::R<FTN_CTRL_SPEC>;
#[doc = "Register `FTN_CTRL` writer"]
pub type W = crate::W<FTN_CTRL_SPEC>;
#[doc = "Field `FTN_START` reader - Start Calibration Loop of Filter Tuning Network"]
pub type FTN_START_R = crate::BitReader;
#[doc = "Field `FTN_START` writer - Start Calibration Loop of Filter Tuning Network"]
pub type FTN_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - Start Calibration Loop of Filter Tuning Network"]
    #[inline(always)]
    pub fn ftn_start(&self) -> FTN_START_R {
        FTN_START_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Start Calibration Loop of Filter Tuning Network"]
    #[inline(always)]
    #[must_use]
    pub fn ftn_start(&mut self) -> FTN_START_W<FTN_CTRL_SPEC> {
        FTN_START_W::new(self, 7)
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
#[doc = "Transceiver Filter Tuning Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftn_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftn_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTN_CTRL_SPEC;
impl crate::RegisterSpec for FTN_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ftn_ctrl::R`](R) reader structure"]
impl crate::Readable for FTN_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ftn_ctrl::W`](W) writer structure"]
impl crate::Writable for FTN_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTN_CTRL to value 0"]
impl crate::Resettable for FTN_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
