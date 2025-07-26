#[doc = "Register `TCBROUTEA` reader"]
pub type R = crate::R<TCBROUTEA_SPEC>;
#[doc = "Register `TCBROUTEA` writer"]
pub type W = crate::W<TCBROUTEA_SPEC>;
#[doc = "Field `TCB0` reader - TCB0 Output"]
pub type TCB0_R = crate::BitReader<TCB0_A>;
#[doc = "TCB0 Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCB0_A {
    #[doc = "0: WO: PA2"]
    DEFAULT = 0,
}
impl From<TCB0_A> for bool {
    #[inline(always)]
    fn from(variant: TCB0_A) -> Self {
        variant as u8 != 0
    }
}
impl TCB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCB0_A> {
        match self.bits {
            false => Some(TCB0_A::DEFAULT),
            _ => None,
        }
    }
    #[doc = "WO: PA2"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCB0_A::DEFAULT
    }
}
#[doc = "Field `TCB0` writer - TCB0 Output"]
pub type TCB0_W<'a, REG> = crate::BitWriter<'a, REG, TCB0_A>;
impl<'a, REG> TCB0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WO: PA2"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(TCB0_A::DEFAULT)
    }
}
#[doc = "Field `TCB1` reader - TCB1 Output"]
pub type TCB1_R = crate::BitReader<TCB1_A>;
#[doc = "TCB1 Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCB1_A {
    #[doc = "0: WO: PA3"]
    DEFAULT = 0,
}
impl From<TCB1_A> for bool {
    #[inline(always)]
    fn from(variant: TCB1_A) -> Self {
        variant as u8 != 0
    }
}
impl TCB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCB1_A> {
        match self.bits {
            false => Some(TCB1_A::DEFAULT),
            _ => None,
        }
    }
    #[doc = "WO: PA3"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCB1_A::DEFAULT
    }
}
#[doc = "Field `TCB1` writer - TCB1 Output"]
pub type TCB1_W<'a, REG> = crate::BitWriter<'a, REG, TCB1_A>;
impl<'a, REG> TCB1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WO: PA3"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(TCB1_A::DEFAULT)
    }
}
impl R {
    #[doc = "Bit 0 - TCB0 Output"]
    #[inline(always)]
    pub fn tcb0(&self) -> TCB0_R {
        TCB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TCB1 Output"]
    #[inline(always)]
    pub fn tcb1(&self) -> TCB1_R {
        TCB1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TCB0 Output"]
    #[inline(always)]
    #[must_use]
    pub fn tcb0(&mut self) -> TCB0_W<TCBROUTEA_SPEC> {
        TCB0_W::new(self, 0)
    }
    #[doc = "Bit 1 - TCB1 Output"]
    #[inline(always)]
    #[must_use]
    pub fn tcb1(&mut self) -> TCB1_W<TCBROUTEA_SPEC> {
        TCB1_W::new(self, 1)
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
#[doc = "TCB route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcbroutea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcbroutea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCBROUTEA_SPEC;
impl crate::RegisterSpec for TCBROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tcbroutea::R`](R) reader structure"]
impl crate::Readable for TCBROUTEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcbroutea::W`](W) writer structure"]
impl crate::Writable for TCBROUTEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCBROUTEA to value 0"]
impl crate::Resettable for TCBROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
