#[doc = "Register `LLDRH` reader"]
pub type R = crate::R<LLDRH_SPEC>;
#[doc = "Register `LLDRH` writer"]
pub type W = crate::W<LLDRH_SPEC>;
#[doc = "Field `LLDRH` reader - High-Byte Data Register Bits"]
pub type LLDRH_R = crate::FieldReader<LLDRH_A>;
#[doc = "High-Byte Data Register Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LLDRH_A {
    #[doc = "0: Calibration limit for fast process corner/high output voltage"]
    CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE = 0,
    #[doc = "16: Calibration limit for slow process corner/low output voltage"]
    CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE = 16,
}
impl From<LLDRH_A> for u8 {
    #[inline(always)]
    fn from(variant: LLDRH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LLDRH_A {
    type Ux = u8;
}
impl LLDRH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LLDRH_A> {
        match self.bits {
            0 => Some(LLDRH_A::CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE),
            16 => Some(LLDRH_A::CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE),
            _ => None,
        }
    }
    #[doc = "Calibration limit for fast process corner/high output voltage"]
    #[inline(always)]
    pub fn is_calibration_limit_for_fast_process_corner_high_output_voltage(&self) -> bool {
        *self == LLDRH_A::CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE
    }
    #[doc = "Calibration limit for slow process corner/low output voltage"]
    #[inline(always)]
    pub fn is_calibration_limit_for_slow_process_corner_low_output_voltage(&self) -> bool {
        *self == LLDRH_A::CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE
    }
}
#[doc = "Field `LLDRH` writer - High-Byte Data Register Bits"]
pub type LLDRH_W<'a, REG> = crate::FieldWriter<'a, REG, 5, LLDRH_A>;
impl<'a, REG> LLDRH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Calibration limit for fast process corner/high output voltage"]
    #[inline(always)]
    pub fn calibration_limit_for_fast_process_corner_high_output_voltage(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(LLDRH_A::CALIBRATION_LIMIT_FOR_FAST_PROCESS_CORNER_HIGH_OUTPUT_VOLTAGE)
    }
    #[doc = "Calibration limit for slow process corner/low output voltage"]
    #[inline(always)]
    pub fn calibration_limit_for_slow_process_corner_low_output_voltage(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(LLDRH_A::CALIBRATION_LIMIT_FOR_SLOW_PROCESS_CORNER_LOW_OUTPUT_VOLTAGE)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - High-Byte Data Register Bits"]
    #[inline(always)]
    pub fn lldrh(&self) -> LLDRH_R {
        LLDRH_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:4 - High-Byte Data Register Bits"]
    #[inline(always)]
    #[must_use]
    pub fn lldrh(&mut self) -> LLDRH_W<LLDRH_SPEC> {
        LLDRH_W::new(self, 0)
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<LLDRH_SPEC> {
        RES_W::new(self, 5)
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
#[doc = "Low Leakage Voltage Regulator Data Register (High-Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lldrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lldrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LLDRH_SPEC;
impl crate::RegisterSpec for LLDRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lldrh::R`](R) reader structure"]
impl crate::Readable for LLDRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lldrh::W`](W) writer structure"]
impl crate::Writable for LLDRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LLDRH to value 0"]
impl crate::Resettable for LLDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
