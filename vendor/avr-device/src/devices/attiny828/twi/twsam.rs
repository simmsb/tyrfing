#[doc = "Register `TWSAM` reader"]
pub type R = crate::R<TWSAM_SPEC>;
#[doc = "Register `TWSAM` writer"]
pub type W = crate::W<TWSAM_SPEC>;
#[doc = "Field `TWAE` reader - TWI Address Enable"]
pub type TWAE_R = crate::BitReader;
#[doc = "Field `TWAE` writer - TWI Address Enable"]
pub type TWAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWSAM` reader - TWI Address Mask Bits"]
pub type TWSAM_R = crate::FieldReader;
#[doc = "Field `TWSAM` writer - TWI Address Mask Bits"]
pub type TWSAM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - TWI Address Enable"]
    #[inline(always)]
    pub fn twae(&self) -> TWAE_R {
        TWAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - TWI Address Mask Bits"]
    #[inline(always)]
    pub fn twsam(&self) -> TWSAM_R {
        TWSAM_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - TWI Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twae(&mut self) -> TWAE_W<TWSAM_SPEC> {
        TWAE_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - TWI Address Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn twsam(&mut self) -> TWSAM_W<TWSAM_SPEC> {
        TWSAM_W::new(self, 1)
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
#[doc = "TWI Slave Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twsam::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twsam::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWSAM_SPEC;
impl crate::RegisterSpec for TWSAM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twsam::R`](R) reader structure"]
impl crate::Readable for TWSAM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twsam::W`](W) writer structure"]
impl crate::Writable for TWSAM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWSAM to value 0"]
impl crate::Resettable for TWSAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
