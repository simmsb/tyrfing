#[doc = "Register `RXDATAH` reader"]
pub type R = crate::R<RXDATAH_SPEC>;
#[doc = "Field `DATA8` reader - Receiver Data Register"]
pub type DATA8_R = crate::BitReader;
#[doc = "Field `PERR` reader - Parity Error"]
pub type PERR_R = crate::BitReader;
#[doc = "Field `FERR` reader - Frame Error"]
pub type FERR_R = crate::BitReader;
#[doc = "Field `BUFOVF` reader - Buffer Overflow"]
pub type BUFOVF_R = crate::BitReader;
#[doc = "Field `RXCIF` reader - Receive Complete Interrupt Flag"]
pub type RXCIF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Data Register"]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&self) -> BUFOVF_R {
        BUFOVF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Flag"]
    #[inline(always)]
    pub fn rxcif(&self) -> RXCIF_R {
        RXCIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Receive Data High Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdatah::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDATAH_SPEC;
impl crate::RegisterSpec for RXDATAH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxdatah::R`](R) reader structure"]
impl crate::Readable for RXDATAH_SPEC {}
#[doc = "`reset()` method sets RXDATAH to value 0"]
impl crate::Resettable for RXDATAH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
