#[doc = "Register `EECR` reader"]
pub type R = crate::R<EECR_SPEC>;
#[doc = "Register `EECR` writer"]
pub type W = crate::W<EECR_SPEC>;
#[doc = "Field `EERE` reader - EEPROM Read Enable"]
pub type EERE_R = crate::BitReader;
#[doc = "Field `EERE` writer - EEPROM Read Enable"]
pub type EERE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEPE` reader - EEPROM Programming Enable"]
pub type EEPE_R = crate::BitReader;
#[doc = "Field `EEPE` writer - EEPROM Programming Enable"]
pub type EEPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEMPE` reader - EEPROM Master Write Enable"]
pub type EEMPE_R = crate::BitReader;
#[doc = "Field `EEMPE` writer - EEPROM Master Write Enable"]
pub type EEMPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EERIE` reader - EEPROM Ready Interrupt Enable"]
pub type EERIE_R = crate::BitReader;
#[doc = "Field `EERIE` writer - EEPROM Ready Interrupt Enable"]
pub type EERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEPM` reader - EEPROM Programming Mode"]
pub type EEPM_R = crate::FieldReader<EEPM_A>;
#[doc = "EEPROM Programming Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EEPM_A {
    #[doc = "0: Erase and Write in one operation (Atomic Operation)"]
    ERASE_AND_WRITE_IN_ONE_OPERATION_ATOMIC_OPERATION = 0,
    #[doc = "1: Erase only"]
    ERASE_ONLY = 1,
    #[doc = "2: Write only"]
    WRITE_ONLY = 2,
    #[doc = "3: Reserved for future use"]
    RESERVED_FOR_FUTURE_USE = 3,
}
impl From<EEPM_A> for u8 {
    #[inline(always)]
    fn from(variant: EEPM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EEPM_A {
    type Ux = u8;
}
impl EEPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EEPM_A {
        match self.bits {
            0 => EEPM_A::ERASE_AND_WRITE_IN_ONE_OPERATION_ATOMIC_OPERATION,
            1 => EEPM_A::ERASE_ONLY,
            2 => EEPM_A::WRITE_ONLY,
            3 => EEPM_A::RESERVED_FOR_FUTURE_USE,
            _ => unreachable!(),
        }
    }
    #[doc = "Erase and Write in one operation (Atomic Operation)"]
    #[inline(always)]
    pub fn is_erase_and_write_in_one_operation_atomic_operation(&self) -> bool {
        *self == EEPM_A::ERASE_AND_WRITE_IN_ONE_OPERATION_ATOMIC_OPERATION
    }
    #[doc = "Erase only"]
    #[inline(always)]
    pub fn is_erase_only(&self) -> bool {
        *self == EEPM_A::ERASE_ONLY
    }
    #[doc = "Write only"]
    #[inline(always)]
    pub fn is_write_only(&self) -> bool {
        *self == EEPM_A::WRITE_ONLY
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn is_reserved_for_future_use(&self) -> bool {
        *self == EEPM_A::RESERVED_FOR_FUTURE_USE
    }
}
#[doc = "Field `EEPM` writer - EEPROM Programming Mode"]
pub type EEPM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EEPM_A>;
impl<'a, REG> EEPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Erase and Write in one operation (Atomic Operation)"]
    #[inline(always)]
    pub fn erase_and_write_in_one_operation_atomic_operation(self) -> &'a mut crate::W<REG> {
        self.variant(EEPM_A::ERASE_AND_WRITE_IN_ONE_OPERATION_ATOMIC_OPERATION)
    }
    #[doc = "Erase only"]
    #[inline(always)]
    pub fn erase_only(self) -> &'a mut crate::W<REG> {
        self.variant(EEPM_A::ERASE_ONLY)
    }
    #[doc = "Write only"]
    #[inline(always)]
    pub fn write_only(self) -> &'a mut crate::W<REG> {
        self.variant(EEPM_A::WRITE_ONLY)
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn reserved_for_future_use(self) -> &'a mut crate::W<REG> {
        self.variant(EEPM_A::RESERVED_FOR_FUTURE_USE)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - EEPROM Read Enable"]
    #[inline(always)]
    pub fn eere(&self) -> EERE_R {
        EERE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EEPROM Programming Enable"]
    #[inline(always)]
    pub fn eepe(&self) -> EEPE_R {
        EEPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EEPROM Master Write Enable"]
    #[inline(always)]
    pub fn eempe(&self) -> EEMPE_R {
        EEMPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EEPROM Ready Interrupt Enable"]
    #[inline(always)]
    pub fn eerie(&self) -> EERIE_R {
        EERIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - EEPROM Programming Mode"]
    #[inline(always)]
    pub fn eepm(&self) -> EEPM_R {
        EEPM_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - EEPROM Read Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eere(&mut self) -> EERE_W<EECR_SPEC> {
        EERE_W::new(self, 0)
    }
    #[doc = "Bit 1 - EEPROM Programming Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eepe(&mut self) -> EEPE_W<EECR_SPEC> {
        EEPE_W::new(self, 1)
    }
    #[doc = "Bit 2 - EEPROM Master Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eempe(&mut self) -> EEMPE_W<EECR_SPEC> {
        EEMPE_W::new(self, 2)
    }
    #[doc = "Bit 3 - EEPROM Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eerie(&mut self) -> EERIE_W<EECR_SPEC> {
        EERIE_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - EEPROM Programming Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eepm(&mut self) -> EEPM_W<EECR_SPEC> {
        EEPM_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<EECR_SPEC> {
        RES_W::new(self, 6)
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
#[doc = "EEPROM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EECR_SPEC;
impl crate::RegisterSpec for EECR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eecr::R`](R) reader structure"]
impl crate::Readable for EECR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eecr::W`](W) writer structure"]
impl crate::Writable for EECR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EECR to value 0"]
impl crate::Resettable for EECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
