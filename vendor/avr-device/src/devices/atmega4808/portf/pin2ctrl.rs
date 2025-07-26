#[doc = "Register `PIN2CTRL` reader"]
pub type R = crate::R<PIN2CTRL_SPEC>;
#[doc = "Register `PIN2CTRL` writer"]
pub type W = crate::W<PIN2CTRL_SPEC>;
#[doc = "Field `ISC` reader - Input/Sense Configuration"]
pub type ISC_R = crate::FieldReader<ISC_A>;
#[doc = "Input/Sense Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISC_A {
    #[doc = "0: Interrupt disabled but input buffer enabled"]
    INTDISABLE = 0,
    #[doc = "1: Sense Both Edges"]
    BOTHEDGES = 1,
    #[doc = "2: Sense Rising Edge"]
    RISING = 2,
    #[doc = "3: Sense Falling Edge"]
    FALLING = 3,
    #[doc = "4: Digital Input Buffer disabled"]
    INPUT_DISABLE = 4,
    #[doc = "5: Sense low Level"]
    LEVEL = 5,
}
impl From<ISC_A> for u8 {
    #[inline(always)]
    fn from(variant: ISC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ISC_A {
    type Ux = u8;
}
impl ISC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ISC_A> {
        match self.bits {
            0 => Some(ISC_A::INTDISABLE),
            1 => Some(ISC_A::BOTHEDGES),
            2 => Some(ISC_A::RISING),
            3 => Some(ISC_A::FALLING),
            4 => Some(ISC_A::INPUT_DISABLE),
            5 => Some(ISC_A::LEVEL),
            _ => None,
        }
    }
    #[doc = "Interrupt disabled but input buffer enabled"]
    #[inline(always)]
    pub fn is_intdisable(&self) -> bool {
        *self == ISC_A::INTDISABLE
    }
    #[doc = "Sense Both Edges"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == ISC_A::BOTHEDGES
    }
    #[doc = "Sense Rising Edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ISC_A::RISING
    }
    #[doc = "Sense Falling Edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ISC_A::FALLING
    }
    #[doc = "Digital Input Buffer disabled"]
    #[inline(always)]
    pub fn is_input_disable(&self) -> bool {
        *self == ISC_A::INPUT_DISABLE
    }
    #[doc = "Sense low Level"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISC_A::LEVEL
    }
}
#[doc = "Field `ISC` writer - Input/Sense Configuration"]
pub type ISC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ISC_A>;
impl<'a, REG> ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt disabled but input buffer enabled"]
    #[inline(always)]
    pub fn intdisable(self) -> &'a mut crate::W<REG> {
        self.variant(ISC_A::INTDISABLE)
    }
    #[doc = "Sense Both Edges"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut crate::W<REG> {
        self.variant(ISC_A::BOTHEDGES)
    }
    #[doc = "Sense Rising Edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(ISC_A::RISING)
    }
    #[doc = "Sense Falling Edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(ISC_A::FALLING)
    }
    #[doc = "Digital Input Buffer disabled"]
    #[inline(always)]
    pub fn input_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ISC_A::INPUT_DISABLE)
    }
    #[doc = "Sense low Level"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(ISC_A::LEVEL)
    }
}
#[doc = "Field `PULLUPEN` reader - Pullup enable"]
pub type PULLUPEN_R = crate::BitReader;
#[doc = "Field `PULLUPEN` writer - Pullup enable"]
pub type PULLUPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEN` reader - Inverted I/O Enable"]
pub type INVEN_R = crate::BitReader;
#[doc = "Field `INVEN` writer - Inverted I/O Enable"]
pub type INVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Input/Sense Configuration"]
    #[inline(always)]
    pub fn isc(&self) -> ISC_R {
        ISC_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Pullup enable"]
    #[inline(always)]
    pub fn pullupen(&self) -> PULLUPEN_R {
        PULLUPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Inverted I/O Enable"]
    #[inline(always)]
    pub fn inven(&self) -> INVEN_R {
        INVEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input/Sense Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn isc(&mut self) -> ISC_W<PIN2CTRL_SPEC> {
        ISC_W::new(self, 0)
    }
    #[doc = "Bit 3 - Pullup enable"]
    #[inline(always)]
    #[must_use]
    pub fn pullupen(&mut self) -> PULLUPEN_W<PIN2CTRL_SPEC> {
        PULLUPEN_W::new(self, 3)
    }
    #[doc = "Bit 7 - Inverted I/O Enable"]
    #[inline(always)]
    #[must_use]
    pub fn inven(&mut self) -> INVEN_W<PIN2CTRL_SPEC> {
        INVEN_W::new(self, 7)
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
#[doc = "Pin 2 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin2ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin2ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN2CTRL_SPEC;
impl crate::RegisterSpec for PIN2CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pin2ctrl::R`](R) reader structure"]
impl crate::Readable for PIN2CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin2ctrl::W`](W) writer structure"]
impl crate::Writable for PIN2CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIN2CTRL to value 0"]
impl crate::Resettable for PIN2CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
