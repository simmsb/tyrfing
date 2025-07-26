#[doc = "Register `USERTCB0` reader"]
pub type R = crate::R<USERTCB0_SPEC>;
#[doc = "Register `USERTCB0` writer"]
pub type W = crate::W<USERTCB0_SPEC>;
#[doc = "Field `CHANNEL` reader - Channel selector"]
pub type CHANNEL_R = crate::FieldReader<CHANNEL_A>;
#[doc = "Channel selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHANNEL_A {
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
impl From<CHANNEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHANNEL_A {
    type Ux = u8;
}
impl CHANNEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CHANNEL_A> {
        match self.bits {
            0 => Some(CHANNEL_A::OFF),
            1 => Some(CHANNEL_A::CHANNEL0),
            2 => Some(CHANNEL_A::CHANNEL1),
            3 => Some(CHANNEL_A::CHANNEL2),
            4 => Some(CHANNEL_A::CHANNEL3),
            5 => Some(CHANNEL_A::CHANNEL4),
            6 => Some(CHANNEL_A::CHANNEL5),
            _ => None,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CHANNEL_A::OFF
    }
    #[doc = "Connect user to event channel 0"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == CHANNEL_A::CHANNEL0
    }
    #[doc = "Connect user to event channel 1"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == CHANNEL_A::CHANNEL1
    }
    #[doc = "Connect user to event channel 2"]
    #[inline(always)]
    pub fn is_channel2(&self) -> bool {
        *self == CHANNEL_A::CHANNEL2
    }
    #[doc = "Connect user to event channel 3"]
    #[inline(always)]
    pub fn is_channel3(&self) -> bool {
        *self == CHANNEL_A::CHANNEL3
    }
    #[doc = "Connect user to event channel 4"]
    #[inline(always)]
    pub fn is_channel4(&self) -> bool {
        *self == CHANNEL_A::CHANNEL4
    }
    #[doc = "Connect user to event channel 5"]
    #[inline(always)]
    pub fn is_channel5(&self) -> bool {
        *self == CHANNEL_A::CHANNEL5
    }
}
#[doc = "Field `CHANNEL` writer - Channel selector"]
pub type CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, CHANNEL_A>;
impl<'a, REG> CHANNEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::OFF)
    }
    #[doc = "Connect user to event channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::CHANNEL0)
    }
    #[doc = "Connect user to event channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::CHANNEL1)
    }
    #[doc = "Connect user to event channel 2"]
    #[inline(always)]
    pub fn channel2(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::CHANNEL2)
    }
    #[doc = "Connect user to event channel 3"]
    #[inline(always)]
    pub fn channel3(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::CHANNEL3)
    }
    #[doc = "Connect user to event channel 4"]
    #[inline(always)]
    pub fn channel4(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::CHANNEL4)
    }
    #[doc = "Connect user to event channel 5"]
    #[inline(always)]
    pub fn channel5(self) -> &'a mut crate::W<REG> {
        self.variant(CHANNEL_A::CHANNEL5)
    }
}
impl R {
    #[doc = "Bits 0:7 - Channel selector"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel selector"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self) -> CHANNEL_W<USERTCB0_SPEC> {
        CHANNEL_W::new(self, 0)
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
#[doc = "User TCB0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usertcb0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usertcb0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USERTCB0_SPEC;
impl crate::RegisterSpec for USERTCB0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usertcb0::R`](R) reader structure"]
impl crate::Readable for USERTCB0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usertcb0::W`](W) writer structure"]
impl crate::Writable for USERTCB0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USERTCB0 to value 0"]
impl crate::Resettable for USERTCB0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
