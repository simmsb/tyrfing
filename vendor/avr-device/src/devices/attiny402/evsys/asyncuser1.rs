#[doc = "Register `ASYNCUSER1` reader"]
pub type R = crate::R<ASYNCUSER1_SPEC>;
#[doc = "Register `ASYNCUSER1` writer"]
pub type W = crate::W<ASYNCUSER1_SPEC>;
#[doc = "Field `ASYNCUSER1` reader - Asynchronous User Ch 1 Input Selection - ADC0"]
pub type ASYNCUSER1_R = crate::FieldReader<ASYNCUSER1_A>;
#[doc = "Asynchronous User Ch 1 Input Selection - ADC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASYNCUSER1_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Synchronous Event Channel 0"]
    SYNCCH0 = 1,
    #[doc = "3: Asynchronous Event Channel 0"]
    ASYNCCH0 = 3,
    #[doc = "4: Asynchronous Event Channel 1"]
    ASYNCCH1 = 4,
}
impl From<ASYNCUSER1_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCUSER1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ASYNCUSER1_A {
    type Ux = u8;
}
impl ASYNCUSER1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ASYNCUSER1_A> {
        match self.bits {
            0 => Some(ASYNCUSER1_A::OFF),
            1 => Some(ASYNCUSER1_A::SYNCCH0),
            3 => Some(ASYNCUSER1_A::ASYNCCH0),
            4 => Some(ASYNCUSER1_A::ASYNCCH1),
            _ => None,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCUSER1_A::OFF
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn is_syncch0(&self) -> bool {
        *self == ASYNCUSER1_A::SYNCCH0
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn is_asyncch0(&self) -> bool {
        *self == ASYNCUSER1_A::ASYNCCH0
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn is_asyncch1(&self) -> bool {
        *self == ASYNCUSER1_A::ASYNCCH1
    }
}
#[doc = "Field `ASYNCUSER1` writer - Asynchronous User Ch 1 Input Selection - ADC0"]
pub type ASYNCUSER1_W<'a, REG> = crate::FieldWriter<'a, REG, 8, ASYNCUSER1_A>;
impl<'a, REG> ASYNCUSER1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER1_A::OFF)
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn syncch0(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER1_A::SYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn asyncch0(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER1_A::ASYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn asyncch1(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER1_A::ASYNCCH1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous User Ch 1 Input Selection - ADC0"]
    #[inline(always)]
    pub fn asyncuser1(&self) -> ASYNCUSER1_R {
        ASYNCUSER1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous User Ch 1 Input Selection - ADC0"]
    #[inline(always)]
    #[must_use]
    pub fn asyncuser1(&mut self) -> ASYNCUSER1_W<ASYNCUSER1_SPEC> {
        ASYNCUSER1_W::new(self, 0)
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
#[doc = "Asynchronous User Ch 1 Input Selection - ADC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASYNCUSER1_SPEC;
impl crate::RegisterSpec for ASYNCUSER1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`asyncuser1::R`](R) reader structure"]
impl crate::Readable for ASYNCUSER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asyncuser1::W`](W) writer structure"]
impl crate::Writable for ASYNCUSER1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCUSER1 to value 0"]
impl crate::Resettable for ASYNCUSER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
