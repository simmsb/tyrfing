#[doc = "Register `TRXPR` reader"]
pub type R = crate::R<TRXPR_SPEC>;
#[doc = "Register `TRXPR` writer"]
pub type W = crate::W<TRXPR_SPEC>;
#[doc = "Field `TRXRST` reader - Force Transceiver Reset"]
pub type TRXRST_R = crate::BitReader;
#[doc = "Field `TRXRST` writer - Force Transceiver Reset"]
pub type TRXRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLPTR` reader - Multi-purpose Transceiver Control Bit"]
pub type SLPTR_R = crate::BitReader;
#[doc = "Field `SLPTR` writer - Multi-purpose Transceiver Control Bit"]
pub type SLPTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Force Transceiver Reset"]
    #[inline(always)]
    pub fn trxrst(&self) -> TRXRST_R {
        TRXRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Multi-purpose Transceiver Control Bit"]
    #[inline(always)]
    pub fn slptr(&self) -> SLPTR_R {
        SLPTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bit 0 - Force Transceiver Reset"]
    #[inline(always)]
    #[must_use]
    pub fn trxrst(&mut self) -> TRXRST_W<TRXPR_SPEC> {
        TRXRST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Multi-purpose Transceiver Control Bit"]
    #[inline(always)]
    #[must_use]
    pub fn slptr(&mut self) -> SLPTR_W<TRXPR_SPEC> {
        SLPTR_W::new(self, 1)
    }
    #[doc = "Bits 4:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<TRXPR_SPEC> {
        RES_W::new(self, 4)
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
#[doc = "Transceiver Pin Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trxpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trxpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRXPR_SPEC;
impl crate::RegisterSpec for TRXPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`trxpr::R`](R) reader structure"]
impl crate::Readable for TRXPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trxpr::W`](W) writer structure"]
impl crate::Writable for TRXPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRXPR to value 0"]
impl crate::Resettable for TRXPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
