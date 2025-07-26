#[doc = "Register `UDADDR` reader"]
pub type R = crate::R<UDADDR_SPEC>;
#[doc = "Register `UDADDR` writer"]
pub type W = crate::W<UDADDR_SPEC>;
#[doc = "Field `UADD` reader - USB Address Bits"]
pub type UADD_R = crate::FieldReader;
#[doc = "Field `UADD` writer - USB Address Bits"]
pub type UADD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `ADDEN` reader - Address Enable Bit"]
pub type ADDEN_R = crate::BitReader;
#[doc = "Field `ADDEN` writer - Address Enable Bit"]
pub type ADDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - USB Address Bits"]
    #[inline(always)]
    pub fn uadd(&self) -> UADD_R {
        UADD_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Address Enable Bit"]
    #[inline(always)]
    pub fn adden(&self) -> ADDEN_R {
        ADDEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Address Bits"]
    #[inline(always)]
    #[must_use]
    pub fn uadd(&mut self) -> UADD_W<UDADDR_SPEC> {
        UADD_W::new(self, 0)
    }
    #[doc = "Bit 7 - Address Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn adden(&mut self) -> ADDEN_W<UDADDR_SPEC> {
        ADDEN_W::new(self, 7)
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
#[doc = "USB Device Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDADDR_SPEC;
impl crate::RegisterSpec for UDADDR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`udaddr::R`](R) reader structure"]
impl crate::Readable for UDADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udaddr::W`](W) writer structure"]
impl crate::Writable for UDADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDADDR to value 0"]
impl crate::Resettable for UDADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
