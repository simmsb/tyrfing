#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<SINGLE_CTRLC_SPEC>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<SINGLE_CTRLC_SPEC>;
#[doc = "Field `CMP0OV` reader - Compare 0 Waveform Output Value"]
pub type CMP0OV_R = crate::BitReader;
#[doc = "Field `CMP0OV` writer - Compare 0 Waveform Output Value"]
pub type CMP0OV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1OV` reader - Compare 1 Waveform Output Value"]
pub type CMP1OV_R = crate::BitReader;
#[doc = "Field `CMP1OV` writer - Compare 1 Waveform Output Value"]
pub type CMP1OV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2OV` reader - Compare 2 Waveform Output Value"]
pub type CMP2OV_R = crate::BitReader;
#[doc = "Field `CMP2OV` writer - Compare 2 Waveform Output Value"]
pub type CMP2OV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Compare 0 Waveform Output Value"]
    #[inline(always)]
    pub fn cmp0ov(&self) -> CMP0OV_R {
        CMP0OV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 1 Waveform Output Value"]
    #[inline(always)]
    pub fn cmp1ov(&self) -> CMP1OV_R {
        CMP1OV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 2 Waveform Output Value"]
    #[inline(always)]
    pub fn cmp2ov(&self) -> CMP2OV_R {
        CMP2OV_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare 0 Waveform Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0ov(&mut self) -> CMP0OV_W<SINGLE_CTRLC_SPEC> {
        CMP0OV_W::new(self, 0)
    }
    #[doc = "Bit 1 - Compare 1 Waveform Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1ov(&mut self) -> CMP1OV_W<SINGLE_CTRLC_SPEC> {
        CMP1OV_W::new(self, 1)
    }
    #[doc = "Bit 2 - Compare 2 Waveform Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2ov(&mut self) -> CMP2OV_W<SINGLE_CTRLC_SPEC> {
        CMP2OV_W::new(self, 2)
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
#[doc = "Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_ctrlc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_ctrlc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLE_CTRLC_SPEC;
impl crate::RegisterSpec for SINGLE_CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`single_ctrlc::R`](R) reader structure"]
impl crate::Readable for SINGLE_CTRLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`single_ctrlc::W`](W) writer structure"]
impl crate::Writable for SINGLE_CTRLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for SINGLE_CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
