#[doc = "Register `TCCR2A` reader"]
pub type R = crate::R<TCCR2A_SPEC>;
#[doc = "Register `TCCR2A` writer"]
pub type W = crate::W<TCCR2A_SPEC>;
#[doc = "Field `WGM2` reader - Waveform Generation Mode"]
pub type WGM2_R = crate::FieldReader<WGM2_A>;
#[doc = "Waveform Generation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WGM2_A {
    #[doc = "0: Normal mode of operation"]
    NORMAL_MODE_OF_OPERATION = 0,
    #[doc = "1: PWM, phase correct, TOP=0xFF"]
    PWM_PHASE_CORRECT_TOP_0XFF = 1,
    #[doc = "2: CTC, TOP = OCRA"]
    CTC_TOP_OCRA = 2,
    #[doc = "3: Fast PWM, TOP=0xFF"]
    FAST_PWM_TOP_0XFF = 3,
}
impl From<WGM2_A> for u8 {
    #[inline(always)]
    fn from(variant: WGM2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WGM2_A {
    type Ux = u8;
}
impl WGM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WGM2_A {
        match self.bits {
            0 => WGM2_A::NORMAL_MODE_OF_OPERATION,
            1 => WGM2_A::PWM_PHASE_CORRECT_TOP_0XFF,
            2 => WGM2_A::CTC_TOP_OCRA,
            3 => WGM2_A::FAST_PWM_TOP_0XFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn is_normal_mode_of_operation(&self) -> bool {
        *self == WGM2_A::NORMAL_MODE_OF_OPERATION
    }
    #[doc = "PWM, phase correct, TOP=0xFF"]
    #[inline(always)]
    pub fn is_pwm_phase_correct_top_0xff(&self) -> bool {
        *self == WGM2_A::PWM_PHASE_CORRECT_TOP_0XFF
    }
    #[doc = "CTC, TOP = OCRA"]
    #[inline(always)]
    pub fn is_ctc_top_ocra(&self) -> bool {
        *self == WGM2_A::CTC_TOP_OCRA
    }
    #[doc = "Fast PWM, TOP=0xFF"]
    #[inline(always)]
    pub fn is_fast_pwm_top_0xff(&self) -> bool {
        *self == WGM2_A::FAST_PWM_TOP_0XFF
    }
}
#[doc = "Field `WGM2` writer - Waveform Generation Mode"]
pub type WGM2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, WGM2_A>;
impl<'a, REG> WGM2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode of operation"]
    #[inline(always)]
    pub fn normal_mode_of_operation(self) -> &'a mut crate::W<REG> {
        self.variant(WGM2_A::NORMAL_MODE_OF_OPERATION)
    }
    #[doc = "PWM, phase correct, TOP=0xFF"]
    #[inline(always)]
    pub fn pwm_phase_correct_top_0xff(self) -> &'a mut crate::W<REG> {
        self.variant(WGM2_A::PWM_PHASE_CORRECT_TOP_0XFF)
    }
    #[doc = "CTC, TOP = OCRA"]
    #[inline(always)]
    pub fn ctc_top_ocra(self) -> &'a mut crate::W<REG> {
        self.variant(WGM2_A::CTC_TOP_OCRA)
    }
    #[doc = "Fast PWM, TOP=0xFF"]
    #[inline(always)]
    pub fn fast_pwm_top_0xff(self) -> &'a mut crate::W<REG> {
        self.variant(WGM2_A::FAST_PWM_TOP_0XFF)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `COM2B` reader - Compare Match Output B Mode"]
