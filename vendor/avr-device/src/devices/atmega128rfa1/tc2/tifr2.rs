#[doc = "Register `TIFR2` reader"]
pub type R = crate::R<TIFR2_SPEC>;
#[doc = "Register `TIFR2` writer"]
pub type W = crate::W<TIFR2_SPEC>;
#[doc = "Field `TOV2` reader - Timer/Counter2 Overflow Flag"]
pub type TOV2_R = crate::BitReader;
#[doc = "Field `TOV2` writer - Timer/Counter2 Overflow Flag"]
pub type TOV2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF2A` reader - Output Compare Flag 2 A"]
pub type OCF2A_R = crate::BitReader;
#[doc = "Field `OCF2A` writer - Output Compare Flag 2 A"]
pub type OCF2A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCF2B` reader - Output Compare Flag 2 B"]
pub type OCF2B_R = crate::BitReader;
#[doc = "Field `OCF2B` writer - Output Compare Flag 2 B"]
pub type OCF2B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Timer/Counter2 Overflow Flag"]
    #[inline(always)]
    pub fn tov2(&self) -> TOV2_R {
        TOV2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Compare Flag 2 A"]
    #[inline(always)]
    pub fn ocf2a(&self) -> OCF2A_R {
        OCF2A_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Compare Flag 2 B"]
    #[inline(always)]
    pub fn ocf2b(&self) -> OCF2B_R {
        OCF2B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 3) & 0x1f)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter2 Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tov2(&mut self) -> TOV2_W<TIFR2_SPEC> {
        TOV2_W::new(self, 0)
    }
    #[doc = "Bit 1 - Output Compare Flag 2 A"]
    #[inline(always)]
    #[must_use]
    pub fn ocf2a(&mut self) -> OCF2A_W<TIFR2_SPEC> {
        OCF2A_W::new(self, 1)
    }
    #[doc = "Bit 2 - Output Compare Flag 2 B"]
    #[inline(always)]
    #[must_use]
    pub fn ocf2b(&mut self) -> OCF2B_W<TIFR2_SPEC> {
        OCF2B_W::new(self, 2)
    }
    #[doc = "Bits 3:7 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<TIFR2_SPEC> {
        RES_W::new(self, 3)
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
#[doc = "Timer/Counter Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tifr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tifr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIFR2_SPEC;
impl crate::RegisterSpec for TIFR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tifr2::R`](R) reader structure"]
impl crate::Readable for TIFR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tifr2::W`](W) writer structure"]
impl crate::Writable for TIFR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFR2 to value 0"]
impl crate::Resettable for TIFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
