#[doc = "Register `DIR` reader"]
pub type R = crate::R<DIR_SPEC>;
#[doc = "Register `DIR` writer"]
pub type W = crate::W<DIR_SPEC>;
#[doc = "Field `PA0` reader - Pin A0"]
pub type PA0_R = crate::BitReader<PA0_A>;
#[doc = "Pin A0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PA0_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<PA0_A> for bool {
    #[inline(always)]
    fn from(variant: PA0_A) -> Self {
        variant as u8 != 0
    }
}
impl PA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PA0_A {
        match self.bits {
            false => PA0_A::INPUT,
            true => PA0_A::OUTPUT,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PA0_A::INPUT
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PA0_A::OUTPUT
    }
}
#[doc = "Field `PA0` writer - Pin A0"]
pub type PA0_W<'a, REG> = crate::BitWriter<'a, REG, PA0_A>;
impl<'a, REG> PA0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PA0_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PA0_A::OUTPUT)
    }
}
#[doc = "Field `PA1` reader - Pin A1"]
pub use PA0_R as PA1_R;
#[doc = "Field `PA2` reader - Pin A2"]
pub use PA0_R as PA2_R;
#[doc = "Field `PA3` reader - Pin A3"]
pub use PA0_R as PA3_R;
#[doc = "Field `PA4` reader - Pin A4"]
pub use PA0_R as PA4_R;
#[doc = "Field `PA5` reader - Pin A5"]
pub use PA0_R as PA5_R;
#[doc = "Field `PA6` reader - Pin A6"]
pub use PA0_R as PA6_R;
#[doc = "Field `PA7` reader - Pin A7"]
pub use PA0_R as PA7_R;
#[doc = "Field `PA1` writer - Pin A1"]
pub use PA0_W as PA1_W;
#[doc = "Field `PA2` writer - Pin A2"]
pub use PA0_W as PA2_W;
#[doc = "Field `PA3` writer - Pin A3"]
pub use PA0_W as PA3_W;
#[doc = "Field `PA4` writer - Pin A4"]
pub use PA0_W as PA4_W;
#[doc = "Field `PA5` writer - Pin A5"]
pub use PA0_W as PA5_W;
#[doc = "Field `PA6` writer - Pin A6"]
pub use PA0_W as PA6_W;
#[doc = "Field `PA7` writer - Pin A7"]
pub use PA0_W as PA7_W;
impl R {
    #[doc = "Bit 0 - Pin A0"]
    #[inline(always)]
    pub fn pa0(&self) -> PA0_R {
        PA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin A1"]
    #[inline(always)]
    pub fn pa1(&self) -> PA1_R {
        PA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin A2"]
    #[inline(always)]
    pub fn pa2(&self) -> PA2_R {
        PA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin A3"]
    #[inline(always)]
    pub fn pa3(&self) -> PA3_R {
        PA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin A4"]
    #[inline(always)]
    pub fn pa4(&self) -> PA4_R {
        PA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin A5"]
    #[inline(always)]
    pub fn pa5(&self) -> PA5_R {
        PA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin A6"]
    #[inline(always)]
    pub fn pa6(&self) -> PA6_R {
        PA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin A7"]
    #[inline(always)]
    pub fn pa7(&self) -> PA7_R {
        PA7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin A0"]
    #[inline(always)]
    #[must_use]
    pub fn pa0(&mut self) -> PA0_W<DIR_SPEC> {
        PA0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin A1"]
    #[inline(always)]
    #[must_use]
    pub fn pa1(&mut self) -> PA1_W<DIR_SPEC> {
        PA1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin A2"]
    #[inline(always)]
    #[must_use]
    pub fn pa2(&mut self) -> PA2_W<DIR_SPEC> {
        PA2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin A3"]
    #[inline(always)]
    #[must_use]
    pub fn pa3(&mut self) -> PA3_W<DIR_SPEC> {
        PA3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin A4"]
    #[inline(always)]
    #[must_use]
    pub fn pa4(&mut self) -> PA4_W<DIR_SPEC> {
        PA4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin A5"]
    #[inline(always)]
    #[must_use]
    pub fn pa5(&mut self) -> PA5_W<DIR_SPEC> {
        PA5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin A6"]
    #[inline(always)]
    #[must_use]
    pub fn pa6(&mut self) -> PA6_W<DIR_SPEC> {
        PA6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin A7"]
    #[inline(always)]
    #[must_use]
    pub fn pa7(&mut self) -> PA7_W<DIR_SPEC> {
        PA7_W::new(self, 7)
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