pub type COM2B_R = crate::FieldReader<COM2B_A>;
#[doc = "Compare Match Output B Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM2B_A {
    #[doc = "0: Normal port operation, OC2B disconnected"]
    NORMAL_PORT_OPERATION_OC2B_DISCONNECTED = 0,
    #[doc = "1: Toggle OC2B on Compare Match"]
    TOGGLE_OC2B_ON_COMPARE_MATCH = 1,
    #[doc = "2: Clear OC2B on Compare Match"]
    CLEAR_OC2B_ON_COMPARE_MATCH = 2,
    #[doc = "3: Set OC2B on Compare Match"]
    SET_OC2B_ON_COMPARE_MATCH = 3,
}
impl From<COM2B_A> for u8 {
    #[inline(always)]
    fn from(variant: COM2B_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COM2B_A {
    type Ux = u8;
}
impl COM2B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM2B_A {
        match self.bits {
            0 => COM2B_A::NORMAL_PORT_OPERATION_OC2B_DISCONNECTED,
            1 => COM2B_A::TOGGLE_OC2B_ON_COMPARE_MATCH,
            2 => COM2B_A::CLEAR_OC2B_ON_COMPARE_MATCH,
            3 => COM2B_A::SET_OC2B_ON_COMPARE_MATCH,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal port operation, OC2B disconnected"]
    #[inline(always)]
    pub fn is_normal_port_operation_oc2b_disconnected(&self) -> bool {
        *self == COM2B_A::NORMAL_PORT_OPERATION_OC2B_DISCONNECTED
    }
    #[doc = "Toggle OC2B on Compare Match"]
    #[inline(always)]
    pub fn is_toggle_oc2b_on_compare_match(&self) -> bool {
        *self == COM2B_A::TOGGLE_OC2B_ON_COMPARE_MATCH
    }
    #[doc = "Clear OC2B on Compare Match"]
    #[inline(always)]
    pub fn is_clear_oc2b_on_compare_match(&self) -> bool {
        *self == COM2B_A::CLEAR_OC2B_ON_COMPARE_MATCH
    }
    #[doc = "Set OC2B on Compare Match"]
    #[inline(always)]
    pub fn is_set_oc2b_on_compare_match(&self) -> bool {
        *self == COM2B_A::SET_OC2B_ON_COMPARE_MATCH
    }
}
#[doc = "Field `COM2B` writer - Compare Match Output B Mode"]
pub type COM2B_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COM2B_A>;
impl<'a, REG> COM2B_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal port operation, OC2B disconnected"]
    #[inline(always)]
    pub fn normal_port_operation_oc2b_disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(COM2B_A::NORMAL_PORT_OPERATION_OC2B_DISCONNECTED)
    }
    #[doc = "Toggle OC2B on Compare Match"]
    #[inline(always)]
    pub fn toggle_oc2b_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM2B_A::TOGGLE_OC2B_ON_COMPARE_MATCH)
    }
    #[doc = "Clear OC2B on Compare Match"]
    #[inline(always)]
    pub fn clear_oc2b_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM2B_A::CLEAR_OC2B_ON_COMPARE_MATCH)
    }
    #[doc = "Set OC2B on Compare Match"]
    #[inline(always)]
    pub fn set_oc2b_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM2B_A::SET_OC2B_ON_COMPARE_MATCH)
    }
}
#[doc = "Field `COM2A` reader - Compare Match Output A Mode"]
pub type COM2A_R = crate::FieldReader<COM2A_A>;
#[doc = "Compare Match Output A Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COM2A_A {
    #[doc = "0: Normal port operation, OC2A disconnected"]
    NORMAL_PORT_OPERATION_OC2A_DISCONNECTED = 0,
    #[doc = "1: Toggle OC2A on Compare Match"]
    TOGGLE_OC2A_ON_COMPARE_MATCH = 1,
    #[doc = "2: Clear OC2A on Compare Match"]
    CLEAR_OC2A_ON_COMPARE_MATCH = 2,
    #[doc = "3: Set OC2A on Compare Match"]
    SET_OC2A_ON_COMPARE_MATCH = 3,
}
impl From<COM2A_A> for u8 {
    #[inline(always)]
    fn from(variant: COM2A_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COM2A_A {
    type Ux = u8;
}
impl COM2A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COM2A_A {
        match self.bits {
            0 => COM2A_A::NORMAL_PORT_OPERATION_OC2A_DISCONNECTED,
            1 => COM2A_A::TOGGLE_OC2A_ON_COMPARE_MATCH,
            2 => COM2A_A::CLEAR_OC2A_ON_COMPARE_MATCH,
            3 => COM2A_A::SET_OC2A_ON_COMPARE_MATCH,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal port operation, OC2A disconnected"]
    #[inline(always)]
    pub fn is_normal_port_operation_oc2a_disconnected(&self) -> bool {
        *self == COM2A_A::NORMAL_PORT_OPERATION_OC2A_DISCONNECTED
    }
    #[doc = "Toggle OC2A on Compare Match"]
    #[inline(always)]
    pub fn is_toggle_oc2a_on_compare_match(&self) -> bool {
        *self == COM2A_A::TOGGLE_OC2A_ON_COMPARE_MATCH
    }
    #[doc = "Clear OC2A on Compare Match"]
    #[inline(always)]
    pub fn is_clear_oc2a_on_compare_match(&self) -> bool {
        *self == COM2A_A::CLEAR_OC2A_ON_COMPARE_MATCH
    }
    #[doc = "Set OC2A on Compare Match"]
    #[inline(always)]
    pub fn is_set_oc2a_on_compare_match(&self) -> bool {
        *self == COM2A_A::SET_OC2A_ON_COMPARE_MATCH
    }
}
#[doc = "Field `COM2A` writer - Compare Match Output A Mode"]
pub type COM2A_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COM2A_A>;
impl<'a, REG> COM2A_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal port operation, OC2A disconnected"]
    #[inline(always)]
    pub fn normal_port_operation_oc2a_disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(COM2A_A::NORMAL_PORT_OPERATION_OC2A_DISCONNECTED)
    }
    #[doc = "Toggle OC2A on Compare Match"]
    #[inline(always)]
    pub fn toggle_oc2a_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM2A_A::TOGGLE_OC2A_ON_COMPARE_MATCH)
    }
    #[doc = "Clear OC2A on Compare Match"]
    #[inline(always)]
    pub fn clear_oc2a_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM2A_A::CLEAR_OC2A_ON_COMPARE_MATCH)
    }
    #[doc = "Set OC2A on Compare Match"]
    #[inline(always)]
    pub fn set_oc2a_on_compare_match(self) -> &'a mut crate::W<REG> {
        self.variant(COM2A_A::SET_OC2A_ON_COMPARE_MATCH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wgm2(&self) -> WGM2_R {
        WGM2_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Compare Match Output B Mode"]
    #[inline(always)]
    pub fn com2b(&self) -> COM2B_R {
        COM2B_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Compare Match Output A Mode"]
    #[inline(always)]
    pub fn com2a(&self) -> COM2A_R {
        COM2A_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wgm2(&mut self) -> WGM2_W<TCCR2A_SPEC> {
        WGM2_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<TCCR2A_SPEC> {
        RES_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Compare Match Output B Mode"]
    #[inline(always)]
    #[must_use]
    pub fn com2b(&mut self) -> COM2B_W<TCCR2A_SPEC> {
        COM2B_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Compare Match Output A Mode"]
    #[inline(always)]
    #[must_use]
    pub fn com2a(&mut self) -> COM2A_W<TCCR2A_SPEC> {
        COM2A_W::new(self, 6)
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
#[doc = "Timer/Counter2 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr2a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr2a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCCR2A_SPEC;
impl crate::RegisterSpec for TCCR2A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tccr2a::R`](R) reader structure"]
impl crate::Readable for TCCR2A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tccr2a::W`](W) writer structure"]
impl crate::Writable for TCCR2A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR2A to value 0"]
impl crate::Resettable for TCCR2A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
