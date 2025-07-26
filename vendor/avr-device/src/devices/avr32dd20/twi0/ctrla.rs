#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `FMPEN` reader - Fast-mode Plus Enable"]
pub type FMPEN_R = crate::BitReader<FMPEN_A>;
#[doc = "Fast-mode Plus Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMPEN_A {
    #[doc = "0: Operating in Standard-mode or Fast-mode"]
    OFF = 0,
    #[doc = "1: Operating in Fast-mode Plus"]
    ON = 1,
}
impl From<FMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FMPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMPEN_A {
        match self.bits {
            false => FMPEN_A::OFF,
            true => FMPEN_A::ON,
        }
    }
    #[doc = "Operating in Standard-mode or Fast-mode"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FMPEN_A::OFF
    }
    #[doc = "Operating in Fast-mode Plus"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FMPEN_A::ON
    }
}
#[doc = "Field `FMPEN` writer - Fast-mode Plus Enable"]
pub type FMPEN_W<'a, REG> = crate::BitWriter<'a, REG, FMPEN_A>;
impl<'a, REG> FMPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Operating in Standard-mode or Fast-mode"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(FMPEN_A::OFF)
    }
    #[doc = "Operating in Fast-mode Plus"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(FMPEN_A::ON)
    }
}
#[doc = "Field `SDAHOLD` reader - SDA Hold Time"]
pub type SDAHOLD_R = crate::FieldReader<SDAHOLD_A>;
#[doc = "SDA Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDAHOLD_A {
    #[doc = "0: No SDA Hold Delay"]
    OFF = 0,
    #[doc = "1: Short SDA hold time"]
    _50NS = 1,
    #[doc = "2: Meets SMBUS 2.0 specification under typical conditions"]
    _300NS = 2,
    #[doc = "3: Meets SMBUS 2.0 specificaiton across all corners"]
    _500NS = 3,
}
impl From<SDAHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: SDAHOLD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDAHOLD_A {
    type Ux = u8;
}
impl SDAHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDAHOLD_A {
        match self.bits {
            0 => SDAHOLD_A::OFF,
            1 => SDAHOLD_A::_50NS,
            2 => SDAHOLD_A::_300NS,
            3 => SDAHOLD_A::_500NS,
            _ => unreachable!(),
        }
    }
    #[doc = "No SDA Hold Delay"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SDAHOLD_A::OFF
    }
    #[doc = "Short SDA hold time"]
    #[inline(always)]
    pub fn is_50ns(&self) -> bool {
        *self == SDAHOLD_A::_50NS
    }
    #[doc = "Meets SMBUS 2.0 specification under typical conditions"]
    #[inline(always)]
    pub fn is_300ns(&self) -> bool {
        *self == SDAHOLD_A::_300NS
    }
    #[doc = "Meets SMBUS 2.0 specificaiton across all corners"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == SDAHOLD_A::_500NS
    }
}
#[doc = "Field `SDAHOLD` writer - SDA Hold Time"]
pub type SDAHOLD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SDAHOLD_A>;
impl<'a, REG> SDAHOLD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No SDA Hold Delay"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLD_A::OFF)
    }
    #[doc = "Short SDA hold time"]
    #[inline(always)]
    pub fn _50ns(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLD_A::_50NS)
    }
    #[doc = "Meets SMBUS 2.0 specification under typical conditions"]
    #[inline(always)]
    pub fn _300ns(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLD_A::_300NS)
    }
    #[doc = "Meets SMBUS 2.0 specificaiton across all corners"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut crate::W<REG> {
        self.variant(SDAHOLD_A::_500NS)
    }
}
#[doc = "Field `SDASETUP` reader - SDA Setup Time"]
pub type SDASETUP_R = crate::BitReader<SDASETUP_A>;
#[doc = "SDA Setup Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDASETUP_A {
    #[doc = "0: SDA setup time is four clock cycles"]
    _4CYC = 0,
    #[doc = "1: SDA setup time is eight clock cycle"]
    _8CYC = 1,
}
impl From<SDASETUP_A> for bool {
    #[inline(always)]
    fn from(variant: SDASETUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SDASETUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDASETUP_A {
        match self.bits {
            false => SDASETUP_A::_4CYC,
            true => SDASETUP_A::_8CYC,
        }
    }
    #[doc = "SDA setup time is four clock cycles"]
    #[inline(always)]
    pub fn is_4cyc(&self) -> bool {
        *self == SDASETUP_A::_4CYC
    }
    #[doc = "SDA setup time is eight clock cycle"]
    #[inline(always)]
    pub fn is_8cyc(&self) -> bool {
        *self == SDASETUP_A::_8CYC
    }
}
#[doc = "Field `SDASETUP` writer - SDA Setup Time"]
pub type SDASETUP_W<'a, REG> = crate::BitWriter<'a, REG, SDASETUP_A>;
impl<'a, REG> SDASETUP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDA setup time is four clock cycles"]
    #[inline(always)]
    pub fn _4cyc(self) -> &'a mut crate::W<REG> {
        self.variant(SDASETUP_A::_4CYC)
    }
    #[doc = "SDA setup time is eight clock cycle"]
    #[inline(always)]
    pub fn _8cyc(self) -> &'a mut crate::W<REG> {
        self.variant(SDASETUP_A::_8CYC)
    }
}
#[doc = "Field `INPUTLVL` reader - Input voltage transition level"]
pub type INPUTLVL_R = crate::BitReader<INPUTLVL_A>;
#[doc = "Input voltage transition level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPUTLVL_A {
    #[doc = "0: I2C input voltage transition level"]
    I2C = 0,
    #[doc = "1: SMBus 3.0 input voltage transition level"]
    SMBUS = 1,
}
impl From<INPUTLVL_A> for bool {
    #[inline(always)]
    fn from(variant: INPUTLVL_A) -> Self {
        variant as u8 != 0
    }
}
impl INPUTLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INPUTLVL_A {
        match self.bits {
            false => INPUTLVL_A::I2C,
            true => INPUTLVL_A::SMBUS,
        }
    }
    #[doc = "I2C input voltage transition level"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == INPUTLVL_A::I2C
    }
    #[doc = "SMBus 3.0 input voltage transition level"]
    #[inline(always)]
    pub fn is_smbus(&self) -> bool {
        *self == INPUTLVL_A::SMBUS
    }
}
#[doc = "Field `INPUTLVL` writer - Input voltage transition level"]
pub type INPUTLVL_W<'a, REG> = crate::BitWriter<'a, REG, INPUTLVL_A>;
impl<'a, REG> INPUTLVL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C input voltage transition level"]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTLVL_A::I2C)
    }
    #[doc = "SMBus 3.0 input voltage transition level"]
    #[inline(always)]
    pub fn smbus(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTLVL_A::SMBUS)
    }
}
impl R {
    #[doc = "Bit 1 - Fast-mode Plus Enable"]
    #[inline(always)]
    pub fn fmpen(&self) -> FMPEN_R {
        FMPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&self) -> SDAHOLD_R {
        SDAHOLD_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - SDA Setup Time"]
    #[inline(always)]
    pub fn sdasetup(&self) -> SDASETUP_R {
        SDASETUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Input voltage transition level"]
    #[inline(always)]
    pub fn inputlvl(&self) -> INPUTLVL_R {
        INPUTLVL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Fast-mode Plus Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmpen(&mut self) -> FMPEN_W<CTRLA_SPEC> {
        FMPEN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - SDA Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn sdahold(&mut self) -> SDAHOLD_W<CTRLA_SPEC> {
        SDAHOLD_W::new(self, 2)
    }
    #[doc = "Bit 4 - SDA Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn sdasetup(&mut self) -> SDASETUP_W<CTRLA_SPEC> {
        SDASETUP_W::new(self, 4)
    }
    #[doc = "Bit 6 - Input voltage transition level"]
    #[inline(always)]
    #[must_use]
    pub fn inputlvl(&mut self) -> INPUTLVL_W<CTRLA_SPEC> {
        INPUTLVL_W::new(self, 6)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
