#[doc = "Register `BGCR` reader"]
pub type R = crate::R<BGCR_SPEC>;
#[doc = "Register `BGCR` writer"]
pub type W = crate::W<BGCR_SPEC>;
#[doc = "Field `BGCAL` reader - Coarse Calibration Bits"]
pub type BGCAL_R = crate::FieldReader<BGCAL_A>;
#[doc = "Coarse Calibration Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BGCAL_A {
    #[doc = "0: Setting for highest voltage"]
    SETTING_FOR_HIGHEST_VOLTAGE = 0,
    #[doc = "3: Voltage step up"]
    VOLTAGE_STEP_UP = 3,
    #[doc = "4: Center value"]
    CENTER_VALUE = 4,
    #[doc = "5: Voltage step down"]
    VOLTAGE_STEP_DOWN = 5,
    #[doc = "7: Setting for lowest voltage"]
    SETTING_FOR_LOWEST_VOLTAGE = 7,
}
impl From<BGCAL_A> for u8 {
    #[inline(always)]
    fn from(variant: BGCAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BGCAL_A {
    type Ux = u8;
}
impl BGCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BGCAL_A> {
        match self.bits {
            0 => Some(BGCAL_A::SETTING_FOR_HIGHEST_VOLTAGE),
            3 => Some(BGCAL_A::VOLTAGE_STEP_UP),
            4 => Some(BGCAL_A::CENTER_VALUE),
            5 => Some(BGCAL_A::VOLTAGE_STEP_DOWN),
            7 => Some(BGCAL_A::SETTING_FOR_LOWEST_VOLTAGE),
            _ => None,
        }
    }
    #[doc = "Setting for highest voltage"]
    #[inline(always)]
    pub fn is_setting_for_highest_voltage(&self) -> bool {
        *self == BGCAL_A::SETTING_FOR_HIGHEST_VOLTAGE
    }
    #[doc = "Voltage step up"]
    #[inline(always)]
    pub fn is_voltage_step_up(&self) -> bool {
        *self == BGCAL_A::VOLTAGE_STEP_UP
    }
    #[doc = "Center value"]
    #[inline(always)]
    pub fn is_center_value(&self) -> bool {
        *self == BGCAL_A::CENTER_VALUE
    }
    #[doc = "Voltage step down"]
    #[inline(always)]
    pub fn is_voltage_step_down(&self) -> bool {
        *self == BGCAL_A::VOLTAGE_STEP_DOWN
    }
    #[doc = "Setting for lowest voltage"]
    #[inline(always)]
    pub fn is_setting_for_lowest_voltage(&self) -> bool {
        *self == BGCAL_A::SETTING_FOR_LOWEST_VOLTAGE
    }
}
#[doc = "Field `BGCAL` writer - Coarse Calibration Bits"]
pub type BGCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BGCAL_A>;
impl<'a, REG> BGCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Setting for highest voltage"]
    #[inline(always)]
    pub fn setting_for_highest_voltage(self) -> &'a mut crate::W<REG> {
        self.variant(BGCAL_A::SETTING_FOR_HIGHEST_VOLTAGE)
    }
    #[doc = "Voltage step up"]
    #[inline(always)]
    pub fn voltage_step_up(self) -> &'a mut crate::W<REG> {
        self.variant(BGCAL_A::VOLTAGE_STEP_UP)
    }
    #[doc = "Center value"]
    #[inline(always)]
    pub fn center_value(self) -> &'a mut crate::W<REG> {
        self.variant(BGCAL_A::CENTER_VALUE)
    }
    #[doc = "Voltage step down"]
    #[inline(always)]
    pub fn voltage_step_down(self) -> &'a mut crate::W<REG> {
        self.variant(BGCAL_A::VOLTAGE_STEP_DOWN)
    }
    #[doc = "Setting for lowest voltage"]
    #[inline(always)]
    pub fn setting_for_lowest_voltage(self) -> &'a mut crate::W<REG> {
        self.variant(BGCAL_A::SETTING_FOR_LOWEST_VOLTAGE)
    }
}
#[doc = "Field `BGCAL_FINE` reader - Fine Calibration Bits"]
pub type BGCAL_FINE_R = crate::FieldReader<BGCAL_FINE_A>;
#[doc = "Fine Calibration Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BGCAL_FINE_A {
    #[doc = "0: Center value"]
    CENTER_VALUE = 0,
    #[doc = "1: Voltage step up"]
    VOLTAGE_STEP_UP = 1,
    #[doc = "7: Setting for highest voltage"]
    SETTING_FOR_HIGHEST_VOLTAGE = 7,
    #[doc = "8: Voltage step down"]
    VOLTAGE_STEP_DOWN = 8,
    #[doc = "15: Setting for lowest voltage"]
    SETTING_FOR_LOWEST_VOLTAGE = 15,
}
impl From<BGCAL_FINE_A> for u8 {
    #[inline(always)]
    fn from(variant: BGCAL_FINE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BGCAL_FINE_A {
    type Ux = u8;
}
impl BGCAL_FINE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BGCAL_FINE_A> {
        match self.bits {
            0 => Some(BGCAL_FINE_A::CENTER_VALUE),
            1 => Some(BGCAL_FINE_A::VOLTAGE_STEP_UP),
            7 => Some(BGCAL_FINE_A::SETTING_FOR_HIGHEST_VOLTAGE),
            8 => Some(BGCAL_FINE_A::VOLTAGE_STEP_DOWN),
            15 => Some(BGCAL_FINE_A::SETTING_FOR_LOWEST_VOLTAGE),
            _ => None,
        }
    }
    #[doc = "Center value"]
    #[inline(always)]
    pub fn is_center_value(&self) -> bool {
        *self == BGCAL_FINE_A::CENTER_VALUE
    }
    #[doc = "Voltage step up"]
    #[inline(always)]
    pub fn is_voltage_step_up(&self) -> bool {
        *self == BGCAL_FINE_A::VOLTAGE_STEP_UP
    }
    #[doc = "Setting for highest voltage"]
    #[inline(always)]
    pub fn is_setting_for_highest_voltage(&self) -> bool {
        *self == BGCAL_FINE_A::SETTING_FOR_HIGHEST_VOLTAGE
    }
    #[doc = "Voltage step down"]
    #[inline(always)]
    pub fn is_voltage_step_down(&self) -> bool {
        *self == BGCAL_FINE_A::VOLTAGE_STEP_DOWN
    }
    #[doc = "Setting for lowest voltage"]
    #[inline(always)]
    pub fn is_setting_for_lowest_voltage(&self) -> bool {
        *self == BGCAL_FINE_A::SETTING_FOR_LOWEST_VOLTAGE
    }
}
#[doc = "Field `BGCAL_FINE` writer - Fine Calibration Bits"]
pub type BGCAL_FINE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BGCAL_FINE_A>;
impl<'a, REG> BGCAL_FINE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Center value"]
    #[inline(always)]
    pub fn center_value(self) -> &'a mut crate::W<REG> {
        self.variant(BGCAL_FINE_A::CENTER_VALUE)
    }
    #[doc = "Voltage step up"]
    #[inline(always)]
    pub fn voltage_step_up(self) -> &'a mut crate::W<REG> {
        self.variant(BGCAL_FINE_A::VOLTAGE_STEP_UP)
    }
    #[doc = "Setting for highest voltage"]
    #[inline(always)]
    pub fn setting_for_highest_voltage(self) -> &'a mut crate::W<REG> {
        self.variant(BGCAL_FINE_A::SETTING_FOR_HIGHEST_VOLTAGE)
    }
    #[doc = "Voltage step down"]
    #[inline(always)]
    pub fn voltage_step_down(self) -> &'a mut crate::W<REG> {
        self.variant(BGCAL_FINE_A::VOLTAGE_STEP_DOWN)
    }
    #[doc = "Setting for lowest voltage"]
    #[inline(always)]
    pub fn setting_for_lowest_voltage(self) -> &'a mut crate::W<REG> {
        self.variant(BGCAL_FINE_A::SETTING_FOR_LOWEST_VOLTAGE)
    }
}
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::BitReader;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Coarse Calibration Bits"]
    #[inline(always)]
    pub fn bgcal(&self) -> BGCAL_R {
        BGCAL_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:6 - Fine Calibration Bits"]
    #[inline(always)]
    pub fn bgcal_fine(&self) -> BGCAL_FINE_R {
        BGCAL_FINE_R::new((self.bits >> 3) & 0x0f)
    }
    #[doc = "Bit 7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Coarse Calibration Bits"]
    #[inline(always)]
    #[must_use]
    pub fn bgcal(&mut self) -> BGCAL_W<BGCR_SPEC> {
        BGCAL_W::new(self, 0)
    }
    #[doc = "Bits 3:6 - Fine Calibration Bits"]
    #[inline(always)]
    #[must_use]
    pub fn bgcal_fine(&mut self) -> BGCAL_FINE_W<BGCR_SPEC> {
        BGCAL_FINE_W::new(self, 3)
    }
    #[doc = "Bit 7 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<BGCR_SPEC> {
        RES_W::new(self, 7)
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
#[doc = "Reference Voltage Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BGCR_SPEC;
impl crate::RegisterSpec for BGCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bgcr::R`](R) reader structure"]
impl crate::Readable for BGCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bgcr::W`](W) writer structure"]
impl crate::Writable for BGCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BGCR to value 0"]
impl crate::Resettable for BGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
