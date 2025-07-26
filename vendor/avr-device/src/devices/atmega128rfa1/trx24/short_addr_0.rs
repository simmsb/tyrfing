#[doc = "Register `SHORT_ADDR_0` reader"]
pub type R = crate::R<SHORT_ADDR_0_SPEC>;
#[doc = "Register `SHORT_ADDR_0` writer"]
pub type W = crate::W<SHORT_ADDR_0_SPEC>;
#[doc = "Field `SHORT_ADDR_00` reader - MAC Short Address"]
pub type SHORT_ADDR_00_R = crate::BitReader;
#[doc = "Field `SHORT_ADDR_00` writer - MAC Short Address"]
pub type SHORT_ADDR_00_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORT_ADDR_01` reader - MAC Short Address"]
pub type SHORT_ADDR_01_R = crate::BitReader;
#[doc = "Field `SHORT_ADDR_01` writer - MAC Short Address"]
pub type SHORT_ADDR_01_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORT_ADDR_02` reader - MAC Short Address"]
pub type SHORT_ADDR_02_R = crate::BitReader;
#[doc = "Field `SHORT_ADDR_02` writer - MAC Short Address"]
pub type SHORT_ADDR_02_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORT_ADDR_03` reader - MAC Short Address"]
pub type SHORT_ADDR_03_R = crate::BitReader;
#[doc = "Field `SHORT_ADDR_03` writer - MAC Short Address"]
pub type SHORT_ADDR_03_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORT_ADDR_04` reader - MAC Short Address"]
pub type SHORT_ADDR_04_R = crate::BitReader;
#[doc = "Field `SHORT_ADDR_04` writer - MAC Short Address"]
pub type SHORT_ADDR_04_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORT_ADDR_05` reader - MAC Short Address"]
pub type SHORT_ADDR_05_R = crate::BitReader;
#[doc = "Field `SHORT_ADDR_05` writer - MAC Short Address"]
pub type SHORT_ADDR_05_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORT_ADDR_06` reader - MAC Short Address"]
pub type SHORT_ADDR_06_R = crate::BitReader;
#[doc = "Field `SHORT_ADDR_06` writer - MAC Short Address"]
pub type SHORT_ADDR_06_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORT_ADDR_07` reader - MAC Short Address"]
pub type SHORT_ADDR_07_R = crate::BitReader;
#[doc = "Field `SHORT_ADDR_07` writer - MAC Short Address"]
pub type SHORT_ADDR_07_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MAC Short Address"]
    #[inline(always)]
    pub fn short_addr_00(&self) -> SHORT_ADDR_00_R {
        SHORT_ADDR_00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MAC Short Address"]
    #[inline(always)]
    pub fn short_addr_01(&self) -> SHORT_ADDR_01_R {
        SHORT_ADDR_01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MAC Short Address"]
    #[inline(always)]
    pub fn short_addr_02(&self) -> SHORT_ADDR_02_R {
        SHORT_ADDR_02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MAC Short Address"]
    #[inline(always)]
    pub fn short_addr_03(&self) -> SHORT_ADDR_03_R {
        SHORT_ADDR_03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MAC Short Address"]
    #[inline(always)]
    pub fn short_addr_04(&self) -> SHORT_ADDR_04_R {
        SHORT_ADDR_04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MAC Short Address"]
    #[inline(always)]
    pub fn short_addr_05(&self) -> SHORT_ADDR_05_R {
        SHORT_ADDR_05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MAC Short Address"]
    #[inline(always)]
    pub fn short_addr_06(&self) -> SHORT_ADDR_06_R {
        SHORT_ADDR_06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MAC Short Address"]
    #[inline(always)]
    pub fn short_addr_07(&self) -> SHORT_ADDR_07_R {
        SHORT_ADDR_07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MAC Short Address"]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_00(&mut self) -> SHORT_ADDR_00_W<SHORT_ADDR_0_SPEC> {
        SHORT_ADDR_00_W::new(self, 0)
    }
    #[doc = "Bit 1 - MAC Short Address"]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_01(&mut self) -> SHORT_ADDR_01_W<SHORT_ADDR_0_SPEC> {
        SHORT_ADDR_01_W::new(self, 1)
    }
    #[doc = "Bit 2 - MAC Short Address"]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_02(&mut self) -> SHORT_ADDR_02_W<SHORT_ADDR_0_SPEC> {
        SHORT_ADDR_02_W::new(self, 2)
    }
    #[doc = "Bit 3 - MAC Short Address"]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_03(&mut self) -> SHORT_ADDR_03_W<SHORT_ADDR_0_SPEC> {
        SHORT_ADDR_03_W::new(self, 3)
    }
    #[doc = "Bit 4 - MAC Short Address"]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_04(&mut self) -> SHORT_ADDR_04_W<SHORT_ADDR_0_SPEC> {
        SHORT_ADDR_04_W::new(self, 4)
    }
    #[doc = "Bit 5 - MAC Short Address"]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_05(&mut self) -> SHORT_ADDR_05_W<SHORT_ADDR_0_SPEC> {
        SHORT_ADDR_05_W::new(self, 5)
    }
    #[doc = "Bit 6 - MAC Short Address"]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_06(&mut self) -> SHORT_ADDR_06_W<SHORT_ADDR_0_SPEC> {
        SHORT_ADDR_06_W::new(self, 6)
    }
    #[doc = "Bit 7 - MAC Short Address"]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_07(&mut self) -> SHORT_ADDR_07_W<SHORT_ADDR_0_SPEC> {
        SHORT_ADDR_07_W::new(self, 7)
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
#[doc = "Transceiver MAC Short Address Register (Low Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`short_addr_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`short_addr_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHORT_ADDR_0_SPEC;
impl crate::RegisterSpec for SHORT_ADDR_0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`short_addr_0::R`](R) reader structure"]
impl crate::Readable for SHORT_ADDR_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`short_addr_0::W`](W) writer structure"]
impl crate::Writable for SHORT_ADDR_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHORT_ADDR_0 to value 0"]
impl crate::Resettable for SHORT_ADDR_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
