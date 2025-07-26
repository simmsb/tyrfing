#[doc = "Register `USEREVSYSEVOUTC` reader"]
pub type R = crate::R<USEREVSYSEVOUTC_SPEC>;
#[doc = "Register `USEREVSYSEVOUTC` writer"]
pub type W = crate::W<USEREVSYSEVOUTC_SPEC>;
#[doc = "Field `USER` reader - User channel select"]
pub type USER_R = crate::FieldReader<USER_A>;
#[doc = "User channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Connect user to event channel 0"]
    CHANNEL0 = 1,
    #[doc = "2: Connect user to event channel 1"]
    CHANNEL1 = 2,
    #[doc = "3: Connect user to event channel 2"]
    CHANNEL2 = 3,
    #[doc = "4: Connect user to event channel 3"]
    CHANNEL3 = 4,
    #[doc = "5: Connect user to event channel 4"]
    CHANNEL4 = 5,
    #[doc = "6: Connect user to event channel 5"]
    CHANNEL5 = 6,
}
impl From<USER_A> for u8 {
    #[inline(always)]
    fn from(variant: USER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USER_A {
    type Ux = u8;
}
impl USER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USER_A> {
        match self.bits {
            0 => Some(USER_A::OFF),
            1 => Some(USER_A::CHANNEL0),
            2 => Some(USER_A::CHANNEL1),
            3 => Some(USER_A::CHANNEL2),
            4 => Some(USER_A::CHANNEL3),
            5 => Some(USER_A::CHANNEL4),
            6 => Some(USER_A::CHANNEL5),
            _ => None,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == USER_A::OFF
    }
    #[doc = "Connect user to event channel 0"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == USER_A::CHANNEL0
    }
    #[doc = "Connect user to event channel 1"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == USER_A::CHANNEL1
    }
    #[doc = "Connect user to event channel 2"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == USER_A::CHANNEL2
    }
    #[doc = "Connect user to event channel 3"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == USER_A::CHANNEL3
    }
    #[doc = "Connect user to event channel 4"]
    #[inline(always)]
    pub fn is_channel4(&self) -> bool {
        *self == USER_A::CHANNEL4
    }
    #[doc = "Connect user to event channel 5"]
    #[inline(always)]
    pub fn is_channel5(&self) -> bool {
        *self == USER_A::CHANNEL5
    }
}
#[doc = "Field `USER` writer - User channel select"]
pub type USER_W<'a, REG> = crate::FieldWriter<'a, REG, 8, USER_A>;
impl<'a, REG> USER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(USER_A::OFF)
    }
    #[doc = "Connect user to event channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(USER_A::CHANNEL0)
    }
    #[doc = "Connect user to event channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(USER_A::CHANNEL1)
    }
    #[doc = "Connect user to event channel 2"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut crate::W<REG> {
        self.variant(USER_A::CHANNEL2)
    }
    #[doc = "Connect user to event channel 3"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut crate::W<REG> {
        self.variant(USER_A::CHANNEL3)
    }
    #[doc = "Connect user to event channel 4"]
    #[inline(always)]
    pub fn channel4(self) -> &'a mut crate::W<REG> {
        self.variant(USER_A::CHANNEL4)
    }
    #[doc = "Connect user to event channel 5"]
    #[inline(always)]
    pub fn channel5(self) -> &'a mut crate::W<REG> {
        self.variant(USER_A::CHANNEL5)
    }
}
impl R {
    #[doc = "Bits 0:7 - User channel select"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - User channel select"]
    #[inline(always)]
    #[must_use]
    pub fn user(&mut self) -> USER_W<USEREVSYSEVOUTC_SPEC> {
        USER_W::new(self, 0)
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
#[doc = "User 15 - EVOUTC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`userevsysevoutc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`userevsysevoutc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USEREVSYSEVOUTC_SPEC;
impl crate::RegisterSpec for USEREVSYSEVOUTC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`userevsysevoutc::R`](R) reader structure"]
impl crate::Readable for USEREVSYSEVOUTC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`userevsysevoutc::W`](W) writer structure"]
impl crate::Writable for USEREVSYSEVOUTC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USEREVSYSEVOUTC to value 0"]
impl crate::Resettable for USEREVSYSEVOUTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
