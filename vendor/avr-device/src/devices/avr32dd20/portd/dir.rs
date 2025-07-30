#[doc = "Register `DIR` reader"]
pub type R = crate::R<DIR_SPEC>;
#[doc = "Register `DIR` writer"]
pub type W = crate::W<DIR_SPEC>;
#[doc = "Field `PD4` reader - Pin D4"]
pub type PD4_R = crate::BitReader<PD4_A>;
#[doc = "Pin D4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD4_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<PD4_A> for bool {
    #[inline(always)]
    fn from(variant: PD4_A) -> Self {
        variant as u8 != 0
    }
}
impl PD4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PD4_A {
        match self.bits {
            false => PD4_A::INPUT,
            true => PD4_A::OUTPUT,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PD4_A::INPUT
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PD4_A::OUTPUT
    }
}
#[doc = "Field `PD4` writer - Pin D4"]
pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG, PD4_A>;
impl<'a, REG> PD4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PD4_A::OUTPUT)
    }
}
#[doc = "Field `PD5` reader - Pin D5"]
pub use PD4_R as PD5_R;
#[doc = "Field `PD6` reader - Pin D6"]
pub use PD4_R as PD6_R;
#[doc = "Field `PD7` reader - Pin D7"]
pub use PD4_R as PD7_R;
#[doc = "Field `PD5` writer - Pin D5"]
pub use PD4_W as PD5_W;
#[doc = "Field `PD6` writer - Pin D6"]
pub use PD4_W as PD6_W;
#[doc = "Field `PD7` writer - Pin D7"]
pub use PD4_W as PD7_W;
impl R {
    #[doc = "Bit 4 - Pin D4"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin D5"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin D6"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin D7"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Pin D4"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<DIR_SPEC> {
        PD4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin D5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<DIR_SPEC> {
        PD5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin D6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<DIR_SPEC> {
        PD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin D7"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<DIR_SPEC> {
        PD7_W::new(self, 7)
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
