#[doc = "Register `TIFR0` reader"]
pub type R = crate::R<TIFR0_SPEC>;
#[doc = "Register `TIFR0` writer"]
pub type W = crate::W<TIFR0_SPEC>;
#[doc = "Field `TOV0` reader - Timer/Counter0 Overflow Flag"]
pub type TOV0_R = crate::BitReader;
#[doc = "Field `TOV0` writer - Timer/Counter0 Overflow Flag"]
pub type TOV0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF0A` reader - Timer/Counter0 Output Compare Flag 0A"]
pub type OCF0A_R = crate::BitReader;
#[doc = "Field `OCF0A` writer - Timer/Counter0 Output Compare Flag 0A"]
pub type OCF0A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF0B` reader - Timer/Counter0 Output Compare Flag 0B"]
pub type OCF0B_R = crate::BitReader;
#[doc = "Field `OCF0B` writer - Timer/Counter0 Output Compare Flag 0B"]
pub type OCF0B_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer/Counter0 Overflow Flag"]
    #[inline(always)]
    pub fn tov0(&self) -> TOV0_R {
        TOV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter0 Output Compare Flag 0A"]
    #[inline(always)]
    pub fn ocf0a(&self) -> OCF0A_R {
        OCF0A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter0 Output Compare Flag 0B"]
    #[inline(always)]
    pub fn ocf0b(&self) -> OCF0B_R {
        OCF0B_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter0 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov0(&mut self) -> TOV0_W<TIFR0_SPEC> {
        TOV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer/Counter0 Output Compare Flag 0A"]
    #[inline(always)]
    #[must_use]
    pub fn ocf0a(&mut self) -> OCF0A_W<TIFR0_SPEC> {
        OCF0A_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer/Counter0 Output Compare Flag 0B"]
    #[inline(always)]
    #[must_use]
    pub fn ocf0b(&mut self) -> OCF0B_W<TIFR0_SPEC> {
        OCF0B_W::new(self, 2)
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
#[doc = "Timer/Counter0 Interrupt Flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIFR0_SPEC;
impl crate::RegisterSpec for TIFR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tifr0::R`](R) reader structure"]
impl crate::Readable for TIFR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tifr0::W`](W) writer structure"]
impl crate::Writable for TIFR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFR0 to value 0"]
impl crate::Resettable for TIFR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
