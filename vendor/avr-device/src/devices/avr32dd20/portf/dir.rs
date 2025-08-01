#[doc = "Register `DIR` reader"]
pub type R = crate::R<DIR_SPEC>;
#[doc = "Register `DIR` writer"]
pub type W = crate::W<DIR_SPEC>;
#[doc = "Field `PF6` reader - Pin F6"]
pub type PF6_R = crate::BitReader<PF6_A>;
#[doc = "Pin F6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PF6_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<PF6_A> for bool {
    #[inline(always)]
    fn from(variant: PF6_A) -> Self {
        variant as u8 != 0
    }
}
impl PF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PF6_A {
        match self.bits {
            false => PF6_A::INPUT,
            true => PF6_A::OUTPUT,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PF6_A::INPUT
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PF6_A::OUTPUT
    }
}
#[doc = "Field `PF6` writer - Pin F6"]
pub type PF6_W<'a, REG> = crate::BitWriter<'a, REG, PF6_A>;
impl<'a, REG> PF6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PF6_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PF6_A::OUTPUT)
    }
}
#[doc = "Field `PF7` reader - Pin F7"]
pub use PF6_R as PF7_R;
#[doc = "Field `PF7` writer - Pin F7"]
pub use PF6_W as PF7_W;
impl R {
    #[doc = "Bit 6 - Pin F6"]
    #[inline(always)]
    pub fn pf6(&self) -> PF6_R {
        PF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin F7"]
    #[inline(always)]
    pub fn pf7(&self) -> PF7_R {
        PF7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Pin F6"]
    #[inline(always)]
    #[must_use]
    pub fn pf6(&mut self) -> PF6_W<DIR_SPEC> {
        PF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin F7"]
    #[inline(always)]
    #[must_use]
    pub fn pf7(&mut self) -> PF7_W<DIR_SPEC> {
        PF7_W::new(self, 7)
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
#[doc = "Data Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIR_SPEC;
impl crate::RegisterSpec for DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dir::R`](R) reader structure"]
impl crate::Readable for DIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dir::W`](W) writer structure"]
impl crate::Writable for DIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIR to value 0"]
impl crate::Resettable for DIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
