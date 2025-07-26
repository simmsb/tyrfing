#[doc = "Register `CTRLD` reader"]
pub type R = crate::R<CTRLD_SPEC>;
#[doc = "Register `CTRLD` writer"]
pub type W = crate::W<CTRLD_SPEC>;
#[doc = "Field `TCB0` reader - Port Multiplexer TCB0"]
pub type TCB0_R = crate::BitReader<TCB0_A>;
#[doc = "Port Multiplexer TCB0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCB0_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
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
    pub const fn variant(&self) -> TCB0_A {
        match self.bits {
            false => TCB0_A::DEFAULT,
            true => TCB0_A::ALTERNATE,
        }
    }
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCB0_A::DEFAULT
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCB0_A::ALTERNATE
    }
}
#[doc = "Field `TCB0` writer - Port Multiplexer TCB0"]
pub type TCB0_W<'a, REG> = crate::BitWriter<'a, REG, TCB0_A>;
impl<'a, REG> TCB0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(TCB0_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(TCB0_A::ALTERNATE)
    }
}
#[doc = "Field `TCB1` reader - Port Multiplexer TCB1"]
pub type TCB1_R = crate::BitReader<TCB1_A>;
#[doc = "Port Multiplexer TCB1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCB1_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
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
    pub const fn variant(&self) -> TCB1_A {
        match self.bits {
            false => TCB1_A::DEFAULT,
            true => TCB1_A::ALTERNATE,
        }
    }
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCB1_A::DEFAULT
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCB1_A::ALTERNATE
    }
}
#[doc = "Field `TCB1` writer - Port Multiplexer TCB1"]
pub type TCB1_W<'a, REG> = crate::BitWriter<'a, REG, TCB1_A>;
impl<'a, REG> TCB1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(TCB1_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(TCB1_A::ALTERNATE)
    }
}
impl R {
    #[doc = "Bit 0 - Port Multiplexer TCB0"]
    #[inline(always)]
    pub fn tcb0(&self) -> TCB0_R {
        TCB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Multiplexer TCB1"]
    #[inline(always)]
    pub fn tcb1(&self) -> TCB1_R {
        TCB1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Multiplexer TCB0"]
    #[inline(always)]
    #[must_use]
    pub fn tcb0(&mut self) -> TCB0_W<CTRLD_SPEC> {
        TCB0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port Multiplexer TCB1"]
    #[inline(always)]
    #[must_use]
    pub fn tcb1(&mut self) -> TCB1_W<CTRLD_SPEC> {
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
#[doc = "Port Multiplexer Control D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLD_SPEC;
impl crate::RegisterSpec for CTRLD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrld::R`](R) reader structure"]
impl crate::Readable for CTRLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrld::W`](W) writer structure"]
impl crate::Writable for CTRLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::Resettable for CTRLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
