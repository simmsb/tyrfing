#[doc = "Register `CSMA_SEED_0` reader"]
pub type R = crate::R<CSMA_SEED_0_SPEC>;
#[doc = "Register `CSMA_SEED_0` writer"]
pub type W = crate::W<CSMA_SEED_0_SPEC>;
#[doc = "Field `CSMA_SEED_00` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_00_R = crate::BitReader;
#[doc = "Field `CSMA_SEED_00` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_00_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMA_SEED_01` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_01_R = crate::BitReader;
#[doc = "Field `CSMA_SEED_01` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_01_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMA_SEED_02` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_02_R = crate::BitReader;
#[doc = "Field `CSMA_SEED_02` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_02_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMA_SEED_03` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_03_R = crate::BitReader;
#[doc = "Field `CSMA_SEED_03` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_03_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMA_SEED_04` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_04_R = crate::BitReader;
#[doc = "Field `CSMA_SEED_04` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_04_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMA_SEED_05` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_05_R = crate::BitReader;
#[doc = "Field `CSMA_SEED_05` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_05_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMA_SEED_06` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_06_R = crate::BitReader;
#[doc = "Field `CSMA_SEED_06` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_06_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMA_SEED_07` reader - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_07_R = crate::BitReader;
#[doc = "Field `CSMA_SEED_07` writer - Seed Value for CSMA Random Number Generator"]
pub type CSMA_SEED_07_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_00(&self) -> CSMA_SEED_00_R {
        CSMA_SEED_00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_01(&self) -> CSMA_SEED_01_R {
        CSMA_SEED_01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_02(&self) -> CSMA_SEED_02_R {
        CSMA_SEED_02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_03(&self) -> CSMA_SEED_03_R {
        CSMA_SEED_03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_04(&self) -> CSMA_SEED_04_R {
        CSMA_SEED_04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_05(&self) -> CSMA_SEED_05_R {
        CSMA_SEED_05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_06(&self) -> CSMA_SEED_06_R {
        CSMA_SEED_06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    pub fn csma_seed_07(&self) -> CSMA_SEED_07_R {
        CSMA_SEED_07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_00(&mut self) -> CSMA_SEED_00_W<CSMA_SEED_0_SPEC> {
        CSMA_SEED_00_W::new(self, 0)
    }
    #[doc = "Bit 1 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_01(&mut self) -> CSMA_SEED_01_W<CSMA_SEED_0_SPEC> {
        CSMA_SEED_01_W::new(self, 1)
    }
    #[doc = "Bit 2 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_02(&mut self) -> CSMA_SEED_02_W<CSMA_SEED_0_SPEC> {
        CSMA_SEED_02_W::new(self, 2)
    }
    #[doc = "Bit 3 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_03(&mut self) -> CSMA_SEED_03_W<CSMA_SEED_0_SPEC> {
        CSMA_SEED_03_W::new(self, 3)
    }
    #[doc = "Bit 4 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_04(&mut self) -> CSMA_SEED_04_W<CSMA_SEED_0_SPEC> {
        CSMA_SEED_04_W::new(self, 4)
    }
    #[doc = "Bit 5 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_05(&mut self) -> CSMA_SEED_05_W<CSMA_SEED_0_SPEC> {
        CSMA_SEED_05_W::new(self, 5)
    }
    #[doc = "Bit 6 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_06(&mut self) -> CSMA_SEED_06_W<CSMA_SEED_0_SPEC> {
        CSMA_SEED_06_W::new(self, 6)
    }
    #[doc = "Bit 7 - Seed Value for CSMA Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn csma_seed_07(&mut self) -> CSMA_SEED_07_W<CSMA_SEED_0_SPEC> {
        CSMA_SEED_07_W::new(self, 7)
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
#[doc = "Transceiver CSMA-CA Random Number Generator Seed Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csma_seed_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csma_seed_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSMA_SEED_0_SPEC;
impl crate::RegisterSpec for CSMA_SEED_0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csma_seed_0::R`](R) reader structure"]
impl crate::Readable for CSMA_SEED_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csma_seed_0::W`](W) writer structure"]
impl crate::Writable for CSMA_SEED_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSMA_SEED_0 to value 0"]
impl crate::Resettable for CSMA_SEED_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
