#[doc = "Register `BATMON` reader"]
pub type R = crate::R<BATMON_SPEC>;
#[doc = "Register `BATMON` writer"]
pub type W = crate::W<BATMON_SPEC>;
#[doc = "Field `BATMON_VTH` reader - Battery Monitor Threshold Voltage"]
pub type BATMON_VTH_R = crate::FieldReader<BATMON_VTH_A>;
#[doc = "Battery Monitor Threshold Voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BATMON_VTH_A {
    #[doc = "0: 2.550V (BATMON_HR=1) 1.70V (BATMON_HR=0)"]
    _2_550V_BATMON_HR_1_1_70V_BATMON_HR_0 = 0,
    #[doc = "1: 2.625V (BATMON_HR=1) 1.75V (BATMON_HR=0)"]
    _2_625V_BATMON_HR_1_1_75V_BATMON_HR_0 = 1,
    #[doc = "2: 2.700V (BATMON_HR=1) 1.80V (BATMON_HR=0)"]
    _2_700V_BATMON_HR_1_1_80V_BATMON_HR_0 = 2,
    #[doc = "3: 2.775V (BATMON_HR=1) 1.85V (BATMON_HR=0)"]
    _2_775V_BATMON_HR_1_1_85V_BATMON_HR_0 = 3,
    #[doc = "4: 2.850V (BATMON_HR=1) 1.90V (BATMON_HR=0)"]
    _2_850V_BATMON_HR_1_1_90V_BATMON_HR_0 = 4,
    #[doc = "5: 2.925V (BATMON_HR=1) 1.95V (BATMON_HR=0)"]
    _2_925V_BATMON_HR_1_1_95V_BATMON_HR_0 = 5,
    #[doc = "6: 3.000V (BATMON_HR=1) 2.00V (BATMON_HR=0)"]
    _3_000V_BATMON_HR_1_2_00V_BATMON_HR_0 = 6,
    #[doc = "7: 3.075V (BATMON_HR=1) 2.05V (BATMON_HR=0)"]
    _3_075V_BATMON_HR_1_2_05V_BATMON_HR_0 = 7,
    #[doc = "8: 3.150V (BATMON_HR=1) 2.10V (BATMON_HR=0)"]
    _3_150V_BATMON_HR_1_2_10V_BATMON_HR_0 = 8,
    #[doc = "9: 3.225V (BATMON_HR=1) 2.15V (BATMON_HR=0)"]
    _3_225V_BATMON_HR_1_2_15V_BATMON_HR_0 = 9,
    #[doc = "10: 3.300V (BATMON_HR=1) 2.20V (BATMON_HR=0)"]
    _3_300V_BATMON_HR_1_2_20V_BATMON_HR_0 = 10,
    #[doc = "11: 3.375V (BATMON_HR=1) 2.25V (BATMON_HR=0)"]
    _3_375V_BATMON_HR_1_2_25V_BATMON_HR_0 = 11,
    #[doc = "12: 3.450V (BATMON_HR=1) 2.30V (BATMON_HR=0)"]
    _3_450V_BATMON_HR_1_2_30V_BATMON_HR_0 = 12,
    #[doc = "13: 3.525V (BATMON_HR=1) 2.35V (BATMON_HR=0)"]
    _3_525V_BATMON_HR_1_2_35V_BATMON_HR_0 = 13,
    #[doc = "14: 3.600V (BATMON_HR=1) 2.40V (BATMON_HR=0)"]
    _3_600V_BATMON_HR_1_2_40V_BATMON_HR_0 = 14,
    #[doc = "15: 3.675V (BATMON_HR=1) 2.45V (BATMON_HR=0)"]
    _3_675V_BATMON_HR_1_2_45V_BATMON_HR_0 = 15,
}
impl From<BATMON_VTH_A> for u8 {
    #[inline(always)]
    fn from(variant: BATMON_VTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BATMON_VTH_A {
    type Ux = u8;
}
impl BATMON_VTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BATMON_VTH_A {
        match self.bits {
            0 => BATMON_VTH_A::_2_550V_BATMON_HR_1_1_70V_BATMON_HR_0,
            1 => BATMON_VTH_A::_2_625V_BATMON_HR_1_1_75V_BATMON_HR_0,
            2 => BATMON_VTH_A::_2_700V_BATMON_HR_1_1_80V_BATMON_HR_0,
            3 => BATMON_VTH_A::_2_775V_BATMON_HR_1_1_85V_BATMON_HR_0,
            4 => BATMON_VTH_A::_2_850V_BATMON_HR_1_1_90V_BATMON_HR_0,
            5 => BATMON_VTH_A::_2_925V_BATMON_HR_1_1_95V_BATMON_HR_0,
            6 => BATMON_VTH_A::_3_000V_BATMON_HR_1_2_00V_BATMON_HR_0,
            7 => BATMON_VTH_A::_3_075V_BATMON_HR_1_2_05V_BATMON_HR_0,
            8 => BATMON_VTH_A::_3_150V_BATMON_HR_1_2_10V_BATMON_HR_0,
            9 => BATMON_VTH_A::_3_225V_BATMON_HR_1_2_15V_BATMON_HR_0,
            10 => BATMON_VTH_A::_3_300V_BATMON_HR_1_2_20V_BATMON_HR_0,
            11 => BATMON_VTH_A::_3_375V_BATMON_HR_1_2_25V_BATMON_HR_0,
            12 => BATMON_VTH_A::_3_450V_BATMON_HR_1_2_30V_BATMON_HR_0,
            13 => BATMON_VTH_A::_3_525V_BATMON_HR_1_2_35V_BATMON_HR_0,
            14 => BATMON_VTH_A::_3_600V_BATMON_HR_1_2_40V_BATMON_HR_0,
            15 => BATMON_VTH_A::_3_675V_BATMON_HR_1_2_45V_BATMON_HR_0,
            _ => unreachable!(),
        }
    }
    #[doc = "2.550V (BATMON_HR=1) 1.70V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_2_550v_batmon_hr_1_1_70v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_2_550V_BATMON_HR_1_1_70V_BATMON_HR_0
    }
    #[doc = "2.625V (BATMON_HR=1) 1.75V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_2_625v_batmon_hr_1_1_75v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_2_625V_BATMON_HR_1_1_75V_BATMON_HR_0
    }
    #[doc = "2.700V (BATMON_HR=1) 1.80V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_2_700v_batmon_hr_1_1_80v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_2_700V_BATMON_HR_1_1_80V_BATMON_HR_0
    }
    #[doc = "2.775V (BATMON_HR=1) 1.85V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_2_775v_batmon_hr_1_1_85v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_2_775V_BATMON_HR_1_1_85V_BATMON_HR_0
    }
    #[doc = "2.850V (BATMON_HR=1) 1.90V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_2_850v_batmon_hr_1_1_90v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_2_850V_BATMON_HR_1_1_90V_BATMON_HR_0
    }
    #[doc = "2.925V (BATMON_HR=1) 1.95V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_2_925v_batmon_hr_1_1_95v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_2_925V_BATMON_HR_1_1_95V_BATMON_HR_0
    }
    #[doc = "3.000V (BATMON_HR=1) 2.00V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_3_000v_batmon_hr_1_2_00v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_3_000V_BATMON_HR_1_2_00V_BATMON_HR_0
    }
    #[doc = "3.075V (BATMON_HR=1) 2.05V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_3_075v_batmon_hr_1_2_05v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_3_075V_BATMON_HR_1_2_05V_BATMON_HR_0
    }
    #[doc = "3.150V (BATMON_HR=1) 2.10V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_3_150v_batmon_hr_1_2_10v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_3_150V_BATMON_HR_1_2_10V_BATMON_HR_0
    }
    #[doc = "3.225V (BATMON_HR=1) 2.15V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_3_225v_batmon_hr_1_2_15v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_3_225V_BATMON_HR_1_2_15V_BATMON_HR_0
    }
    #[doc = "3.300V (BATMON_HR=1) 2.20V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_3_300v_batmon_hr_1_2_20v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_3_300V_BATMON_HR_1_2_20V_BATMON_HR_0
    }
    #[doc = "3.375V (BATMON_HR=1) 2.25V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_3_375v_batmon_hr_1_2_25v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_3_375V_BATMON_HR_1_2_25V_BATMON_HR_0
    }
    #[doc = "3.450V (BATMON_HR=1) 2.30V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_3_450v_batmon_hr_1_2_30v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_3_450V_BATMON_HR_1_2_30V_BATMON_HR_0
    }
    #[doc = "3.525V (BATMON_HR=1) 2.35V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_3_525v_batmon_hr_1_2_35v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_3_525V_BATMON_HR_1_2_35V_BATMON_HR_0
    }
    #[doc = "3.600V (BATMON_HR=1) 2.40V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_3_600v_batmon_hr_1_2_40v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_3_600V_BATMON_HR_1_2_40V_BATMON_HR_0
    }
    #[doc = "3.675V (BATMON_HR=1) 2.45V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn is_3_675v_batmon_hr_1_2_45v_batmon_hr_0(&self) -> bool {
        *self == BATMON_VTH_A::_3_675V_BATMON_HR_1_2_45V_BATMON_HR_0
    }
}
#[doc = "Field `BATMON_VTH` writer - Battery Monitor Threshold Voltage"]
pub type BATMON_VTH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, BATMON_VTH_A>;
impl<'a, REG> BATMON_VTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.550V (BATMON_HR=1) 1.70V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _2_550v_batmon_hr_1_1_70v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_2_550V_BATMON_HR_1_1_70V_BATMON_HR_0)
    }
    #[doc = "2.625V (BATMON_HR=1) 1.75V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _2_625v_batmon_hr_1_1_75v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_2_625V_BATMON_HR_1_1_75V_BATMON_HR_0)
    }
    #[doc = "2.700V (BATMON_HR=1) 1.80V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _2_700v_batmon_hr_1_1_80v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_2_700V_BATMON_HR_1_1_80V_BATMON_HR_0)
    }
    #[doc = "2.775V (BATMON_HR=1) 1.85V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _2_775v_batmon_hr_1_1_85v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_2_775V_BATMON_HR_1_1_85V_BATMON_HR_0)
    }
    #[doc = "2.850V (BATMON_HR=1) 1.90V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _2_850v_batmon_hr_1_1_90v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_2_850V_BATMON_HR_1_1_90V_BATMON_HR_0)
    }
    #[doc = "2.925V (BATMON_HR=1) 1.95V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _2_925v_batmon_hr_1_1_95v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_2_925V_BATMON_HR_1_1_95V_BATMON_HR_0)
    }
    #[doc = "3.000V (BATMON_HR=1) 2.00V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _3_000v_batmon_hr_1_2_00v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_3_000V_BATMON_HR_1_2_00V_BATMON_HR_0)
    }
    #[doc = "3.075V (BATMON_HR=1) 2.05V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _3_075v_batmon_hr_1_2_05v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_3_075V_BATMON_HR_1_2_05V_BATMON_HR_0)
    }
    #[doc = "3.150V (BATMON_HR=1) 2.10V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _3_150v_batmon_hr_1_2_10v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_3_150V_BATMON_HR_1_2_10V_BATMON_HR_0)
    }
    #[doc = "3.225V (BATMON_HR=1) 2.15V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _3_225v_batmon_hr_1_2_15v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_3_225V_BATMON_HR_1_2_15V_BATMON_HR_0)
    }
    #[doc = "3.300V (BATMON_HR=1) 2.20V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _3_300v_batmon_hr_1_2_20v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_3_300V_BATMON_HR_1_2_20V_BATMON_HR_0)
    }
    #[doc = "3.375V (BATMON_HR=1) 2.25V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _3_375v_batmon_hr_1_2_25v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_3_375V_BATMON_HR_1_2_25V_BATMON_HR_0)
    }
    #[doc = "3.450V (BATMON_HR=1) 2.30V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _3_450v_batmon_hr_1_2_30v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_3_450V_BATMON_HR_1_2_30V_BATMON_HR_0)
    }
    #[doc = "3.525V (BATMON_HR=1) 2.35V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _3_525v_batmon_hr_1_2_35v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_3_525V_BATMON_HR_1_2_35V_BATMON_HR_0)
    }
    #[doc = "3.600V (BATMON_HR=1) 2.40V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _3_600v_batmon_hr_1_2_40v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_3_600V_BATMON_HR_1_2_40V_BATMON_HR_0)
    }
    #[doc = "3.675V (BATMON_HR=1) 2.45V (BATMON_HR=0)"]
    #[inline(always)]
    pub fn _3_675v_batmon_hr_1_2_45v_batmon_hr_0(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_VTH_A::_3_675V_BATMON_HR_1_2_45V_BATMON_HR_0)
    }
}
#[doc = "Field `BATMON_HR` reader - Battery Monitor Voltage Range"]
pub type BATMON_HR_R = crate::BitReader<BATMON_HR_A>;
#[doc = "Battery Monitor Voltage Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BATMON_HR_A {
    #[doc = "0: Enables the low range, see BATMON_VTH"]
    BATMON_HR_DIS = 0,
    #[doc = "1: Enables the high range, see BATMON_VTH"]
    BATMON_HR_EN = 1,
}
impl From<BATMON_HR_A> for bool {
    #[inline(always)]
    fn from(variant: BATMON_HR_A) -> Self {
        variant as u8 != 0
    }
}
impl BATMON_HR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BATMON_HR_A {
        match self.bits {
            false => BATMON_HR_A::BATMON_HR_DIS,
            true => BATMON_HR_A::BATMON_HR_EN,
        }
    }
    #[doc = "Enables the low range, see BATMON_VTH"]
    #[inline(always)]
    pub fn is_batmon_hr_dis(&self) -> bool {
        *self == BATMON_HR_A::BATMON_HR_DIS
    }
    #[doc = "Enables the high range, see BATMON_VTH"]
    #[inline(always)]
    pub fn is_batmon_hr_en(&self) -> bool {
        *self == BATMON_HR_A::BATMON_HR_EN
    }
}
#[doc = "Field `BATMON_HR` writer - Battery Monitor Voltage Range"]
pub type BATMON_HR_W<'a, REG> = crate::BitWriter<'a, REG, BATMON_HR_A>;
impl<'a, REG> BATMON_HR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enables the low range, see BATMON_VTH"]
    #[inline(always)]
    pub fn batmon_hr_dis(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_HR_A::BATMON_HR_DIS)
    }
    #[doc = "Enables the high range, see BATMON_VTH"]
    #[inline(always)]
    pub fn batmon_hr_en(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_HR_A::BATMON_HR_EN)
    }
}
#[doc = "Field `BATMON_OK` reader - Battery Monitor Status"]
pub type BATMON_OK_R = crate::BitReader<BATMON_OK_A>;
#[doc = "Battery Monitor Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BATMON_OK_A {
    #[doc = "0: The battery voltage is below the threshold."]
    THE_BATTERY_VOLTAGE_IS_BELOW_THE_THRESHOLD = 0,
    #[doc = "1: The battery voltage is above the threshold."]
    THE_BATTERY_VOLTAGE_IS_ABOVE_THE_THRESHOLD = 1,
}
impl From<BATMON_OK_A> for bool {
    #[inline(always)]
    fn from(variant: BATMON_OK_A) -> Self {
        variant as u8 != 0
    }
}
impl BATMON_OK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BATMON_OK_A {
        match self.bits {
            false => BATMON_OK_A::THE_BATTERY_VOLTAGE_IS_BELOW_THE_THRESHOLD,
            true => BATMON_OK_A::THE_BATTERY_VOLTAGE_IS_ABOVE_THE_THRESHOLD,
        }
    }
    #[doc = "The battery voltage is below the threshold."]
    #[inline(always)]
    pub fn is_the_battery_voltage_is_below_the_threshold(&self) -> bool {
        *self == BATMON_OK_A::THE_BATTERY_VOLTAGE_IS_BELOW_THE_THRESHOLD
    }
    #[doc = "The battery voltage is above the threshold."]
    #[inline(always)]
    pub fn is_the_battery_voltage_is_above_the_threshold(&self) -> bool {
        *self == BATMON_OK_A::THE_BATTERY_VOLTAGE_IS_ABOVE_THE_THRESHOLD
    }
}
#[doc = "Field `BATMON_OK` writer - Battery Monitor Status"]
pub type BATMON_OK_W<'a, REG> = crate::BitWriter<'a, REG, BATMON_OK_A>;
impl<'a, REG> BATMON_OK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The battery voltage is below the threshold."]
    #[inline(always)]
    pub fn the_battery_voltage_is_below_the_threshold(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_OK_A::THE_BATTERY_VOLTAGE_IS_BELOW_THE_THRESHOLD)
    }
    #[doc = "The battery voltage is above the threshold."]
    #[inline(always)]
    pub fn the_battery_voltage_is_above_the_threshold(self) -> &'a mut crate::W<REG> {
        self.variant(BATMON_OK_A::THE_BATTERY_VOLTAGE_IS_ABOVE_THE_THRESHOLD)
    }
}
#[doc = "Field `BAT_LOW_EN` reader - Battery Monitor Interrupt Enable"]
pub type BAT_LOW_EN_R = crate::BitReader;
#[doc = "Field `BAT_LOW_EN` writer - Battery Monitor Interrupt Enable"]
pub type BAT_LOW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BAT_LOW` reader - Battery Monitor Interrupt Status"]
pub type BAT_LOW_R = crate::BitReader;
#[doc = "Field `BAT_LOW` writer - Battery Monitor Interrupt Status"]
pub type BAT_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Battery Monitor Threshold Voltage"]
    #[inline(always)]
    pub fn batmon_vth(&self) -> BATMON_VTH_R {
        BATMON_VTH_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Battery Monitor Voltage Range"]
    #[inline(always)]
    pub fn batmon_hr(&self) -> BATMON_HR_R {
        BATMON_HR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Battery Monitor Status"]
    #[inline(always)]
    pub fn batmon_ok(&self) -> BATMON_OK_R {
        BATMON_OK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Battery Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn bat_low_en(&self) -> BAT_LOW_EN_R {
        BAT_LOW_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Battery Monitor Interrupt Status"]
    #[inline(always)]
    pub fn bat_low(&self) -> BAT_LOW_R {
        BAT_LOW_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Battery Monitor Threshold Voltage"]
    #[inline(always)]
    #[must_use]
    pub fn batmon_vth(&mut self) -> BATMON_VTH_W<BATMON_SPEC> {
        BATMON_VTH_W::new(self, 0)
    }
    #[doc = "Bit 4 - Battery Monitor Voltage Range"]
    #[inline(always)]
    #[must_use]
    pub fn batmon_hr(&mut self) -> BATMON_HR_W<BATMON_SPEC> {
        BATMON_HR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Battery Monitor Status"]
    #[inline(always)]
    #[must_use]
    pub fn batmon_ok(&mut self) -> BATMON_OK_W<BATMON_SPEC> {
        BATMON_OK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Battery Monitor Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bat_low_en(&mut self) -> BAT_LOW_EN_W<BATMON_SPEC> {
        BAT_LOW_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Battery Monitor Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn bat_low(&mut self) -> BAT_LOW_W<BATMON_SPEC> {
        BAT_LOW_W::new(self, 7)
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
#[doc = "Battery Monitor Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batmon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batmon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BATMON_SPEC;
impl crate::RegisterSpec for BATMON_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`batmon::R`](R) reader structure"]
impl crate::Readable for BATMON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`batmon::W`](W) writer structure"]
impl crate::Writable for BATMON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BATMON to value 0"]
impl crate::Resettable for BATMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
