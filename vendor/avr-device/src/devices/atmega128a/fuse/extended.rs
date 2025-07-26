#[doc = "Register `EXTENDED` reader"]
pub type R = crate::R<EXTENDED_SPEC>;
#[doc = "Register `EXTENDED` writer"]
pub type W = crate::W<EXTENDED_SPEC>;
#[doc = "Field `WDTON` reader - Watchdog Timer always on"]
pub type WDTON_R = crate::BitReader;
#[doc = "Field `WDTON` writer - Watchdog Timer always on"]
pub type WDTON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M103C` reader - ATmega103 Compatibility Mode"]
pub type M103C_R = crate::BitReader;
#[doc = "Field `M103C` writer - ATmega103 Compatibility Mode"]
pub type M103C_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Watchdog Timer always on"]
    #[inline(always)]
    pub fn wdton(&self) -> WDTON_R {
        WDTON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ATmega103 Compatibility Mode"]
    #[inline(always)]
    pub fn m103c(&self) -> M103C_R {
        M103C_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer always on"]
    #[inline(always)]
    #[must_use]
    pub fn wdton(&mut self) -> WDTON_W<EXTENDED_SPEC> {
        WDTON_W::new(self, 0)
    }
    #[doc = "Bit 1 - ATmega103 Compatibility Mode"]
    #[inline(always)]
    #[must_use]
    pub fn m103c(&mut self) -> M103C_W<EXTENDED_SPEC> {
        M103C_W::new(self, 1)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extended::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extended::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTENDED_SPEC;
impl crate::RegisterSpec for EXTENDED_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`extended::R`](R) reader structure"]
impl crate::Readable for EXTENDED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extended::W`](W) writer structure"]
impl crate::Writable for EXTENDED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTENDED to value 0"]
impl crate::Resettable for EXTENDED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
