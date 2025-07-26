#[doc = "Register `LLDRL` reader"]
pub type R = crate::R<LLDRL_SPEC>;
#[doc = "Register `LLDRL` writer"]
pub type W = crate::W<LLDRL_SPEC>;
#[doc = "Field `LLDRL` reader - Low-Byte Data Register Bits"]
pub type LLDRL_R = crate::FieldReader<LLDRL_A>;
#[doc = "Low-Byte Data Register Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LLDRL_A {
    #[doc = "0: Calibration limit for fast process corner/high output voltage"]
    CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE = 0,
    #[doc = "8: Calibration limit for slow process corner/low output voltage"]
    CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE = 8,
}
impl From<LLDRL_A> for u8 {
    #[inline(always)]
    fn from(variant: LLDRL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LLDRL_A {
    type Ux = u8;
}
impl LLDRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LLDRL_A> {
        match self.bits {
            0 => Some(LLDRL_A::CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE),
            8 => Some(LLDRL_A::CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE),
            _ => None,
        }
    }
    #[doc = "Calibration limit for fast process corner/high output voltage"]
    #[inline(always)]
    pub fn is_calibration_limit_for_fast_process_corner_high_output_voltage(&self) -> bool {
        *self == LLDRL_A::CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE
    }
    #[doc = "Calibration limit for slow process corner/low output voltage"]
    #[inline(always)]
    pub fn is_calibration_limit_for_slow_process_corner_low_output_voltage(&self) -> bool {
        *self == LLDRL_A::CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE
    }
}
#[doc = "Field `LLDRL` writer - Low-Byte Data Register Bits"]
pub type LLDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, LLDRL_A>;
impl<'a, REG> LLDRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Calibration limit for fast process corner/high output voltage"]
    #[inline(always)]
    pub fn calibration_limit_for_fast_process_corner_high_output_voltage(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(LLDRL_A::CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE)
    }
    #[doc = "Calibration limit for slow process corner/low output voltage"]
    #[inline(always)]
    pub fn calibration_limit_for_slow_process_corner_low_output_voltage(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(LLDRL_A::CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Low-Byte Data Register Bits"]
    #[inline(always)]
    pub fn lldrl(&self) -> LLDRL_R {
        LLDRL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Low-Byte Data Register Bits"]
    #[inline(always)]
    #[must_use]
    pub fn lldrl(&mut self) -> LLDRL_W<LLDRL_SPEC> {
        LLDRL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<LLDRL_SPEC> {
        RES_W::new(self, 4)
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
#[doc = "Low Leakage Voltage Regulator Data Register (Low-Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lldrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lldrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LLDRL_SPEC;
impl crate::RegisterSpec for LLDRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lldrl::R`](R) reader structure"]
impl crate::Readable for LLDRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lldrl::W`](W) writer structure"]
impl crate::Writable for LLDRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LLDRL to value 0"]
impl crate::Resettable for LLDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
