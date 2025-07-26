#[doc = "Register `SYNCUSER1` reader"]
pub type R = crate::R<SYNCUSER1_SPEC>;
#[doc = "Register `SYNCUSER1` writer"]
pub type W = crate::W<SYNCUSER1_SPEC>;
#[doc = "Field `SYNCUSER1` reader - Synchronous User Ch 1 - USART0"]
pub type SYNCUSER1_R = crate::FieldReader<SYNCUSER1_A>;
#[doc = "Synchronous User Ch 1 - USART0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCUSER1_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Synchronous Event Channel 0"]
    SYNCCH0 = 1,
    #[doc = "2: Synchronous Event Channel 1"]
    SYNCCH1 = 2,
}
impl From<SYNCUSER1_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCUSER1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCUSER1_A {
    type Ux = u8;
}
impl SYNCUSER1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCUSER1_A> {
        match self.bits {
            0 => Some(SYNCUSER1_A::OFF),
            1 => Some(SYNCUSER1_A::SYNCCH0),
            2 => Some(SYNCUSER1_A::SYNCCH1),
            _ => None,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SYNCUSER1_A::OFF
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn is_syncch0(&self) -> bool {
        *self == SYNCUSER1_A::SYNCCH0
    }
    #[doc = "Synchronous Event Channel 1"]
    #[inline(always)]
    pub fn is_syncch1(&self) -> bool {
        *self == SYNCUSER1_A::SYNCCH1
    }
}
#[doc = "Field `SYNCUSER1` writer - Synchronous User Ch 1 - USART0"]
pub type SYNCUSER1_W<'a, REG> = crate::FieldWriter<'a, REG, 8, SYNCUSER1_A>;
impl<'a, REG> SYNCUSER1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCUSER1_A::OFF)
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn syncch0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCUSER1_A::SYNCCH0)
    }
    #[doc = "Synchronous Event Channel 1"]
    #[inline(always)]
    pub fn syncch1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCUSER1_A::SYNCCH1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Synchronous User Ch 1 - USART0"]
    #[inline(always)]
    pub fn syncuser1(&self) -> SYNCUSER1_R {
        SYNCUSER1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronous User Ch 1 - USART0"]
    #[inline(always)]
    #[must_use]
    pub fn syncuser1(&mut self) -> SYNCUSER1_W<SYNCUSER1_SPEC> {
        SYNCUSER1_W::new(self, 0)
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
#[doc = "Synchronous User Ch 1 - USART0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncuser1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncuser1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCUSER1_SPEC;
impl crate::RegisterSpec for SYNCUSER1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`syncuser1::R`](R) reader structure"]
impl crate::Readable for SYNCUSER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`syncuser1::W`](W) writer structure"]
impl crate::Writable for SYNCUSER1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCUSER1 to value 0"]
impl crate::Resettable for SYNCUSER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
