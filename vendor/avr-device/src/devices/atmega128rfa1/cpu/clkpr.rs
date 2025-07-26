#[doc = "Register `CLKPR` reader"]
pub type R = crate::R<CLKPR_SPEC>;
#[doc = "Register `CLKPR` writer"]
pub type W = crate::W<CLKPR_SPEC>;
#[doc = "Field `CLKPS` reader - Clock Prescaler Select Bits"]
pub type CLKPS_R = crate::FieldReader<CLKPS_A>;
#[doc = "Clock Prescaler Select Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKPS_A {
    #[doc = "0: Division factor 1 / RC-Oscillator 2"]
    DIVISION_FACTOR_1_RC_OSCILLATOR_2 = 0,
    #[doc = "1: Division factor 2 / RC-Oscillator 4"]
    DIVISION_FACTOR_2_RC_OSCILLATOR_4 = 1,
    #[doc = "2: Division factor 4 / RC-Oscillator 8"]
    DIVISION_FACTOR_4_RC_OSCILLATOR_8 = 2,
    #[doc = "3: Division factor 8 / RC-Oscillator 16"]
    DIVISION_FACTOR_8_RC_OSCILLATOR_16 = 3,
    #[doc = "4: Division factor 16 / RC-Oscillator 32"]
    DIVISION_FACTOR_16_RC_OSCILLATOR_32 = 4,
    #[doc = "5: Division factor 32 / RC-Oscillator 64"]
    DIVISION_FACTOR_32_RC_OSCILLATOR_64 = 5,
    #[doc = "6: Division factor 64 / RC-Oscillator 128"]
    DIVISION_FACTOR_64_RC_OSCILLATOR_128 = 6,
    #[doc = "7: Division factor 128 / RC-Oscillator 256"]
    DIVISION_FACTOR_128_RC_OSCILLATOR_256 = 7,
    #[doc = "8: Division factor 256 / RC-Oscillator 512"]
    DIVISION_FACTOR_256_RC_OSCILLATOR_512 = 8,
    #[doc = "15: Division factor 1 only permitted for RC-Oscillator. Flash and EEPROM programming is not allowed."]
    DIVISION_FACTOR_1_ONLY_PERMITTED_FOR_RC_OSCILLATOR_FLASH_AND_EEPROM_PROGRAMMING_IS_NOT_ALLOWED =
        15,
}
impl From<CLKPS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKPS_A {
    type Ux = u8;
}
impl CLKPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKPS_A> {
        match self . bits { 0 => Some (CLKPS_A :: DIVISION_FACTOR_1_RC_OSCILLATOR_2) , 1 => Some (CLKPS_A :: DIVISION_FACTOR_2_RC_OSCILLATOR_4) , 2 => Some (CLKPS_A :: DIVISION_FACTOR_4_RC_OSCILLATOR_8) , 3 => Some (CLKPS_A :: DIVISION_FACTOR_8_RC_OSCILLATOR_16) , 4 => Some (CLKPS_A :: DIVISION_FACTOR_16_RC_OSCILLATOR_32) , 5 => Some (CLKPS_A :: DIVISION_FACTOR_32_RC_OSCILLATOR_64) , 6 => Some (CLKPS_A :: DIVISION_FACTOR_64_RC_OSCILLATOR_128) , 7 => Some (CLKPS_A :: DIVISION_FACTOR_128_RC_OSCILLATOR_256) , 8 => Some (CLKPS_A :: DIVISION_FACTOR_256_RC_OSCILLATOR_512) , 15 => Some (CLKPS_A :: DIVISION_FACTOR_1_ONLY_PERMITTED_FOR_RC_OSCILLATOR_FLASH_AND_EEPROM_PROGRAMMING_IS_NOT_ALLOWED) , _ => None , }
    }
    #[doc = "Division factor 1 / RC-Oscillator 2"]
    #[inline(always)]
    pub fn is_division_factor_1_rc_oscillator_2(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_1_RC_OSCILLATOR_2
    }
    #[doc = "Division factor 2 / RC-Oscillator 4"]
    #[inline(always)]
    pub fn is_division_factor_2_rc_oscillator_4(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_2_RC_OSCILLATOR_4
    }
    #[doc = "Division factor 4 / RC-Oscillator 8"]
    #[inline(always)]
    pub fn is_division_factor_4_rc_oscillator_8(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_4_RC_OSCILLATOR_8
    }
    #[doc = "Division factor 8 / RC-Oscillator 16"]
    #[inline(always)]
    pub fn is_division_factor_8_rc_oscillator_16(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_8_RC_OSCILLATOR_16
    }
    #[doc = "Division factor 16 / RC-Oscillator 32"]
    #[inline(always)]
    pub fn is_division_factor_16_rc_oscillator_32(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_16_RC_OSCILLATOR_32
    }
    #[doc = "Division factor 32 / RC-Oscillator 64"]
    #[inline(always)]
    pub fn is_division_factor_32_rc_oscillator_64(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_32_RC_OSCILLATOR_64
    }
    #[doc = "Division factor 64 / RC-Oscillator 128"]
    #[inline(always)]
    pub fn is_division_factor_64_rc_oscillator_128(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_64_RC_OSCILLATOR_128
    }
    #[doc = "Division factor 128 / RC-Oscillator 256"]
    #[inline(always)]
    pub fn is_division_factor_128_rc_oscillator_256(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_128_RC_OSCILLATOR_256
    }
    #[doc = "Division factor 256 / RC-Oscillator 512"]
    #[inline(always)]
    pub fn is_division_factor_256_rc_oscillator_512(&self) -> bool {
        *self == CLKPS_A::DIVISION_FACTOR_256_RC_OSCILLATOR_512
    }
    #[doc = "Division factor 1 only permitted for RC-Oscillator. Flash and EEPROM programming is not allowed."]
    #[inline(always)]
    pub fn is_division_factor_1_only_permitted_for_rc_oscillator_flash_and_eeprom_programming_is_not_allowed(
        &self,
    ) -> bool {
        * self == CLKPS_A :: DIVISION_FACTOR_1_ONLY_PERMITTED_FOR_RC_OSCILLATOR_FLASH_AND_EEPROM_PROGRAMMING_IS_NOT_ALLOWED
    }
}
#[doc = "Field `CLKPS` writer - Clock Prescaler Select Bits"]
pub type CLKPS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CLKPS_A>;
impl<'a, REG> CLKPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Division factor 1 / RC-Oscillator 2"]
    #[inline(always)]
    pub fn division_factor_1_rc_oscillator_2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::DIVISION_FACTOR_1_RC_OSCILLATOR_2)
    }
    #[doc = "Division factor 2 / RC-Oscillator 4"]
    #[inline(always)]
    pub fn division_factor_2_rc_oscillator_4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::DIVISION_FACTOR_2_RC_OSCILLATOR_4)
    }
    #[doc = "Division factor 4 / RC-Oscillator 8"]
    #[inline(always)]
    pub fn division_factor_4_rc_oscillator_8(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::DIVISION_FACTOR_4_RC_OSCILLATOR_8)
    }
    #[doc = "Division factor 8 / RC-Oscillator 16"]
    #[inline(always)]
    pub fn division_factor_8_rc_oscillator_16(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::DIVISION_FACTOR_8_RC_OSCILLATOR_16)
    }
    #[doc = "Division factor 16 / RC-Oscillator 32"]
    #[inline(always)]
    pub fn division_factor_16_rc_oscillator_32(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::DIVISION_FACTOR_16_RC_OSCILLATOR_32)
    }
    #[doc = "Division factor 32 / RC-Oscillator 64"]
    #[inline(always)]
    pub fn division_factor_32_rc_oscillator_64(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::DIVISION_FACTOR_32_RC_OSCILLATOR_64)
    }
    #[doc = "Division factor 64 / RC-Oscillator 128"]
    #[inline(always)]
    pub fn division_factor_64_rc_oscillator_128(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::DIVISION_FACTOR_64_RC_OSCILLATOR_128)
    }
    #[doc = "Division factor 128 / RC-Oscillator 256"]
    #[inline(always)]
    pub fn division_factor_128_rc_oscillator_256(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::DIVISION_FACTOR_128_RC_OSCILLATOR_256)
    }
    #[doc = "Division factor 256 / RC-Oscillator 512"]
    #[inline(always)]
    pub fn division_factor_256_rc_oscillator_512(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPS_A::DIVISION_FACTOR_256_RC_OSCILLATOR_512)
    }
    #[doc = "Division factor 1 only permitted for RC-Oscillator. Flash and EEPROM programming is not allowed."]
    #[inline(always)]
    pub fn division_factor_1_only_permitted_for_rc_oscillator_flash_and_eeprom_programming_is_not_allowed(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (CLKPS_A :: DIVISION_FACTOR_1_ONLY_PERMITTED_FOR_RC_OSCILLATOR_FLASH_AND_EEPROM_PROGRAMMING_IS_NOT_ALLOWED)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `CLKPCE` reader - Clock Prescaler Change Enable"]
pub type CLKPCE_R = crate::BitReader;
#[doc = "Field `CLKPCE` writer - Clock Prescaler Change Enable"]
pub type CLKPCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Clock Prescaler Select Bits"]
    #[inline(always)]
    pub fn clkps(&self) -> CLKPS_R {
        CLKPS_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - Clock Prescaler Change Enable"]
    #[inline(always)]
    pub fn clkpce(&self) -> CLKPCE_R {
        CLKPCE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Prescaler Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn clkps(&mut self) -> CLKPS_W<CLKPR_SPEC> {
        CLKPS_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<CLKPR_SPEC> {
        RES_W::new(self, 4)
    }
    #[doc = "Bit 7 - Clock Prescaler Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkpce(&mut self) -> CLKPCE_W<CLKPR_SPEC> {
        CLKPCE_W::new(self, 7)
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
#[doc = "Clock Prescale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKPR_SPEC;
impl crate::RegisterSpec for CLKPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clkpr::R`](R) reader structure"]
impl crate::Readable for CLKPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkpr::W`](W) writer structure"]
impl crate::Writable for CLKPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKPR to value 0"]
impl crate::Resettable for CLKPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
