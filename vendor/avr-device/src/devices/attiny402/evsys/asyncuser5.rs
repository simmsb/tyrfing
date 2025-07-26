#[doc = "Register `ASYNCUSER5` reader"]
pub type R = crate::R<ASYNCUSER5_SPEC>;
#[doc = "Register `ASYNCUSER5` writer"]
pub type W = crate::W<ASYNCUSER5_SPEC>;
#[doc = "Field `ASYNCUSER5` reader - Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1"]
pub type ASYNCUSER5_R = crate::FieldReader<ASYNCUSER5_A>;
#[doc = "Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASYNCUSER5_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Synchronous Event Channel 0"]
    SYNCCH0 = 1,
    #[doc = "3: Asynchronous Event Channel 0"]
    ASYNCCH0 = 3,
    #[doc = "4: Asynchronous Event Channel 1"]
    ASYNCCH1 = 4,
}
impl From<ASYNCUSER5_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCUSER5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ASYNCUSER5_A {
    type Ux = u8;
}
impl ASYNCUSER5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ASYNCUSER5_A> {
        match self.bits {
            0 => Some(ASYNCUSER5_A::OFF),
            1 => Some(ASYNCUSER5_A::SYNCCH0),
            3 => Some(ASYNCUSER5_A::ASYNCCH0),
            4 => Some(ASYNCUSER5_A::ASYNCCH1),
            _ => None,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCUSER5_A::OFF
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn is_syncch0(&self) -> bool {
        *self == ASYNCUSER5_A::SYNCCH0
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn is_asyncch0(&self) -> bool {
        *self == ASYNCUSER5_A::ASYNCCH0
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn is_asyncch1(&self) -> bool {
        *self == ASYNCUSER5_A::ASYNCCH1
    }
}
#[doc = "Field `ASYNCUSER5` writer - Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1"]
pub type ASYNCUSER5_W<'a, REG> = crate::FieldWriter<'a, REG, 8, ASYNCUSER5_A>;
impl<'a, REG> ASYNCUSER5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER5_A::OFF)
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn syncch0(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER5_A::SYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn asyncch0(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER5_A::ASYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn asyncch1(self) -> &'a mut crate::W<REG> {
        self.variant(ASYNCUSER5_A::ASYNCCH1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1"]
    #[inline(always)]
    pub fn asyncuser5(&self) -> ASYNCUSER5_R {
        ASYNCUSER5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1"]
    #[inline(always)]
    #[must_use]
    pub fn asyncuser5(&mut self) -> ASYNCUSER5_W<ASYNCUSER5_SPEC> {
        ASYNCUSER5_W::new(self, 0)
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
#[doc = "Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asyncuser5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asyncuser5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASYNCUSER5_SPEC;
impl crate::RegisterSpec for ASYNCUSER5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`asyncuser5::R`](R) reader structure"]
impl crate::Readable for ASYNCUSER5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`asyncuser5::W`](W) writer structure"]
impl crate::Writable for ASYNCUSER5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ASYNCUSER5 to value 0"]
impl crate::Resettable for ASYNCUSER5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
