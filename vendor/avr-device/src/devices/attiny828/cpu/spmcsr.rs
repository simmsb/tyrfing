#[doc = "Register `SPMCSR` reader"]
pub type R = crate::R<SPMCSR_SPEC>;
#[doc = "Register `SPMCSR` writer"]
pub type W = crate::W<SPMCSR_SPEC>;
#[doc = "Field `SPMEN` reader - Store Program Memory Enable"]
pub type SPMEN_R = crate::BitReader;
#[doc = "Field `SPMEN` writer - Store Program Memory Enable"]
pub type SPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGERS` reader - Page Erase"]
pub type PGERS_R = crate::BitReader;
#[doc = "Field `PGERS` writer - Page Erase"]
pub type PGERS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGWRT` reader - Page Write"]
pub type PGWRT_R = crate::BitReader;
#[doc = "Field `PGWRT` writer - Page Write"]
pub type PGWRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWFLB` reader - Read/Write Fuse and Lock Bits"]
pub type RWFLB_R = crate::BitReader;
#[doc = "Field `RWFLB` writer - Read/Write Fuse and Lock Bits"]
pub type RWFLB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWWSRE` reader - Read-While-Write section read enable"]
pub type RWWSRE_R = crate::BitReader;
#[doc = "Field `RWWSRE` writer - Read-While-Write section read enable"]
pub type RWWSRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSIG` reader - Read Device Signature Imprint Table"]
pub type RSIG_R = crate::BitReader;
#[doc = "Field `RSIG` writer - Read Device Signature Imprint Table"]
pub type RSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWWSB` reader - Read-While-Write Section Busy"]
pub type RWWSB_R = crate::BitReader;
#[doc = "Field `RWWSB` writer - Read-While-Write Section Busy"]
pub type RWWSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPMIE` reader - SPM Interrupt Enable"]
pub type SPMIE_R = crate::BitReader;
#[doc = "Field `SPMIE` writer - SPM Interrupt Enable"]
pub type SPMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Store Program Memory Enable"]
    #[inline(always)]
    pub fn spmen(&self) -> SPMEN_R {
        SPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    pub fn pgers(&self) -> PGERS_R {
        PGERS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Page Write"]
    #[inline(always)]
    pub fn pgwrt(&self) -> PGWRT_R {
        PGWRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read/Write Fuse and Lock Bits"]
    #[inline(always)]
    pub fn rwflb(&self) -> RWFLB_R {
        RWFLB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read-While-Write section read enable"]
    #[inline(always)]
    pub fn rwwsre(&self) -> RWWSRE_R {
        RWWSRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read Device Signature Imprint Table"]
    #[inline(always)]
    pub fn rsig(&self) -> RSIG_R {
        RSIG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read-While-Write Section Busy"]
    #[inline(always)]
    pub fn rwwsb(&self) -> RWWSB_R {
        RWWSB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPM Interrupt Enable"]
    #[inline(always)]
    pub fn spmie(&self) -> SPMIE_R {
        SPMIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Store Program Memory Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spmen(&mut self) -> SPMEN_W<SPMCSR_SPEC> {
        SPMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    #[must_use]
    pub fn pgers(&mut self) -> PGERS_W<SPMCSR_SPEC> {
        PGERS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Page Write"]
    #[inline(always)]
    #[must_use]
    pub fn pgwrt(&mut self) -> PGWRT_W<SPMCSR_SPEC> {
        PGWRT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Read/Write Fuse and Lock Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rwflb(&mut self) -> RWFLB_W<SPMCSR_SPEC> {
        RWFLB_W::new(self, 3)
    }
    #[doc = "Bit 4 - Read-While-Write section read enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwwsre(&mut self) -> RWWSRE_W<SPMCSR_SPEC> {
        RWWSRE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Read Device Signature Imprint Table"]
    #[inline(always)]
    #[must_use]
    pub fn rsig(&mut self) -> RSIG_W<SPMCSR_SPEC> {
        RSIG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Read-While-Write Section Busy"]
    #[inline(always)]
    #[must_use]
    pub fn rwwsb(&mut self) -> RWWSB_W<SPMCSR_SPEC> {
        RWWSB_W::new(self, 6)
    }
    #[doc = "Bit 7 - SPM Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spmie(&mut self) -> SPMIE_W<SPMCSR_SPEC> {
        SPMIE_W::new(self, 7)
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
#[doc = "Store Program Memory Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spmcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spmcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPMCSR_SPEC;
impl crate::RegisterSpec for SPMCSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spmcsr::R`](R) reader structure"]
impl crate::Readable for SPMCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spmcsr::W`](W) writer structure"]
impl crate::Writable for SPMCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPMCSR to value 0"]
impl crate::Resettable for SPMCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
