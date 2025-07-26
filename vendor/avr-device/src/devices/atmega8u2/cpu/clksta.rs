#[doc = "Register `CLKSTA` reader"]
pub type R = crate::R<CLKSTA_SPEC>;
#[doc = "Register `CLKSTA` writer"]
pub type W = crate::W<CLKSTA_SPEC>;
#[doc = "Field `EXTON` reader - No Description."]
pub type EXTON_R = crate::BitReader;
#[doc = "Field `EXTON` writer - No Description."]
pub type EXTON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCON` reader - No Description."]
pub type RCON_R = crate::BitReader;
#[doc = "Field `RCON` writer - No Description."]
pub type RCON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn exton(&self) -> EXTON_R {
        EXTON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn rcon(&self) -> RCON_R {
        RCON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn exton(&mut self) -> EXTON_W<CLKSTA_SPEC> {
        EXTON_W::new(self, 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn rcon(&mut self) -> RCON_W<CLKSTA_SPEC> {
        RCON_W::new(self, 1)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKSTA_SPEC;
impl crate::RegisterSpec for CLKSTA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clksta::R`](R) reader structure"]
impl crate::Readable for CLKSTA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clksta::W`](W) writer structure"]
impl crate::Writable for CLKSTA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSTA to value 0"]
impl crate::Resettable for CLKSTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
