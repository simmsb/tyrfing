#[doc = "Register `SWEVENTA` reader"]
pub type R = crate::R<SWEVENTA_SPEC>;
#[doc = "Register `SWEVENTA` writer"]
pub type W = crate::W<SWEVENTA_SPEC>;
#[doc = "Software event on channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWEVENTA_AW {
    #[doc = "1: Software event on channel 0"]
    CH0 = 1,
    #[doc = "2: Software event on channel 1"]
    CH1 = 2,
    #[doc = "4: Software event on channel 2"]
    CH2 = 4,
    #[doc = "8: Software event on channel 3"]
    CH3 = 8,
    #[doc = "16: Software event on channel 4"]
    CH4 = 16,
    #[doc = "32: Software event on channel 5"]
    CH5 = 32,
    #[doc = "64: Software event on channel 6"]
    CH6 = 64,
    #[doc = "128: Software event on channel 7"]
    CH7 = 128,
}
impl From<SWEVENTA_AW> for u8 {
    #[inline(always)]
    fn from(variant: SWEVENTA_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWEVENTA_AW {
    type Ux = u8;
}
#[doc = "Field `SWEVENTA` writer - Software event on channel select"]
pub type SWEVENTA_W<'a, REG> = crate::FieldWriter<'a, REG, 8, SWEVENTA_AW>;
impl<'a, REG> SWEVENTA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software event on channel 0"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut crate::W<REG> {
        self.variant(SWEVENTA_AW::CH0)
    }
    #[doc = "Software event on channel 1"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut crate::W<REG> {
        self.variant(SWEVENTA_AW::CH1)
    }
    #[doc = "Software event on channel 2"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut crate::W<REG> {
        self.variant(SWEVENTA_AW::CH2)
    }
    #[doc = "Software event on channel 3"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut crate::W<REG> {
        self.variant(SWEVENTA_AW::CH3)
    }
    #[doc = "Software event on channel 4"]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut crate::W<REG> {
        self.variant(SWEVENTA_AW::CH4)
    }
    #[doc = "Software event on channel 5"]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut crate::W<REG> {
        self.variant(SWEVENTA_AW::CH5)
    }
    #[doc = "Software event on channel 6"]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut crate::W<REG> {
        self.variant(SWEVENTA_AW::CH6)
    }
    #[doc = "Software event on channel 7"]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut crate::W<REG> {
        self.variant(SWEVENTA_AW::CH7)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software event on channel select"]
    #[inline(always)]
    #[must_use]
    pub fn sweventa(&mut self) -> SWEVENTA_W<SWEVENTA_SPEC> {
        SWEVENTA_W::new(self, 0)
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
#[doc = "Software Event A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sweventa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sweventa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWEVENTA_SPEC;
impl crate::RegisterSpec for SWEVENTA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sweventa::R`](R) reader structure"]
impl crate::Readable for SWEVENTA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sweventa::W`](W) writer structure"]
impl crate::Writable for SWEVENTA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWEVENTA to value 0"]
impl crate::Resettable for SWEVENTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
