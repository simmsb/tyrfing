#[doc = "Register `DIRTGL` reader"]
pub type R = crate::R<DIRTGL_SPEC>;
#[doc = "Register `DIRTGL` writer"]
pub type W = crate::W<DIRTGL_SPEC>;
#[doc = "Field `PE0` reader - Pin E0"]
pub type PE0_R = crate::BitReader;
#[doc = "Field `PE0` writer - Pin E0"]
pub type PE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE1` reader - Pin E1"]
pub type PE1_R = crate::BitReader;
#[doc = "Field `PE1` writer - Pin E1"]
pub type PE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE2` reader - Pin E2"]
pub type PE2_R = crate::BitReader;
#[doc = "Field `PE2` writer - Pin E2"]
pub type PE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE3` reader - Pin E3"]
pub type PE3_R = crate::BitReader;
#[doc = "Field `PE3` writer - Pin E3"]
pub type PE3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin E0"]
    #[inline(always)]
    pub fn pe0(&self) -> PE0_R {
        PE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin E1"]
    #[inline(always)]
    pub fn pe1(&self) -> PE1_R {
        PE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin E2"]
    #[inline(always)]
    pub fn pe2(&self) -> PE2_R {
        PE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin E3"]
    #[inline(always)]
    pub fn pe3(&self) -> PE3_R {
        PE3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin E0"]
    #[inline(always)]
    #[must_use]
    pub fn pe0(&mut self) -> PE0_W<DIRTGL_SPEC> {
        PE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin E1"]
    #[inline(always)]
    #[must_use]
    pub fn pe1(&mut self) -> PE1_W<DIRTGL_SPEC> {
        PE1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin E2"]
    #[inline(always)]
    #[must_use]
    pub fn pe2(&mut self) -> PE2_W<DIRTGL_SPEC> {
        PE2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin E3"]
    #[inline(always)]
    #[must_use]
    pub fn pe3(&mut self) -> PE3_W<DIRTGL_SPEC> {
        PE3_W::new(self, 3)
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
