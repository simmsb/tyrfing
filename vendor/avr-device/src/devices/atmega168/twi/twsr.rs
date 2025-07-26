#[doc = "Register `TWSR` reader"]
pub type R = crate::R<TWSR_SPEC>;
#[doc = "Register `TWSR` writer"]
pub type W = crate::W<TWSR_SPEC>;
#[doc = "Field `TWPS` reader - TWI Prescaler"]
pub type TWPS_R = crate::FieldReader<TWPS_A>;
#[doc = "TWI Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TWPS_A {
    #[doc = "0: Prescaler Value 1"]
    PRESCALER_1 = 0,
    #[doc = "1: Prescaler Value 4"]
    PRESCALER_4 = 1,
    #[doc = "2: Prescaler Value 16"]
    PRESCALER_16 = 2,
    #[doc = "3: Prescaler Value 64"]
    PRESCALER_64 = 3,
}
impl From<TWPS_A> for u8 {
    #[inline(always)]
    fn from(variant: TWPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TWPS_A {
    type Ux = u8;
}
impl TWPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TWPS_A {
        match self.bits {
            0 => TWPS_A::PRESCALER_1,
            1 => TWPS_A::PRESCALER_4,
            2 => TWPS_A::PRESCALER_16,
            3 => TWPS_A::PRESCALER_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler Value 1"]
    #[inline(always)]
    pub fn is_prescaler_1(&self) -> bool {
        *self == TWPS_A::PRESCALER_1
    }
    #[doc = "Prescaler Value 4"]
    #[inline(always)]
    pub fn is_prescaler_4(&self) -> bool {
        *self == TWPS_A::PRESCALER_4
    }
    #[doc = "Prescaler Value 16"]
    #[inline(always)]
    pub fn is_prescaler_16(&self) -> bool {
        *self == TWPS_A::PRESCALER_16
    }
    #[doc = "Prescaler Value 64"]
    #[inline(always)]
    pub fn is_prescaler_64(&self) -> bool {
        *self == TWPS_A::PRESCALER_64
    }
}
#[doc = "Field `TWPS` writer - TWI Prescaler"]
pub type TWPS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TWPS_A>;
impl<'a, REG> TWPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler Value 1"]
    #[inline(always)]
    pub fn prescaler_1(self) -> &'a mut crate::W<REG> {
        self.variant(TWPS_A::PRESCALER_1)
    }
    #[doc = "Prescaler Value 4"]
    #[inline(always)]
    pub fn prescaler_4(self) -> &'a mut crate::W<REG> {
        self.variant(TWPS_A::PRESCALER_4)
    }
    #[doc = "Prescaler Value 16"]
    #[inline(always)]
    pub fn prescaler_16(self) -> &'a mut crate::W<REG> {
        self.variant(TWPS_A::PRESCALER_16)
    }
    #[doc = "Prescaler Value 64"]
    #[inline(always)]
    pub fn prescaler_64(self) -> &'a mut crate::W<REG> {
        self.variant(TWPS_A::PRESCALER_64)
    }
}
#[doc = "Field `TWS` reader - TWI Status"]
pub type TWS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - TWI Prescaler"]
    #[inline(always)]
    pub fn twps(&self) -> TWPS_R {
        TWPS_R::new(self.bits & 3)
    }
    #[doc = "Bits 3:7 - TWI Status"]
    #[inline(always)]
    pub fn tws(&self) -> TWS_R {
        TWS_R::new((self.bits >> 3) & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:1 - TWI Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn twps(&mut self) -> TWPS_W<TWSR_SPEC> {
        TWPS_W::new(self, 0)
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
#[doc = "TWI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWSR_SPEC;
impl crate::RegisterSpec for TWSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twsr::R`](R) reader structure"]
impl crate::Readable for TWSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twsr::W`](W) writer structure"]
impl crate::Writable for TWSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWSR to value 0"]
impl crate::Resettable for TWSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
