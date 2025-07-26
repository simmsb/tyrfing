#[doc = "Register `LINCR` reader"]
pub type R = crate::R<LINCR_SPEC>;
#[doc = "Register `LINCR` writer"]
pub type W = crate::W<LINCR_SPEC>;
#[doc = "Field `LCMD` reader - LIN Command and Mode bits"]
pub type LCMD_R = crate::FieldReader;
#[doc = "Field `LCMD` writer - LIN Command and Mode bits"]
pub type LCMD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Field `LENA` reader - LIN or UART Enable"]
pub type LENA_R = crate::BitReader;
#[doc = "Field `LENA` writer - LIN or UART Enable"]
pub type LENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCONF` reader - LIN Configuration bits"]
pub type LCONF_R = crate::FieldReader;
#[doc = "Field `LCONF` writer - LIN Configuration bits"]
pub type LCONF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `LIN13` reader - LIN Standard"]
pub type LIN13_R = crate::BitReader;
#[doc = "Field `LIN13` writer - LIN Standard"]
pub type LIN13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSWRES` reader - Software Reset"]
pub type LSWRES_R = crate::BitReader;
#[doc = "Field `LSWRES` writer - Software Reset"]
pub type LSWRES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - LIN Command and Mode bits"]
    #[inline(always)]
    pub fn lcmd(&self) -> LCMD_R {
        LCMD_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - LIN or UART Enable"]
    #[inline(always)]
    pub fn lena(&self) -> LENA_R {
        LENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - LIN Configuration bits"]
    #[inline(always)]
    pub fn lconf(&self) -> LCONF_R {
        LCONF_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - LIN Standard"]
    #[inline(always)]
    pub fn lin13(&self) -> LIN13_R {
        LIN13_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    pub fn lswres(&self) -> LSWRES_R {
        LSWRES_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - LIN Command and Mode bits"]
    #[inline(always)]
    #[must_use]
    pub fn lcmd(&mut self) -> LCMD_W<LINCR_SPEC> {
        LCMD_W::new(self, 0)
    }
    #[doc = "Bit 3 - LIN or UART Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lena(&mut self) -> LENA_W<LINCR_SPEC> {
        LENA_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - LIN Configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn lconf(&mut self) -> LCONF_W<LINCR_SPEC> {
        LCONF_W::new(self, 4)
    }
    #[doc = "Bit 6 - LIN Standard"]
    #[inline(always)]
    #[must_use]
    pub fn lin13(&mut self) -> LIN13_W<LINCR_SPEC> {
        LIN13_W::new(self, 6)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn lswres(&mut self) -> LSWRES_W<LINCR_SPEC> {
        LSWRES_W::new(self, 7)
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
#[doc = "LIN Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lincr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lincr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINCR_SPEC;
impl crate::RegisterSpec for LINCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lincr::R`](R) reader structure"]
impl crate::Readable for LINCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lincr::W`](W) writer structure"]
impl crate::Writable for LINCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINCR to value 0"]
impl crate::Resettable for LINCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
