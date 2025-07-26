#[doc = "Register `DIRCLR` reader"]
pub type R = crate::R<DIRCLR_SPEC>;
#[doc = "Register `DIRCLR` writer"]
pub type W = crate::W<DIRCLR_SPEC>;
#[doc = "Field `PD4` reader - Pin D4"]
pub type PD4_R = crate::BitReader;
#[doc = "Field `PD4` writer - Pin D4"]
pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5` reader - Pin D5"]
pub type PD5_R = crate::BitReader;
#[doc = "Field `PD5` writer - Pin D5"]
pub type PD5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD6` reader - Pin D6"]
pub type PD6_R = crate::BitReader;
#[doc = "Field `PD6` writer - Pin D6"]
pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - Pin D7"]
pub type PD7_R = crate::BitReader;
#[doc = "Field `PD7` writer - Pin D7"]
pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn pd4(&mut self) -> PD4_W<DIRCLR_SPEC> {
        PD4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin D5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<DIRCLR_SPEC> {
        PD5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin D6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<DIRCLR_SPEC> {
        PD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin D7"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<DIRCLR_SPEC> {
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
#[doc = "Data Direction Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dirclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dirclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIRCLR_SPEC;
impl crate::RegisterSpec for DIRCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dirclr::R`](R) reader structure"]
impl crate::Readable for DIRCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dirclr::W`](W) writer structure"]
impl crate::Writable for DIRCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRCLR to value 0"]
impl crate::Resettable for DIRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
