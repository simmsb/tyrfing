#[doc = "Register `STROBE` writer"]
pub type W = crate::W<STROBE_SPEC>;
#[doc = "Software event on channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STROBE0_AW {
    #[doc = "1: No Description."]
    EV_STROBE_CH0 = 1,
    #[doc = "2: No Description."]
    EV_STROBE_CH1 = 2,
    #[doc = "4: No Description."]
    EV_STROBE_CH2 = 4,
    #[doc = "8: No Description."]
    EV_STROBE_CH3 = 8,
    #[doc = "16: No Description."]
    EV_STROBE_CH4 = 16,
    #[doc = "32: No Description."]
    EV_STROBE_CH5 = 32,
    #[doc = "64: No Description."]
    EV_STROBE_CH6 = 64,
    #[doc = "128: No Description."]
    EV_STROBE_CH7 = 128,
}
impl From<STROBE0_AW> for u8 {
    #[inline(always)]
    fn from(variant: STROBE0_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STROBE0_AW {
    type Ux = u8;
}
#[doc = "Field `STROBE0` writer - Software event on channels"]
pub type STROBE0_W<'a, REG> = crate::FieldWriter<'a, REG, 8, STROBE0_AW>;
impl<'a, REG> STROBE0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch0(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE0_AW::EV_STROBE_CH0)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE0_AW::EV_STROBE_CH1)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE0_AW::EV_STROBE_CH2)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE0_AW::EV_STROBE_CH3)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE0_AW::EV_STROBE_CH4)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch5(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE0_AW::EV_STROBE_CH5)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch6(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE0_AW::EV_STROBE_CH6)
    }
    #[doc = "No Description."]
    #[inline(always)]
    pub fn ev_strobe_ch7(self) -> &'a mut crate::W<REG> {
        self.variant(STROBE0_AW::EV_STROBE_CH7)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software event on channels"]
    #[inline(always)]
    #[must_use]
    pub fn strobe0(&mut self) -> STROBE0_W<STROBE_SPEC> {
        STROBE0_W::new(self, 0)
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
#[doc = "Channel Strobe\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`strobe::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STROBE_SPEC;
impl crate::RegisterSpec for STROBE_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`strobe::W`](W) writer structure"]
impl crate::Writable for STROBE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STROBE to value 0"]
impl crate::Resettable for STROBE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
