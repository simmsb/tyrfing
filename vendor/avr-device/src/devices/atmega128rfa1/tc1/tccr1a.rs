#[doc = "Register `TCCR1A` reader"]
pub type R = crate::R<TCCR1A_SPEC>;
#[doc = "Register `TCCR1A` writer"]
pub type W = crate::W<TCCR1A_SPEC>;
#[doc = "Field `WGM1` reader - Waveform Generation Mode"]
pub type WGM1_R = crate::FieldReader<WGM1_A>;
#[doc = "Waveform Generation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGM1_A {
    #[doc = "0: Normal mode of operation"]
    NORMAL_MODE_OF_OPERATION = 0,
    #[doc = "1: PWM, phase correct, 8-bit"]
    PWM_PHASE_CORRECT_8_BIT = 1,
    #[doc = "2: PWM, phase correct, 9-bit"]
    PWM_PHASE_CORRECT_9_BIT = 2,
    #[doc = "3: PWM, phase correct, 10-bit"]
    PWM_PHASE_CORRECT_10_BIT = 3,
}
impl From<WGM1_A> for u8 {
    #[inline(always)]
    fn from(variant: WGM1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WGM1_A {
    type Ux = u8;
}
impl WGM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WGM1_A {
        match self.bits {
            0 => WGM1_A::NORMAL_MODE_OF_OPERATION,
            1 => WGM1_A::PWM_PHASE_CORRECT_8_BIT,
            2 => WGM1_A::PWM_PHASE_CORRECT_9_BIT,
            3 => WGM1_A::PWM_PHASE_CORRECT_10_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn is_normal_mode_of_operation(&self) -> bool {
        *self == WGM1_A::NORMAL_MODE_OF_OPERATION
    }
    #[doc = "PWM, phase correct, 8-bit"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_8_bit(&self) -> bool {
        *self == WGM1_A::PWM_PHASE_CORRECT_8_BIT
    }
    #[doc = "PWM, phase correct, 9-bit"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_9_bit(&self) -> bool {
        *self == WGM1_A::PWM_PHASE_CORRECT_9_BIT
    }
    #[doc = "PWM, phase correct, 10-bit"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_10_bit(&self) -> bool {
        *self == WGM1_A::PWM_PHASE_CORRECT_10_BIT
    }
}
#[doc = "Field `WGM1` writer - Waveform Generation Mode"]
pub type WGM1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WGM1_A>;
impl<'a, REG> WGM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn normal_mode_of_operation(self) -> &'a mut crate::W<REG> {
        self.variant(WGM1_A::NORMAL_MODE_OF_OPERATION)
    }
    #[doc = "PWM, phase correct, 8-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(WGM1_A::PWM_PHASE_CORRECT_8_BIT)
    }
    #[doc = "PWM, phase correct, 9-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(WGM1_A::PWM_PHASE_CORRECT_9_BIT)
    }
    #[doc = "PWM, phase correct, 10-bit"]
    #[inline(always)]
    pub fn pwm_phase_correct_10_bit(self) -> &'a mut crate::W<REG> {
        self.variant(WGM1_A::PWM_PHASE_CORRECT_10_BIT)
    }
}
#[doc = "Field `COM1C` reader - Compare Output Mode for Channel C"]
pub type COM1C_R = crate::FieldReader<COM1C_A>;
#[doc = "Compare Output Mode for Channel C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM1C_A {
    #[doc = "0: Normal port operation, OCnA/OCnB/OCnC disconnected."]
    NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED = 0,
    #[doc = "1: Toggle OCnA/OCnB/OCnC on Compare Match."]
    TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH = 1,
    #[doc = "2: Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL = 2,
    #[doc = "3: Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL = 3,
}
impl From<COM1C_A> for u8 {
    #[inline(always)]
    fn from(variant: COM1C_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COM1C_A {
    type Ux = u8;
}
impl COM1C_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM1C_A {
        match self.bits {
            0 => COM1C_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED,
            1 => COM1C_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH,
            2 => COM1C_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL,
            3 => COM1C_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal port operation, OCnA/OCnB/OCnC disconnected."]
    #[inline(always)]
    pub fn is_normal_port_operation_ocna_ocnb_ocnc_disconnected(&self) -> bool {
        *self == COM1C_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED
    }
    #[doc = "Toggle OCnA/OCnB/OCnC on Compare Match."]
    #[inline(always)]
    pub fn is_toggle_ocna_ocnb_ocnc_on_compare_match(&self) -> bool {
        *self == COM1C_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH
    }
    #[doc = "Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    #[inline(always)]
    pub fn is_clear_ocna_ocnb_ocnc_on_compare_match_set_output_to_low_level(&self) -> bool {
        *self == COM1C_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL
    }
    #[doc = "Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    #[inline(always)]
    pub fn is_set_ocna_ocnb_ocnc_on_compare_match_set_output_to_high_level(&self) -> bool {
        *self == COM1C_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL
    }
}
#[doc = "Field `COM1C` writer - Compare Output Mode for Channel C"]
pub type COM1C_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COM1C_A>;
impl<'a, REG> COM1C_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal port operation, OCnA/OCnB/OCnC disconnected."]
    #[inline(always)]
    pub fn normal_port_operation_ocna_ocnb_ocnc_disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(COM1C_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED)
    }
    #[doc = "Toggle OCnA/OCnB/OCnC on Compare Match."]
    #[inline(always)]
    pub fn toggle_ocna_ocnb_ocnc_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM1C_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH)
    }
    #[doc = "Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    #[inline(always)]
    pub fn clear_ocna_ocnb_ocnc_on_compare_match_set_output_to_low_level(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(COM1C_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL)
    }
    #[doc = "Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    #[inline(always)]
    pub fn set_ocna_ocnb_ocnc_on_compare_match_set_output_to_high_level(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(COM1C_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL)
    }
}
#[doc = "Field `COM1B` reader - Compare Output Mode for Channel B"]
pub type COM1B_R = crate::FieldReader<COM1B_A>;
#[doc = "Compare Output Mode for Channel B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM1B_A {
    #[doc = "0: Normal port operation, OCnA/OCnB/OCnC disconnected."]
    NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED = 0,
    #[doc = "1: Toggle OCnA/OCnB/OCnC on Compare Match."]
    TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH = 1,
    #[doc = "2: Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL = 2,
    #[doc = "3: Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL = 3,
}
impl From<COM1B_A> for u8 {
    #[inline(always)]
    fn from(variant: COM1B_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COM1B_A {
    type Ux = u8;
}
impl COM1B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM1B_A {
        match self.bits {
            0 => COM1B_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED,
            1 => COM1B_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH,
            2 => COM1B_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL,
            3 => COM1B_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal port operation, OCnA/OCnB/OCnC disconnected."]
    #[inline(always)]
    pub fn is_normal_port_operation_ocna_ocnb_ocnc_disconnected(&self) -> bool {
        *self == COM1B_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED
    }
    #[doc = "Toggle OCnA/OCnB/OCnC on Compare Match."]
    #[inline(always)]
    pub fn is_toggle_ocna_ocnb_ocnc_on_compare_match(&self) -> bool {
        *self == COM1B_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH
    }
    #[doc = "Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    #[inline(always)]
    pub fn is_clear_ocna_ocnb_ocnc_on_compare_match_set_output_to_low_level(&self) -> bool {
        *self == COM1B_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL
    }
    #[doc = "Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    #[inline(always)]
    pub fn is_set_ocna_ocnb_ocnc_on_compare_match_set_output_to_high_level(&self) -> bool {
        *self == COM1B_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL
    }
}
#[doc = "Field `COM1B` writer - Compare Output Mode for Channel B"]
pub type COM1B_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COM1B_A>;
impl<'a, REG> COM1B_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal port operation, OCnA/OCnB/OCnC disconnected."]
    #[inline(always)]
    pub fn normal_port_operation_ocna_ocnb_ocnc_disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(COM1B_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED)
    }
    #[doc = "Toggle OCnA/OCnB/OCnC on Compare Match."]
    #[inline(always)]
    pub fn toggle_ocna_ocnb_ocnc_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM1B_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH)
    }
    #[doc = "Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    #[inline(always)]
    pub fn clear_ocna_ocnb_ocnc_on_compare_match_set_output_to_low_level(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(COM1B_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL)
    }
    #[doc = "Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    #[inline(always)]
    pub fn set_ocna_ocnb_ocnc_on_compare_match_set_output_to_high_level(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(COM1B_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL)
    }
}
#[doc = "Field `COM1A` reader - Compare Output Mode for Channel A"]
pub type COM1A_R = crate::FieldReader<COM1A_A>;
#[doc = "Compare Output Mode for Channel A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM1A_A {
    #[doc = "0: Normal port operation, OCnA/OCnB/OCnC disconnected."]
    NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED = 0,
    #[doc = "1: Toggle OCnA/OCnB/OCnC on Compare Match."]
    TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH = 1,
    #[doc = "2: Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL = 2,
    #[doc = "3: Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL = 3,
}
impl From<COM1A_A> for u8 {
    #[inline(always)]
    fn from(variant: COM1A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COM1A_A {
    type Ux = u8;
}
impl COM1A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM1A_A {
        match self.bits {
            0 => COM1A_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED,
            1 => COM1A_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH,
            2 => COM1A_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL,
            3 => COM1A_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal port operation, OCnA/OCnB/OCnC disconnected."]
    #[inline(always)]
    pub fn is_normal_port_operation_ocna_ocnb_ocnc_disconnected(&self) -> bool {
        *self == COM1A_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED
    }
    #[doc = "Toggle OCnA/OCnB/OCnC on Compare Match."]
    #[inline(always)]
    pub fn is_toggle_ocna_ocnb_ocnc_on_compare_match(&self) -> bool {
        *self == COM1A_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH
    }
    #[doc = "Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    #[inline(always)]
    pub fn is_clear_ocna_ocnb_ocnc_on_compare_match_set_output_to_low_level(&self) -> bool {
        *self == COM1A_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL
    }
    #[doc = "Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    #[inline(always)]
    pub fn is_set_ocna_ocnb_ocnc_on_compare_match_set_output_to_high_level(&self) -> bool {
        *self == COM1A_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL
    }
}
#[doc = "Field `COM1A` writer - Compare Output Mode for Channel A"]
pub type COM1A_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COM1A_A>;
impl<'a, REG> COM1A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal port operation, OCnA/OCnB/OCnC disconnected."]
    #[inline(always)]
    pub fn normal_port_operation_ocna_ocnb_ocnc_disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(COM1A_A::NORMAL_PORT_OPERATION_OCNA_OCNB_OCNC_DISCONNECTED)
    }
    #[doc = "Toggle OCnA/OCnB/OCnC on Compare Match."]
    #[inline(always)]
    pub fn toggle_ocna_ocnb_ocnc_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM1A_A::TOGGLE_OCNA_OCNB_OCNC_ON_COMPARE_MATCH)
    }
    #[doc = "Clear OCnA/OCnB/OCnC on Compare Match (set output to low level)."]
    #[inline(always)]
    pub fn clear_ocna_ocnb_ocnc_on_compare_match_set_output_to_low_level(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(COM1A_A::CLEAR_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_LOW_LEVEL)
    }
    #[doc = "Set OCnA/OCnB/OCnC on Compare Match (set output to high level)."]
    #[inline(always)]
    pub fn set_ocna_ocnb_ocnc_on_compare_match_set_output_to_high_level(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(COM1A_A::SET_OCNA_OCNB_OCNC_ON_COMPARE_MATCH_SET_OUTPUT_TO_HIGH_LEVEL)
    }
}
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm1(&self) -> WGM1_R {
        WGM1_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Compare Output Mode for Channel C"]
    #[inline(always)]
    pub fn com1c(&self) -> COM1C_R {
        COM1C_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Compare Output Mode for Channel B"]
    #[inline(always)]
    pub fn com1b(&self) -> COM1B_R {
        COM1B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Output Mode for Channel A"]
    #[inline(always)]
    pub fn com1a(&self) -> COM1A_R {
        COM1A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm1(&mut self) -> WGM1_W<TCCR1A_SPEC> {
        WGM1_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Compare Output Mode for Channel C"]
    #[inline(always)]
    #[must_use]
    pub fn com1c(&mut self) -> COM1C_W<TCCR1A_SPEC> {
        COM1C_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Compare Output Mode for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn com1b(&mut self) -> COM1B_W<TCCR1A_SPEC> {
        COM1B_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Compare Output Mode for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn com1a(&mut self) -> COM1A_W<TCCR1A_SPEC> {
        COM1A_W::new(self, 6)
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
#[doc = "Timer/Counter1 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR1A_SPEC;
impl crate::RegisterSpec for TCCR1A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr1a::R`](R) reader structure"]
impl crate::Readable for TCCR1A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr1a::W`](W) writer structure"]
impl crate::Writable for TCCR1A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1A to value 0"]
impl crate::Resettable for TCCR1A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
