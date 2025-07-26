#[doc = "Register `TIFR3` reader"]
pub type R = crate::R<TIFR3_SPEC>;
#[doc = "Register `TIFR3` writer"]
pub type W = crate::W<TIFR3_SPEC>;
#[doc = "Field `TOV3` reader - Timer/Counter3 Overflow Flag"]
pub type TOV3_R = crate::BitReader;
#[doc = "Field `TOV3` writer - Timer/Counter3 Overflow Flag"]
pub type TOV3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF3A` reader - Output Compare Flag 3A"]
pub type OCF3A_R = crate::BitReader;
#[doc = "Field `OCF3A` writer - Output Compare Flag 3A"]
pub type OCF3A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF3B` reader - Output Compare Flag 3B"]
pub type OCF3B_R = crate::BitReader;
#[doc = "Field `OCF3B` writer - Output Compare Flag 3B"]
pub type OCF3B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF3C` reader - Output Compare Flag 3C"]
pub type OCF3C_R = crate::BitReader;
#[doc = "Field `OCF3C` writer - Output Compare Flag 3C"]
pub type OCF3C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICF3` reader - Input Capture Flag 3"]
pub type ICF3_R = crate::BitReader;
#[doc = "Field `ICF3` writer - Input Capture Flag 3"]
pub type ICF3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter3 Overflow Flag"]
    #[inline(always)]
    pub fn tov3(&self) -> TOV3_R {
        TOV3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Compare Flag 3A"]
    #[inline(always)]
    pub fn ocf3a(&self) -> OCF3A_R {
        OCF3A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Compare Flag 3B"]
    #[inline(always)]
    pub fn ocf3b(&self) -> OCF3B_R {
        OCF3B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Compare Flag 3C"]
    #[inline(always)]
    pub fn ocf3c(&self) -> OCF3C_R {
        OCF3C_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Input Capture Flag 3"]
    #[inline(always)]
    pub fn icf3(&self) -> ICF3_R {
        ICF3_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter3 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov3(&mut self) -> TOV3_W<TIFR3_SPEC> {
        TOV3_W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Compare Flag 3A"]
    #[inline(always)]
    #[must_use]
    pub fn ocf3a(&mut self) -> OCF3A_W<TIFR3_SPEC> {
        OCF3A_W::new(self, 1)
    }
    #[doc = "Bit 2 - Output Compare Flag 3B"]
    #[inline(always)]
    #[must_use]
    pub fn ocf3b(&mut self) -> OCF3B_W<TIFR3_SPEC> {
        OCF3B_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output Compare Flag 3C"]
    #[inline(always)]
    #[must_use]
    pub fn ocf3c(&mut self) -> OCF3C_W<TIFR3_SPEC> {
        OCF3C_W::new(self, 3)
    }
    #[doc = "Bit 5 - Input Capture Flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn icf3(&mut self) -> ICF3_W<TIFR3_SPEC> {
        ICF3_W::new(self, 5)
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
#[doc = "Timer/Counter3 Interrupt Flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIFR3_SPEC;
impl crate::RegisterSpec for TIFR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tifr3::R`](R) reader structure"]
impl crate::Readable for TIFR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tifr3::W`](W) writer structure"]
impl crate::Writable for TIFR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFR3 to value 0"]
impl crate::Resettable for TIFR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
