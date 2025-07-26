#[doc = "Register `TIFR4` reader"]
pub type R = crate::R<TIFR4_SPEC>;
#[doc = "Register `TIFR4` writer"]
pub type W = crate::W<TIFR4_SPEC>;
#[doc = "Field `TOV4` reader - Timer/Counter4 Overflow Flag"]
pub type TOV4_R = crate::BitReader;
#[doc = "Field `TOV4` writer - Timer/Counter4 Overflow Flag"]
pub type TOV4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF4B` reader - Output Compare Flag 4B"]
pub type OCF4B_R = crate::BitReader;
#[doc = "Field `OCF4B` writer - Output Compare Flag 4B"]
pub type OCF4B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF4A` reader - Output Compare Flag 4A"]
pub type OCF4A_R = crate::BitReader;
#[doc = "Field `OCF4A` writer - Output Compare Flag 4A"]
pub type OCF4A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF4D` reader - Output Compare Flag 4D"]
pub type OCF4D_R = crate::BitReader;
#[doc = "Field `OCF4D` writer - Output Compare Flag 4D"]
pub type OCF4D_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Timer/Counter4 Overflow Flag"]
    #[inline(always)]
    pub fn tov4(&self) -> TOV4_R {
        TOV4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Output Compare Flag 4B"]
    #[inline(always)]
    pub fn ocf4b(&self) -> OCF4B_R {
        OCF4B_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output Compare Flag 4A"]
    #[inline(always)]
    pub fn ocf4a(&self) -> OCF4A_R {
        OCF4A_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output Compare Flag 4D"]
    #[inline(always)]
    pub fn ocf4d(&self) -> OCF4D_R {
        OCF4D_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Timer/Counter4 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov4(&mut self) -> TOV4_W<TIFR4_SPEC> {
        TOV4_W::new(self, 2)
    }
    #[doc = "Bit 5 - Output Compare Flag 4B"]
    #[inline(always)]
    #[must_use]
    pub fn ocf4b(&mut self) -> OCF4B_W<TIFR4_SPEC> {
        OCF4B_W::new(self, 5)
    }
    #[doc = "Bit 6 - Output Compare Flag 4A"]
    #[inline(always)]
    #[must_use]
    pub fn ocf4a(&mut self) -> OCF4A_W<TIFR4_SPEC> {
        OCF4A_W::new(self, 6)
    }
    #[doc = "Bit 7 - Output Compare Flag 4D"]
    #[inline(always)]
    #[must_use]
    pub fn ocf4d(&mut self) -> OCF4D_W<TIFR4_SPEC> {
        OCF4D_W::new(self, 7)
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
#[doc = "Timer/Counter4 Interrupt Flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIFR4_SPEC;
impl crate::RegisterSpec for TIFR4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tifr4::R`](R) reader structure"]
impl crate::Readable for TIFR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tifr4::W`](W) writer structure"]
impl crate::Writable for TIFR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFR4 to value 0"]
impl crate::Resettable for TIFR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
