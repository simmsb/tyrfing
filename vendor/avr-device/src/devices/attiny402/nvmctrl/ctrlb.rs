#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `APCWP` reader - Application code write protect"]
pub type APCWP_R = crate::BitReader;
#[doc = "Field `APCWP` writer - Application code write protect"]
pub type APCWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTLOCK` reader - Boot Lock"]
pub type BOOTLOCK_R = crate::BitReader;
#[doc = "Field `BOOTLOCK` writer - Boot Lock"]
pub type BOOTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Application code write protect"]
    #[inline(always)]
    pub fn apcwp(&self) -> APCWP_R {
        APCWP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Boot Lock"]
    #[inline(always)]
    pub fn bootlock(&self) -> BOOTLOCK_R {
        BOOTLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Application code write protect"]
    #[inline(always)]
    #[must_use]
    pub fn apcwp(&mut self) -> APCWP_W<CTRLB_SPEC> {
        APCWP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Boot Lock"]
    #[inline(always)]
    #[must_use]
    pub fn bootlock(&mut self) -> BOOTLOCK_W<CTRLB_SPEC> {
        BOOTLOCK_W::new(self, 1)
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
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
