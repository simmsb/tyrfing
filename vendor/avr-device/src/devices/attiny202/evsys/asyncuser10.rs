#[doc = "Register `ASYNCUSER10` reader"]
pub type R = crate::R<ASYNCUSER10_SPEC>;
#[doc = "Register `ASYNCUSER10` writer"]
pub type W = crate::W<ASYNCUSER10_SPEC>;
#[doc = "Field `ASYNCUSER10` reader - Asynchronous User Ch 10 Input Selection - Event Out 2"]
pub type ASYNCUSER10_R = crate::FieldReader<ASYNCUSER10_A>;
#[doc = "Asynchronous User Ch 10 Input Selection - Event Out 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASYNCUSER10_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Synchronous Event Channel 0"]
    SYNCCH0 = 1,
    #[doc = "3: Asynchronous Event Channel 0"]
    ASYNCCH0 = 3,
    #[doc = "4: Asynchronous Event Channel 1"]
    ASYNCCH1 = 4,
}
impl From<ASYNCUSER10_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCUSER10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ASYNCUSER10_A {
    type Ux = u8;
}
impl ASYNCUSER10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ASYNCUSER10_A> {
        match self.bits {
            0 => Some(ASYNCUSER10_A::OFF),
            1 => Some(ASYNCUSER10_A::SYNCCH0),
            3 => Some(ASYNCUSER10_A::ASYNCCH0),
            4 => Some(ASYNCUSER10_A::ASYNCCH1),
            _ => None,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCUSER10_A::OFF
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn is_syncch0(&self) -> bool {
        *self == ASYNCUSER10_A::SYNCCH0
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn is_asyncch0(&self) -> bool {
        *self == ASYNCUSER10_A::ASYNCCH0
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn is_asyncch1(&self) -> bool {
        *self == ASYNCUSER10_A::ASYNCCH1
    }
}
#[doc = "Field `ASYNCUSER10` writer - Asynchronous User Ch 10 Input Selection - Event Out 2"]
pub type ASYNCUSER10_W<'a, REG> = crate::FieldWriter<'a, REG, 8, ASYNCUSER10_A>;
impl<'a, REG> ASYNCUSER10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER10_A::OFF)
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn syncch0(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER10_A::SYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn asyncch0(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER10_A::ASYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn asyncch1(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER10_A::ASYNCCH1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous User Ch 10 Input Selection - Event Out 2"]
    #[inline(always)]
    pub fn asyncuser10(&self) -> ASYNCUSER10_R {
        ASYNCUSER10_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous User Ch 10 Input Selection - Event Out 2"]
    #[inline(always)]
    #[must_use]
    pub fn asyncuser10(&mut self) -> ASYNCUSER10_W<ASYNCUSER10_SPEC> {
        ASYNCUSER10_W::new(self, 0)
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
#[doc = "Asynchronous User Ch 10 Input Selection - Event Out 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASYNCUSER10_SPEC;
impl crate::RegisterSpec for ASYNCUSER10_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`asyncuser10::R`](R) reader structure"]
impl crate::Readable for ASYNCUSER10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asyncuser10::W`](W) writer structure"]
impl crate::Writable for ASYNCUSER10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCUSER10 to value 0"]
impl crate::Resettable for ASYNCUSER10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
