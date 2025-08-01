#[doc = "Register `DDRB` reader"]
pub type R = crate::R<DDRB_SPEC>;
#[doc = "Register `DDRB` writer"]
pub type W = crate::W<DDRB_SPEC>;
#[doc = "Field `PB0` reader - Pin B0"]
pub type PB0_R = crate::BitReader;
#[doc = "Field `PB0` writer - Pin B0"]
pub type PB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB1` reader - Pin B1"]
pub type PB1_R = crate::BitReader;
#[doc = "Field `PB1` writer - Pin B1"]
pub type PB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB2` reader - Pin B2"]
pub type PB2_R = crate::BitReader;
#[doc = "Field `PB2` writer - Pin B2"]
pub type PB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB3` reader - Pin B3"]
pub type PB3_R = crate::BitReader;
#[doc = "Field `PB3` writer - Pin B3"]
pub type PB3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB4` reader - Pin B4"]
pub type PB4_R = crate::BitReader;
#[doc = "Field `PB4` writer - Pin B4"]
pub type PB4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB5` reader - Pin B5"]
pub type PB5_R = crate::BitReader;
#[doc = "Field `PB5` writer - Pin B5"]
pub type PB5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pin B0"]
    #[inline(always)]
    pub fn pb0(&self) -> PB0_R {
        PB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin B1"]
    #[inline(always)]
    pub fn pb1(&self) -> PB1_R {
        PB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin B2"]
    #[inline(always)]
    pub fn pb2(&self) -> PB2_R {
        PB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin B3"]
    #[inline(always)]
    pub fn pb3(&self) -> PB3_R {
        PB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin B4"]
    #[inline(always)]
    pub fn pb4(&self) -> PB4_R {
        PB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin B5"]
    #[inline(always)]
    pub fn pb5(&self) -> PB5_R {
        PB5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin B0"]
    #[inline(always)]
    #[must_use]
    pub fn pb0(&mut self) -> PB0_W<DDRB_SPEC> {
        PB0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin B1"]
    #[inline(always)]
    #[must_use]
    pub fn pb1(&mut self) -> PB1_W<DDRB_SPEC> {
        PB1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin B2"]
    #[inline(always)]
    #[must_use]
    pub fn pb2(&mut self) -> PB2_W<DDRB_SPEC> {
        PB2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin B3"]
    #[inline(always)]
    #[must_use]
    pub fn pb3(&mut self) -> PB3_W<DDRB_SPEC> {
        PB3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pin B4"]
    #[inline(always)]
    #[must_use]
    pub fn pb4(&mut self) -> PB4_W<DDRB_SPEC> {
        PB4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin B5"]
    #[inline(always)]
    #[must_use]
    pub fn pb5(&mut self) -> PB5_W<DDRB_SPEC> {
        PB5_W::new(self, 5)
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
#[doc = "Data Direction Register, Port B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRB_SPEC;
impl crate::RegisterSpec for DDRB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ddrb::R`](R) reader structure"]
impl crate::Readable for DDRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddrb::W`](W) writer structure"]
impl crate::Writable for DDRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRB to value 0"]
impl crate::Resettable for DDRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
