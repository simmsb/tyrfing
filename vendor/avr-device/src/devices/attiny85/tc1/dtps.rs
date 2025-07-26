#[doc = "Register `DTPS` reader"]
pub type R = crate::R<DTPS_SPEC>;
#[doc = "Register `DTPS` writer"]
pub type W = crate::W<DTPS_SPEC>;
#[doc = "Field `DTPS` reader - No Description."]
pub type DTPS_R = crate::FieldReader<DTPS_A>;
#[doc = "No Description.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTPS_A {
    #[doc = "0: No Prescaling"]
    DIRECT = 0,
    #[doc = "1: Division factor 2"]
    PRESCALE_2 = 1,
    #[doc = "2: Division factor 4"]
    PRESCALE_4 = 2,
    #[doc = "3: Division factor 8"]
    PRESCALE_8 = 3,
}
impl From<DTPS_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTPS_A {
    type Ux = u8;
}
impl DTPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTPS_A {
        match self.bits {
            0 => DTPS_A::DIRECT,
            1 => DTPS_A::PRESCALE_2,
            2 => DTPS_A::PRESCALE_4,
            3 => DTPS_A::PRESCALE_8,
            _ => unreachable!(),
        }
    }
    #[doc = "No Prescaling"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == DTPS_A::DIRECT
    }
    #[doc = "Division factor 2"]
    #[inline(always)]
    pub fn is_prescale_2(&self) -> bool {
        *self == DTPS_A::PRESCALE_2
    }
    #[doc = "Division factor 4"]
    #[inline(always)]
    pub fn is_prescale_4(&self) -> bool {
        *self == DTPS_A::PRESCALE_4
    }
    #[doc = "Division factor 8"]
    #[inline(always)]
    pub fn is_prescale_8(&self) -> bool {
        *self == DTPS_A::PRESCALE_8
    }
}
#[doc = "Field `DTPS` writer - No Description."]
pub type DTPS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DTPS_A>;
impl<'a, REG> DTPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Prescaling"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(DTPS_A::DIRECT)
    }
    #[doc = "Division factor 2"]
    #[inline(always)]
    pub fn prescale_2(self) -> &'a mut crate::W<REG> {
        self.variant(DTPS_A::PRESCALE_2)
    }
    #[doc = "Division factor 4"]
    #[inline(always)]
    pub fn prescale_4(self) -> &'a mut crate::W<REG> {
        self.variant(DTPS_A::PRESCALE_4)
    }
    #[doc = "Division factor 8"]
    #[inline(always)]
    pub fn prescale_8(self) -> &'a mut crate::W<REG> {
        self.variant(DTPS_A::PRESCALE_8)
    }
}
impl R {
    #[doc = "Bits 0:1 - No Description."]
    #[inline(always)]
    pub fn dtps(&self) -> DTPS_R {
        DTPS_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn dtps(&mut self) -> DTPS_W<DTPS_SPEC> {
        DTPS_W::new(self, 0)
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
#[doc = "Dead time prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTPS_SPEC;
impl crate::RegisterSpec for DTPS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dtps::R`](R) reader structure"]
impl crate::Readable for DTPS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtps::W`](W) writer structure"]
impl crate::Writable for DTPS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTPS to value 0"]
impl crate::Resettable for DTPS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
