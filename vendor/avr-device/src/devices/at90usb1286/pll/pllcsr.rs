#[doc = "Register `PLLCSR` reader"]
pub type R = crate::R<PLLCSR_SPEC>;
#[doc = "Register `PLLCSR` writer"]
pub type W = crate::W<PLLCSR_SPEC>;
#[doc = "Field `PLOCK` reader - PLL Lock Status Bit"]
pub type PLOCK_R = crate::BitReader;
#[doc = "Field `PLOCK` writer - PLL Lock Status Bit"]
pub type PLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLE` reader - PLL Enable Bit"]
pub type PLLE_R = crate::BitReader;
#[doc = "Field `PLLE` writer - PLL Enable Bit"]
pub type PLLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLP` reader - PLL prescaler Bits"]
pub type PLLP_R = crate::FieldReader<PLLP_A>;
#[doc = "PLL prescaler Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLP_A {
    #[doc = "3: Clock/4"]
    VAL_0X03 = 3,
    #[doc = "5: Clock/8"]
    VAL_0X05 = 5,
}
impl From<PLLP_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLP_A {
    type Ux = u8;
}
impl PLLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLP_A> {
        match self.bits {
            3 => Some(PLLP_A::VAL_0X03),
            5 => Some(PLLP_A::VAL_0X05),
            _ => None,
        }
    }
    #[doc = "Clock/4"]
    #[inline(always)]
    pub fn is_val_0x03(&self) -> bool {
        *self == PLLP_A::VAL_0X03
    }
    #[doc = "Clock/8"]
    #[inline(always)]
    pub fn is_val_0x05(&self) -> bool {
        *self == PLLP_A::VAL_0X05
    }
}
#[doc = "Field `PLLP` writer - PLL prescaler Bits"]
pub type PLLP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PLLP_A>;
impl<'a, REG> PLLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock/4"]
    #[inline(always)]
    pub fn val_0x03(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP_A::VAL_0X03)
    }
    #[doc = "Clock/8"]
    #[inline(always)]
    pub fn val_0x05(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP_A::VAL_0X05)
    }
}
impl R {
    #[doc = "Bit 0 - PLL Lock Status Bit"]
    #[inline(always)]
    pub fn plock(&self) -> PLOCK_R {
        PLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Enable Bit"]
    #[inline(always)]
    pub fn plle(&self) -> PLLE_R {
        PLLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - PLL prescaler Bits"]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new((self.bits >> 2) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Lock Status Bit"]
    #[inline(always)]
    #[must_use]
    pub fn plock(&mut self) -> PLOCK_W<PLLCSR_SPEC> {
        PLOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - PLL Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn plle(&mut self) -> PLLE_W<PLLCSR_SPEC> {
        PLLE_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - PLL prescaler Bits"]
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<PLLCSR_SPEC> {
        PLLP_W::new(self, 2)
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
#[doc = "PLL Status and Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCSR_SPEC;
impl crate::RegisterSpec for PLLCSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pllcsr::R`](R) reader structure"]
impl crate::Readable for PLLCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllcsr::W`](W) writer structure"]
impl crate::Writable for PLLCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCSR to value 0"]
impl crate::Resettable for PLLCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
