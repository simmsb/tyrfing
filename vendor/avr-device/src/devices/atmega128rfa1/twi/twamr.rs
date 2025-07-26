#[doc = "Register `TWAMR` reader"]
pub type R = crate::R<TWAMR_SPEC>;
#[doc = "Register `TWAMR` writer"]
pub type W = crate::W<TWAMR_SPEC>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::BitReader;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWAM` reader - TWI (Slave) Address Mask Bits"]
pub type TWAM_R = crate::FieldReader;
#[doc = "Field `TWAM` writer - TWI (Slave) Address Mask Bits"]
pub type TWAM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - TWI (Slave) Address Mask Bits"]
    #[inline(always)]
    pub fn twam(&self) -> TWAM_R {
        TWAM_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<TWAMR_SPEC> {
        RES_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - TWI (Slave) Address Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn twam(&mut self) -> TWAM_W<TWAMR_SPEC> {
        TWAM_W::new(self, 1)
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
#[doc = "TWI (Slave) Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twamr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twamr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWAMR_SPEC;
impl crate::RegisterSpec for TWAMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twamr::R`](R) reader structure"]
impl crate::Readable for TWAMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twamr::W`](W) writer structure"]
impl crate::Writable for TWAMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWAMR to value 0"]
impl crate::Resettable for TWAMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
