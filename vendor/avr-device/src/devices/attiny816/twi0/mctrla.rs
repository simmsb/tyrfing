#[doc = "Register `MCTRLA` reader"]
pub type R = crate::R<MCTRLA_SPEC>;
#[doc = "Register `MCTRLA` writer"]
pub type W = crate::W<MCTRLA_SPEC>;
#[doc = "Field `ENABLE` reader - Enable TWI Master"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable TWI Master"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEN` reader - Smart Mode Enable"]
pub type SMEN_R = crate::BitReader;
#[doc = "Field `SMEN` writer - Smart Mode Enable"]
pub type SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - Inactive Bus Timeout"]
pub type TIMEOUT_R = crate::FieldReader<TIMEOUT_A>;
#[doc = "Inactive Bus Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUT_A {
    #[doc = "0: Bus Timeout Disabled"]
    DISABLED = 0,
    #[doc = "1: 50 Microseconds"]
    _50US = 1,
    #[doc = "2: 100 Microseconds"]
    _100US = 2,
    #[doc = "3: 200 Microseconds"]
    _200US = 3,
}
impl From<TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMEOUT_A {
    type Ux = u8;
}
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            0 => TIMEOUT_A::DISABLED,
            1 => TIMEOUT_A::_50US,
            2 => TIMEOUT_A::_100US,
            3 => TIMEOUT_A::_200US,
            _ => unreachable!(),
        }
    }
    #[doc = "Bus Timeout Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMEOUT_A::DISABLED
    }
    #[doc = "50 Microseconds"]
    #[inline(always)]
    pub fn is_50us(&self) -> bool {
        *self == TIMEOUT_A::_50US
    }
    #[doc = "100 Microseconds"]
    #[inline(always)]
    pub fn is_100us(&self) -> bool {
        *self == TIMEOUT_A::_100US
    }
    #[doc = "200 Microseconds"]
    #[inline(always)]
    pub fn is_200us(&self) -> bool {
        *self == TIMEOUT_A::_200US
    }
}
#[doc = "Field `TIMEOUT` writer - Inactive Bus Timeout"]
pub type TIMEOUT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TIMEOUT_A>;
impl<'a, REG> TIMEOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bus Timeout Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT_A::DISABLED)
    }
    #[doc = "50 Microseconds"]
    #[inline(always)]
    pub fn _50us(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT_A::_50US)
    }
    #[doc = "100 Microseconds"]
    #[inline(always)]
    pub fn _100us(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT_A::_100US)
    }
    #[doc = "200 Microseconds"]
    #[inline(always)]
    pub fn _200us(self) -> &'a mut crate::W<REG> {
        self.variant(TIMEOUT_A::_200US)
    }
}
#[doc = "Field `QCEN` reader - Quick Command Enable"]
pub type QCEN_R = crate::BitReader;
#[doc = "Field `QCEN` writer - Quick Command Enable"]
pub type QCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIEN` reader - Write Interrupt Enable"]
pub type WIEN_R = crate::BitReader;
#[doc = "Field `WIEN` writer - Write Interrupt Enable"]
pub type WIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIEN` reader - Read Interrupt Enable"]
pub type RIEN_R = crate::BitReader;
#[doc = "Field `RIEN` writer - Read Interrupt Enable"]
pub type RIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable TWI Master"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Inactive Bus Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Quick Command Enable"]
    #[inline(always)]
    pub fn qcen(&self) -> QCEN_R {
        QCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Interrupt Enable"]
    #[inline(always)]
    pub fn wien(&self) -> WIEN_R {
        WIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read Interrupt Enable"]
    #[inline(always)]
    pub fn rien(&self) -> RIEN_R {
        RIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable TWI Master"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<MCTRLA_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Smart Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smen(&mut self) -> SMEN_W<MCTRLA_SPEC> {
        SMEN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Inactive Bus Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<MCTRLA_SPEC> {
        TIMEOUT_W::new(self, 2)
    }
    #[doc = "Bit 4 - Quick Command Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qcen(&mut self) -> QCEN_W<MCTRLA_SPEC> {
        QCEN_W::new(self, 4)
    }
    #[doc = "Bit 6 - Write Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wien(&mut self) -> WIEN_W<MCTRLA_SPEC> {
        WIEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Read Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rien(&mut self) -> RIEN_W<MCTRLA_SPEC> {
        RIEN_W::new(self, 7)
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
#[doc = "Master Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCTRLA_SPEC;
impl crate::RegisterSpec for MCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mctrla::R`](R) reader structure"]
impl crate::Readable for MCTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mctrla::W`](W) writer structure"]
impl crate::Writable for MCTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTRLA to value 0"]
impl crate::Resettable for MCTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
