#[doc = "Register `DIRTGL` reader"]
pub type R = crate::R<DIRTGL_SPEC>;
#[doc = "Register `DIRTGL` writer"]
pub type W = crate::W<DIRTGL_SPEC>;
#[doc = "Field `PC0` reader - Pin C0"]
pub type PC0_R = crate::BitReader;
#[doc = "Field `PC0` writer - Pin C0"]
pub type PC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC1` reader - Pin C1"]
pub type PC1_R = crate::BitReader;
#[doc = "Field `PC1` writer - Pin C1"]
pub type PC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC2` reader - Pin C2"]
pub type PC2_R = crate::BitReader;
#[doc = "Field `PC2` writer - Pin C2"]
pub type PC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC3` reader - Pin C3"]
pub type PC3_R = crate::BitReader;
#[doc = "Field `PC3` writer - Pin C3"]
pub type PC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC4` reader - Pin C4"]
pub type PC4_R = crate::BitReader;
#[doc = "Field `PC4` writer - Pin C4"]
pub type PC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC5` reader - Pin C5"]
pub type PC5_R = crate::BitReader;
#[doc = "Field `PC5` writer - Pin C5"]
pub type PC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC6` reader - Pin C6"]
pub type PC6_R = crate::BitReader;
#[doc = "Field `PC6` writer - Pin C6"]
pub type PC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC7` reader - Pin C7"]
pub type PC7_R = crate::BitReader;
#[doc = "Field `PC7` writer - Pin C7"]
pub type PC7_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 4 - Pin C4"]
    #[inline(always)]
    pub fn pc4(&self) -> PC4_R {
        PC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin C5"]
    #[inline(always)]
    pub fn pc5(&self) -> PC5_R {
        PC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin C6"]
    #[inline(always)]
    pub fn pc6(&self) -> PC6_R {
        PC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin C7"]
    #[inline(always)]
    pub fn pc7(&self) -> PC7_R {
        PC7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin C0"]
    #[inline(always)]
    #[must_use]
    pub fn pc0(&mut self) -> PC0_W<DIRTGL_SPEC> {
        PC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin C1"]
    #[inline(always)]
    #[must_use]
    pub fn pc1(&mut self) -> PC1_W<DIRTGL_SPEC> {
        PC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin C2"]
    #[inline(always)]
    #[must_use]
    pub fn pc2(&mut self) -> PC2_W<DIRTGL_SPEC> {
        PC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin C3"]
    #[inline(always)]
    #[must_use]
    pub fn pc3(&mut self) -> PC3_W<DIRTGL_SPEC> {
        PC3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin C4"]
    #[inline(always)]
    #[must_use]
    pub fn pc4(&mut self) -> PC4_W<DIRTGL_SPEC> {
        PC4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin C5"]
    #[inline(always)]
    #[must_use]
    pub fn pc5(&mut self) -> PC5_W<DIRTGL_SPEC> {
        PC5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin C6"]
    #[inline(always)]
    #[must_use]
    pub fn pc6(&mut self) -> PC6_W<DIRTGL_SPEC> {
        PC6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin C7"]
    #[inline(always)]
    #[must_use]
    pub fn pc7(&mut self) -> PC7_W<DIRTGL_SPEC> {
        PC7_W::new(self, 7)
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
#[doc = "Data Direction Toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dirtgl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dirtgl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIRTGL_SPEC;
impl crate::RegisterSpec for DIRTGL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dirtgl::R`](R) reader structure"]
impl crate::Readable for DIRTGL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dirtgl::W`](W) writer structure"]
impl crate::Writable for DIRTGL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRTGL to value 0"]
impl crate::Resettable for DIRTGL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
