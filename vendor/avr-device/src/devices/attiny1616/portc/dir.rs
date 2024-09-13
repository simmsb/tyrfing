#[doc = "Register `DIR` reader"]
pub type R = crate::R<DIR_SPEC>;
#[doc = "Register `DIR` writer"]
pub type W = crate::W<DIR_SPEC>;
#[doc = "Field `PC0` reader - Pin C0"]
pub type PC0_R = crate::BitReader<PC0_A>;
#[doc = "Pin C0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PC0_A {
    #[doc = "0: Input"]
    INPUT = 0,
    #[doc = "1: Output"]
    OUTPUT = 1,
}
impl From<PC0_A> for bool {
    #[inline(always)]
    fn from(variant: PC0_A) -> Self {
        variant as u8 != 0
    }
}
impl PC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PC0_A {
        match self.bits {
            false => PC0_A::INPUT,
            true => PC0_A::OUTPUT,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PC0_A::INPUT
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PC0_A::OUTPUT
    }
}
#[doc = "Field `PC0` writer - Pin C0"]
pub type PC0_W<'a, REG> = crate::BitWriter<'a, REG, PC0_A>;
impl<'a, REG> PC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::INPUT)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::OUTPUT)
    }
}
#[doc = "Field `PC1` reader - Pin C1"]
pub use PC0_R as PC1_R;
#[doc = "Field `PC2` reader - Pin C2"]
pub use PC0_R as PC2_R;
#[doc = "Field `PC3` reader - Pin C3"]
pub use PC0_R as PC3_R;
#[doc = "Field `PC1` writer - Pin C1"]
pub use PC0_W as PC1_W;
#[doc = "Field `PC2` writer - Pin C2"]
pub use PC0_W as PC2_W;
#[doc = "Field `PC3` writer - Pin C3"]
pub use PC0_W as PC3_W;
impl R {
    #[doc = "Bit 0 - Pin C0"]
    #[inline(always)]
    pub fn pc0(&self) -> PC0_R {
        PC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin C1"]
    #[inline(always)]
    pub fn pc1(&self) -> PC1_R {
        PC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin C2"]
    #[inline(always)]
    pub fn pc2(&self) -> PC2_R {
        PC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin C3"]
    #[inline(always)]
    pub fn pc3(&self) -> PC3_R {
        PC3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin C0"]
    #[inline(always)]
    #[must_use]
    pub fn pc0(&mut self) -> PC0_W<DIR_SPEC> {
        PC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin C1"]
    #[inline(always)]
    #[must_use]
    pub fn pc1(&mut self) -> PC1_W<DIR_SPEC> {
        PC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin C2"]
    #[inline(always)]
    #[must_use]
    pub fn pc2(&mut self) -> PC2_W<DIR_SPEC> {
        PC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin C3"]
    #[inline(always)]
    #[must_use]
    pub fn pc3(&mut self) -> PC3_W<DIR_SPEC> {
        PC3_W::new(self, 3)
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
