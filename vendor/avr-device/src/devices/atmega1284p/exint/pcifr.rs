#[doc = "Register `PCIFR` reader"]
pub type R = crate::R<PCIFR_SPEC>;
#[doc = "Register `PCIFR` writer"]
pub type W = crate::W<PCIFR_SPEC>;
#[doc = "Field `PCIF0` reader - Pin Change Interrupt Flag 0"]
pub type PCIF0_R = crate::BitReader;
#[doc = "Field `PCIF0` writer - Pin Change Interrupt Flag 0"]
pub type PCIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIF1` reader - Pin Change Interrupt Flag 1"]
pub type PCIF1_R = crate::BitReader;
#[doc = "Field `PCIF1` writer - Pin Change Interrupt Flag 1"]
pub type PCIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIF2` reader - Pin Change Interrupt Flag 2"]
pub type PCIF2_R = crate::BitReader;
#[doc = "Field `PCIF2` writer - Pin Change Interrupt Flag 2"]
pub type PCIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIF3` reader - Pin Change Interrupt Flag 3"]
pub type PCIF3_R = crate::BitReader;
#[doc = "Field `PCIF3` writer - Pin Change Interrupt Flag 3"]
pub type PCIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin Change Interrupt Flag 0"]
    #[inline(always)]
    pub fn pcif0(&self) -> PCIF0_R {
        PCIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin Change Interrupt Flag 1"]
    #[inline(always)]
    pub fn pcif1(&self) -> PCIF1_R {
        PCIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin Change Interrupt Flag 2"]
    #[inline(always)]
    pub fn pcif2(&self) -> PCIF2_R {
        PCIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin Change Interrupt Flag 3"]
    #[inline(always)]
    pub fn pcif3(&self) -> PCIF3_R {
        PCIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin Change Interrupt Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn pcif0(&mut self) -> PCIF0_W<PCIFR_SPEC> {
        PCIF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin Change Interrupt Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn pcif1(&mut self) -> PCIF1_W<PCIFR_SPEC> {
        PCIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin Change Interrupt Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcif2(&mut self) -> PCIF2_W<PCIFR_SPEC> {
        PCIF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin Change Interrupt Flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn pcif3(&mut self) -> PCIF3_W<PCIFR_SPEC> {
        PCIF3_W::new(self, 3)
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
#[doc = "Pin Change Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcifr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcifr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCIFR_SPEC;
impl crate::RegisterSpec for PCIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pcifr::R`](R) reader structure"]
impl crate::Readable for PCIFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcifr::W`](W) writer structure"]
impl crate::Writable for PCIFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCIFR to value 0"]
impl crate::Resettable for PCIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
