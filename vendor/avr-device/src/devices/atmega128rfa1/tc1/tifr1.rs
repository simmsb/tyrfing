#[doc = "Register `TIFR1` reader"]
pub type R = crate::R<TIFR1_SPEC>;
#[doc = "Register `TIFR1` writer"]
pub type W = crate::W<TIFR1_SPEC>;
#[doc = "Field `TOV1` reader - Timer/Counter1 Overflow Flag"]
pub type TOV1_R = crate::BitReader;
#[doc = "Field `TOV1` writer - Timer/Counter1 Overflow Flag"]
pub type TOV1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF1A` reader - Timer/Counter1 Output Compare A Match Flag"]
pub type OCF1A_R = crate::BitReader;
#[doc = "Field `OCF1A` writer - Timer/Counter1 Output Compare A Match Flag"]
pub type OCF1A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF1B` reader - Timer/Counter1 Output Compare B Match Flag"]
pub type OCF1B_R = crate::BitReader;
#[doc = "Field `OCF1B` writer - Timer/Counter1 Output Compare B Match Flag"]
pub type OCF1B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF1C` reader - Timer/Counter1 Output Compare C Match Flag"]
pub type OCF1C_R = crate::BitReader;
#[doc = "Field `OCF1C` writer - Timer/Counter1 Output Compare C Match Flag"]
pub type OCF1C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICF1` reader - Timer/Counter1 Input Capture Flag"]
pub type ICF1_R = crate::BitReader;
#[doc = "Field `ICF1` writer - Timer/Counter1 Input Capture Flag"]
pub type ICF1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter1 Overflow Flag"]
    #[inline(always)]
    pub fn tov1(&self) -> TOV1_R {
        TOV1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter1 Output Compare A Match Flag"]
    #[inline(always)]
    pub fn ocf1a(&self) -> OCF1A_R {
        OCF1A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter1 Output Compare B Match Flag"]
    #[inline(always)]
    pub fn ocf1b(&self) -> OCF1B_R {
        OCF1B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter1 Output Compare C Match Flag"]
    #[inline(always)]
    pub fn ocf1c(&self) -> OCF1C_R {
        OCF1C_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter1 Input Capture Flag"]
    #[inline(always)]
    pub fn icf1(&self) -> ICF1_R {
        ICF1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter1 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov1(&mut self) -> TOV1_W<TIFR1_SPEC> {
        TOV1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer/Counter1 Output Compare A Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf1a(&mut self) -> OCF1A_W<TIFR1_SPEC> {
        OCF1A_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter1 Output Compare B Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf1b(&mut self) -> OCF1B_W<TIFR1_SPEC> {
        OCF1B_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer/Counter1 Output Compare C Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ocf1c(&mut self) -> OCF1C_W<TIFR1_SPEC> {
        OCF1C_W::new(self, 3)
    }
    #[doc = "Bit 5 - Timer/Counter1 Input Capture Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icf1(&mut self) -> ICF1_W<TIFR1_SPEC> {
        ICF1_W::new(self, 5)
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
#[doc = "Timer/Counter1 Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIFR1_SPEC;
impl crate::RegisterSpec for TIFR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tifr1::R`](R) reader structure"]
impl crate::Readable for TIFR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tifr1::W`](W) writer structure"]
impl crate::Writable for TIFR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFR1 to value 0"]
impl crate::Resettable for TIFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
