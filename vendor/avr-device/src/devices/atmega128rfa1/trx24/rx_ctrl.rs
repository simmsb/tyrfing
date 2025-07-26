#[doc = "Register `RX_CTRL` reader"]
pub type R = crate::R<RX_CTRL_SPEC>;
#[doc = "Register `RX_CTRL` writer"]
pub type W = crate::W<RX_CTRL_SPEC>;
#[doc = "Field `PDT_THRES` reader - Receiver Sensitivity Control"]
pub type PDT_THRES_R = crate::FieldReader<PDT_THRES_A>;
#[doc = "Receiver Sensitivity Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PDT_THRES_A {
    #[doc = "3: Recommended correlator threshold for Antenna Diversity operation"]
    PDT_THRES_ANT_DIV_ON = 3,
    #[doc = "7: Reset value, to be used if Antenna Diversity algorithm is disabled"]
    PDT_THRES_ANT_DIV_OFF = 7,
}
impl From<PDT_THRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PDT_THRES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PDT_THRES_A {
    type Ux = u8;
}
impl PDT_THRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PDT_THRES_A> {
        match self.bits {
            3 => Some(PDT_THRES_A::PDT_THRES_ANT_DIV_ON),
            7 => Some(PDT_THRES_A::PDT_THRES_ANT_DIV_OFF),
            _ => None,
        }
    }
    #[doc = "Recommended correlator threshold for Antenna Diversity operation"]
    #[inline(always)]
    pub fn is_pdt_thres_ant_div_on(&self) -> bool {
        *self == PDT_THRES_A::PDT_THRES_ANT_DIV_ON
    }
    #[doc = "Reset value, to be used if Antenna Diversity algorithm is disabled"]
    #[inline(always)]
    pub fn is_pdt_thres_ant_div_off(&self) -> bool {
        *self == PDT_THRES_A::PDT_THRES_ANT_DIV_OFF
    }
}
#[doc = "Field `PDT_THRES` writer - Receiver Sensitivity Control"]
pub type PDT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PDT_THRES_A>;
impl<'a, REG> PDT_THRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Recommended correlator threshold for Antenna Diversity operation"]
    #[inline(always)]
    pub fn pdt_thres_ant_div_on(self) -> &'a mut crate::W<REG> {
        self.variant(PDT_THRES_A::PDT_THRES_ANT_DIV_ON)
    }
    #[doc = "Reset value, to be used if Antenna Diversity algorithm is disabled"]
    #[inline(always)]
    pub fn pdt_thres_ant_div_off(self) -> &'a mut crate::W<REG> {
        self.variant(PDT_THRES_A::PDT_THRES_ANT_DIV_OFF)
    }
}
impl R {
    #[doc = "Bits 0:3 - Receiver Sensitivity Control"]
    #[inline(always)]
    pub fn pdt_thres(&self) -> PDT_THRES_R {
        PDT_THRES_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receiver Sensitivity Control"]
    #[inline(always)]
    #[must_use]
    pub fn pdt_thres(&mut self) -> PDT_THRES_W<RX_CTRL_SPEC> {
        PDT_THRES_W::new(self, 0)
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
#[doc = "Transceiver Receive Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CTRL_SPEC;
impl crate::RegisterSpec for RX_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rx_ctrl::R`](R) reader structure"]
impl crate::Readable for RX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_ctrl::W`](W) writer structure"]
impl crate::Writable for RX_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CTRL to value 0"]
impl crate::Resettable for RX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
