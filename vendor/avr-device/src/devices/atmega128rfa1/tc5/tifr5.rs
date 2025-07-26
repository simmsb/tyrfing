#[doc = "Register `TIFR5` reader"]
pub type R = crate::R<TIFR5_SPEC>;
#[doc = "Register `TIFR5` writer"]
pub type W = crate::W<TIFR5_SPEC>;
#[doc = "Field `TOV5` reader - Timer/Counter5 Overflow Flag"]
pub type TOV5_R = crate::BitReader;
#[doc = "Field `TOV5` writer - Timer/Counter5 Overflow Flag"]
pub type TOV5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF5A` reader - Timer/Counter5 Output Compare A Match Flag"]
pub type OCF5A_R = crate::BitReader;
#[doc = "Field `OCF5A` writer - Timer/Counter5 Output Compare A Match Flag"]
pub type OCF5A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF5B` reader - Timer/Counter5 Output Compare B Match Flag"]
pub type OCF5B_R = crate::BitReader;
#[doc = "Field `OCF5B` writer - Timer/Counter5 Output Compare B Match Flag"]
pub type OCF5B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF5C` reader - Timer/Counter5 Output Compare C Match Flag"]
pub type OCF5C_R = crate::BitReader;
#[doc = "Field `OCF5C` writer - Timer/Counter5 Output Compare C Match Flag"]
pub type OCF5C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICF5` reader - Timer/Counter5 Input Capture Flag"]
pub type ICF5_R = crate::BitReader;
#[doc = "Field `ICF5` writer - Timer/Counter5 Input Capture Flag"]
pub type ICF5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter5 Overflow Flag"]
    #[inline(always)]
    pub fn tov5(&self) -> TOV5_R {
        TOV5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter5 Output Compare A Match Flag"]
    #[inline(always)]
    pub fn ocf5a(&self) -> OCF5A_R {
        OCF5A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter5 Output Compare B Match Flag"]
    #[inline(always)]
    pub fn ocf5b(&self) -> OCF5B_R {
        OCF5B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter5 Output Compare C Match Flag"]
    #[inline(always)]
    pub fn ocf5c(&self) -> OCF5C_R {
        OCF5C_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter5 Input Capture Flag"]
    #[inline(always)]
    pub fn icf5(&self) -> ICF5_R {
        ICF5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter5 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov5(&mut self) -> TOV5_W<TIFR5_SPEC> {
        TOV5_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer/Counter5 Output Compare A Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf5a(&mut self) -> OCF5A_W<TIFR5_SPEC> {
        OCF5A_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter5 Output Compare B Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf5b(&mut self) -> OCF5B_W<TIFR5_SPEC> {
        OCF5B_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer/Counter5 Output Compare C Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf5c(&mut self) -> OCF5C_W<TIFR5_SPEC> {
        OCF5C_W::new(self, 3)
    }
    #[doc = "Bit 5 - Timer/Counter5 Input Capture Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icf5(&mut self) -> ICF5_W<TIFR5_SPEC> {
        ICF5_W::new(self, 5)
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
#[doc = "Timer/Counter5 Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIFR5_SPEC;
impl crate::RegisterSpec for TIFR5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tifr5::R`](R) reader structure"]
impl crate::Readable for TIFR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tifr5::W`](W) writer structure"]
impl crate::Writable for TIFR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFR5 to value 0"]
impl crate::Resettable for TIFR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
