#[doc = "Register `TCCR0A` reader"]
pub type R = crate::R<TCCR0A_SPEC>;
#[doc = "Register `TCCR0A` writer"]
pub type W = crate::W<TCCR0A_SPEC>;
#[doc = "Field `WGM0` reader - Waveform Generation Mode"]
pub type WGM0_R = crate::FieldReader<WGM0_A>;
#[doc = "Waveform Generation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGM0_A {
    #[doc = "0: Normal mode of operation"]
    NORMAL_MODE_OF_OPERATION = 0,
    #[doc = "1: PWM, phase correct, TOP=0xFF"]
    PWM_PHASE_CORRECT_TOP_0XFF = 1,
    #[doc = "2: CTC, TOP = OCRA"]
    CTC_TOP_OCRA = 2,
    #[doc = "3: Fast PWM, TOP=0xFF"]
    FAST_PWM_TOP_0XFF = 3,
}
impl From<WGM0_A> for u8 {
    #[inline(always)]
    fn from(variant: WGM0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WGM0_A {
    type Ux = u8;
}
impl WGM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WGM0_A {
        match self.bits {
            0 => WGM0_A::NORMAL_MODE_OF_OPERATION,
            1 => WGM0_A::PWM_PHASE_CORRECT_TOP_0XFF,
            2 => WGM0_A::CTC_TOP_OCRA,
            3 => WGM0_A::FAST_PWM_TOP_0XFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn is_normal_mode_of_operation(&self) -> bool {
        *self == WGM0_A::NORMAL_MODE_OF_OPERATION
    }
    #[doc = "PWM, phase correct, TOP=0xFF"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_top_0xff(&self) -> bool {
        *self == WGM0_A::PWM_PHASE_CORRECT_TOP_0XFF
    }
    #[doc = "CTC, TOP = OCRA"]
    #[inline(always)]
    pub fn is_ctc_top_ocra(&self) -> bool {
        *self == WGM0_A::CTC_TOP_OCRA
    }
    #[doc = "Fast PWM, TOP=0xFF"]
    #[inline(always)]
    pub fn is_fast_pwm_top_0xff(&self) -> bool {
        *self == WGM0_A::FAST_PWM_TOP_0XFF
    }
}
#[doc = "Field `WGM0` writer - Waveform Generation Mode"]
pub type WGM0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WGM0_A>;
impl<'a, REG> WGM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn normal_mode_of_operation(self) -> &'a mut crate::W<REG> {
        self.variant(WGM0_A::NORMAL_MODE_OF_OPERATION)
    }
    #[doc = "PWM, phase correct, TOP=0xFF"]
    #[inline(always)]
    pub fn pwm_phase_correct_top_0xff(self) -> &'a mut crate::W<REG> {
        self.variant(WGM0_A::PWM_PHASE_CORRECT_TOP_0XFF)
    }
    #[doc = "CTC, TOP = OCRA"]
    #[inline(always)]
    pub fn ctc_top_ocra(self) -> &'a mut crate::W<REG> {
        self.variant(WGM0_A::CTC_TOP_OCRA)
    }
    #[doc = "Fast PWM, TOP=0xFF"]
    #[inline(always)]
    pub fn fast_pwm_top_0xff(self) -> &'a mut crate::W<REG> {
        self.variant(WGM0_A::FAST_PWM_TOP_0XFF)
    }
}
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `COM0B` reader - Compare Match Output B Mode"]
pub type COM0B_R = crate::FieldReader<COM0B_A>;
#[doc = "Compare Match Output B Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM0B_A {
    #[doc = "0: Normal port operation, OC0B disconnected"]
    NORMAL_PORT_OPERATION_OC0B_DISCONNECTED = 0,
    #[doc = "1: Toggle OC0B on Compare Match"]
    TOGGLE_OC0B_ON_COMPARE_MATCH = 1,
    #[doc = "2: Clear OC0B on Compare Match"]
    CLEAR_OC0B_ON_COMPARE_MATCH = 2,
    #[doc = "3: Set OC0B on Compare Match"]
    SET_OC0B_ON_COMPARE_MATCH = 3,
}
impl From<COM0B_A> for u8 {
    #[inline(always)]
    fn from(variant: COM0B_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COM0B_A {
    type Ux = u8;
}
impl COM0B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM0B_A {
        match self.bits {
            0 => COM0B_A::NORMAL_PORT_OPERATION_OC0B_DISCONNECTED,
            1 => COM0B_A::TOGGLE_OC0B_ON_COMPARE_MATCH,
            2 => COM0B_A::CLEAR_OC0B_ON_COMPARE_MATCH,
            3 => COM0B_A::SET_OC0B_ON_COMPARE_MATCH,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal port operation, OC0B disconnected"]
    #[inline(always)]
    pub fn is_normal_port_operation_oc0b_disconnected(&self) -> bool {
        *self == COM0B_A::NORMAL_PORT_OPERATION_OC0B_DISCONNECTED
    }
    #[doc = "Toggle OC0B on Compare Match"]
    #[inline(always)]
    pub fn is_toggle_oc0b_on_compare_match(&self) -> bool {
        *self == COM0B_A::TOGGLE_OC0B_ON_COMPARE_MATCH
    }
    #[doc = "Clear OC0B on Compare Match"]
    #[inline(always)]
    pub fn is_clear_oc0b_on_compare_match(&self) -> bool {
        *self == COM0B_A::CLEAR_OC0B_ON_COMPARE_MATCH
    }
    #[doc = "Set OC0B on Compare Match"]
    #[inline(always)]
    pub fn is_set_oc0b_on_compare_match(&self) -> bool {
        *self == COM0B_A::SET_OC0B_ON_COMPARE_MATCH
    }
}
#[doc = "Field `COM0B` writer - Compare Match Output B Mode"]
pub type COM0B_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COM0B_A>;
impl<'a, REG> COM0B_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal port operation, OC0B disconnected"]
    #[inline(always)]
    pub fn normal_port_operation_oc0b_disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(COM0B_A::NORMAL_PORT_OPERATION_OC0B_DISCONNECTED)
    }
    #[doc = "Toggle OC0B on Compare Match"]
    #[inline(always)]
    pub fn toggle_oc0b_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM0B_A::TOGGLE_OC0B_ON_COMPARE_MATCH)
    }
    #[doc = "Clear OC0B on Compare Match"]
    #[inline(always)]
    pub fn clear_oc0b_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM0B_A::CLEAR_OC0B_ON_COMPARE_MATCH)
    }
    #[doc = "Set OC0B on Compare Match"]
    #[inline(always)]
    pub fn set_oc0b_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM0B_A::SET_OC0B_ON_COMPARE_MATCH)
    }
}
#[doc = "Field `COM0A` reader - Compare Match Output A Mode"]
pub type COM0A_R = crate::FieldReader<COM0A_A>;
#[doc = "Compare Match Output A Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM0A_A {
    #[doc = "0: Normal port operation, OC0A disconnected"]
    NORMAL_PORT_OPERATION_OC0A_DISCONNECTED = 0,
    #[doc = "1: Toggle OC0A on Compare Match"]
    TOGGLE_OC0A_ON_COMPARE_MATCH = 1,
    #[doc = "2: Clear OC0A on Compare Match"]
    CLEAR_OC0A_ON_COMPARE_MATCH = 2,
    #[doc = "3: Set OC0A on Compare Match"]
    SET_OC0A_ON_COMPARE_MATCH = 3,
}
impl From<COM0A_A> for u8 {
    #[inline(always)]
    fn from(variant: COM0A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COM0A_A {
    type Ux = u8;
}
impl COM0A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM0A_A {
        match self.bits {
            0 => COM0A_A::NORMAL_PORT_OPERATION_OC0A_DISCONNECTED,
            1 => COM0A_A::TOGGLE_OC0A_ON_COMPARE_MATCH,
            2 => COM0A_A::CLEAR_OC0A_ON_COMPARE_MATCH,
            3 => COM0A_A::SET_OC0A_ON_COMPARE_MATCH,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal port operation, OC0A disconnected"]
    #[inline(always)]
    pub fn is_normal_port_operation_oc0a_disconnected(&self) -> bool {
        *self == COM0A_A::NORMAL_PORT_OPERATION_OC0A_DISCONNECTED
    }
    #[doc = "Toggle OC0A on Compare Match"]
    #[inline(always)]
    pub fn is_toggle_oc0a_on_compare_match(&self) -> bool {
        *self == COM0A_A::TOGGLE_OC0A_ON_COMPARE_MATCH
    }
    #[doc = "Clear OC0A on Compare Match"]
    #[inline(always)]
    pub fn is_clear_oc0a_on_compare_match(&self) -> bool {
        *self == COM0A_A::CLEAR_OC0A_ON_COMPARE_MATCH
    }
    #[doc = "Set OC0A on Compare Match"]
    #[inline(always)]
    pub fn is_set_oc0a_on_compare_match(&self) -> bool {
        *self == COM0A_A::SET_OC0A_ON_COMPARE_MATCH
    }
}
#[doc = "Field `COM0A` writer - Compare Match Output A Mode"]
pub type COM0A_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COM0A_A>;
impl<'a, REG> COM0A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal port operation, OC0A disconnected"]
    #[inline(always)]
    pub fn normal_port_operation_oc0a_disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(COM0A_A::NORMAL_PORT_OPERATION_OC0A_DISCONNECTED)
    }
    #[doc = "Toggle OC0A on Compare Match"]
    #[inline(always)]
    pub fn toggle_oc0a_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM0A_A::TOGGLE_OC0A_ON_COMPARE_MATCH)
    }
    #[doc = "Clear OC0A on Compare Match"]
    #[inline(always)]
    pub fn clear_oc0a_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM0A_A::CLEAR_OC0A_ON_COMPARE_MATCH)
    }
    #[doc = "Set OC0A on Compare Match"]
    #[inline(always)]
    pub fn set_oc0a_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM0A_A::SET_OC0A_ON_COMPARE_MATCH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm0(&self) -> WGM0_R {
        WGM0_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Compare Match Output B Mode"]
    #[inline(always)]
    pub fn com0b(&self) -> COM0B_R {
        COM0B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Match Output A Mode"]
    #[inline(always)]
    pub fn com0a(&self) -> COM0A_R {
        COM0A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm0(&mut self) -> WGM0_W<TCCR0A_SPEC> {
        WGM0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<TCCR0A_SPEC> {
        RES_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Compare Match Output B Mode"]
    #[inline(always)]
    #[must_use]
    pub fn com0b(&mut self) -> COM0B_W<TCCR0A_SPEC> {
        COM0B_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Compare Match Output A Mode"]
    #[inline(always)]
    #[must_use]
    pub fn com0a(&mut self) -> COM0A_W<TCCR0A_SPEC> {
        COM0A_W::new(self, 6)
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
#[doc = "Timer/Counter0 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr0a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr0a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR0A_SPEC;
impl crate::RegisterSpec for TCCR0A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr0a::R`](R) reader structure"]
impl crate::Readable for TCCR0A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr0a::W`](W) writer structure"]
impl crate::Writable for TCCR0A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR0A to value 0"]
impl crate::Resettable for TCCR0A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
