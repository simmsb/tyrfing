#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYSMODE` reader - Hysteresis Mode"]
pub type HYSMODE_R = crate::FieldReader<HYSMODE_A>;
#[doc = "Hysteresis Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYSMODE_A {
    #[doc = "0: No hysteresis"]
    OFF = 0,
    #[doc = "1: 10mV hysteresis"]
    _10M_V = 1,
    #[doc = "2: 25mV hysteresis"]
    _25M_V = 2,
    #[doc = "3: 50mV hysteresis"]
    _50M_V = 3,
}
impl From<HYSMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: HYSMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HYSMODE_A {
    type Ux = u8;
}
impl HYSMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HYSMODE_A {
        match self.bits {
            0 => HYSMODE_A::OFF,
            1 => HYSMODE_A::_10M_V,
            2 => HYSMODE_A::_25M_V,
            3 => HYSMODE_A::_50M_V,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HYSMODE_A::OFF
    }
    #[doc = "10mV hysteresis"]
    #[inline(always)]
    pub fn is_10m_v(&self) -> bool {
        *self == HYSMODE_A::_10M_V
    }
    #[doc = "25mV hysteresis"]
    #[inline(always)]
    pub fn is_25m_v(&self) -> bool {
        *self == HYSMODE_A::_25M_V
    }
    #[doc = "50mV hysteresis"]
    #[inline(always)]
    pub fn is_50m_v(&self) -> bool {
        *self == HYSMODE_A::_50M_V
    }
}
#[doc = "Field `HYSMODE` writer - Hysteresis Mode"]
pub type HYSMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HYSMODE_A>;
impl<'a, REG> HYSMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(HYSMODE_A::OFF)
    }
    #[doc = "10mV hysteresis"]
    #[inline(always)]
    pub fn _10m_v(self) -> &'a mut crate::W<REG> {
        self.variant(HYSMODE_A::_10M_V)
    }
    #[doc = "25mV hysteresis"]
    #[inline(always)]
    pub fn _25m_v(self) -> &'a mut crate::W<REG> {
        self.variant(HYSMODE_A::_25M_V)
    }
    #[doc = "50mV hysteresis"]
    #[inline(always)]
    pub fn _50m_v(self) -> &'a mut crate::W<REG> {
        self.variant(HYSMODE_A::_50M_V)
    }
}
#[doc = "Field `INTMODE` reader - Interrupt Mode"]
pub type INTMODE_R = crate::FieldReader<INTMODE_A>;
#[doc = "Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INTMODE_A {
    #[doc = "0: Any Edge"]
    BOTHEDGE = 0,
    #[doc = "2: Negative Edge"]
    NEGEDGE = 2,
    #[doc = "3: Positive Edge"]
    POSEDGE = 3,
}
impl From<INTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: INTMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INTMODE_A {
    type Ux = u8;
}
impl INTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INTMODE_A> {
        match self.bits {
            0 => Some(INTMODE_A::BOTHEDGE),
            2 => Some(INTMODE_A::NEGEDGE),
            3 => Some(INTMODE_A::POSEDGE),
            _ => None,
        }
    }
    #[doc = "Any Edge"]
    #[inline(always)]
    pub fn is_bothedge(&self) -> bool {
        *self == INTMODE_A::BOTHEDGE
    }
    #[doc = "Negative Edge"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == INTMODE_A::NEGEDGE
    }
    #[doc = "Positive Edge"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == INTMODE_A::POSEDGE
    }
}
#[doc = "Field `INTMODE` writer - Interrupt Mode"]
pub type INTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, INTMODE_A>;
impl<'a, REG> INTMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any Edge"]
    #[inline(always)]
    pub fn bothedge(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE_A::BOTHEDGE)
    }
    #[doc = "Negative Edge"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE_A::NEGEDGE)
    }
    #[doc = "Positive Edge"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut crate::W<REG> {
        self.variant(INTMODE_A::POSEDGE)
    }
}
#[doc = "Field `OUTEN` reader - Output Buffer Enable"]
pub type OUTEN_R = crate::BitReader;
#[doc = "Field `OUTEN` writer - Output Buffer Enable"]
pub type OUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby Mode"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby Mode"]
pub type RUNSTDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Hysteresis Mode"]
    #[inline(always)]
    pub fn hysmode(&self) -> HYSMODE_R {
        HYSMODE_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 4:5 - Interrupt Mode"]
    #[inline(always)]
    pub fn intmode(&self) -> INTMODE_R {
        INTMODE_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Output Buffer Enable"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Run in Standby Mode"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Hysteresis Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hysmode(&mut self) -> HYSMODE_W<CTRLA_SPEC> {
        HYSMODE_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn intmode(&mut self) -> INTMODE_W<CTRLA_SPEC> {
        INTMODE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Output Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<CTRLA_SPEC> {
        OUTEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Run in Standby Mode"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<CTRLA_SPEC> {
        RUNSTDBY_W::new(self, 7)
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
