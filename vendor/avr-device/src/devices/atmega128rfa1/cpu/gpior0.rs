#[doc = "Register `GPIOR0` reader"]
pub type R = crate::R<GPIOR0_SPEC>;
#[doc = "Register `GPIOR0` writer"]
pub type W = crate::W<GPIOR0_SPEC>;
#[doc = "Field `GPIOR00` reader - General Purpose I/O Register 0 Value"]
pub type GPIOR00_R = crate::BitReader;
#[doc = "Field `GPIOR00` writer - General Purpose I/O Register 0 Value"]
pub type GPIOR00_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOR01` reader - General Purpose I/O Register 0 Value"]
pub type GPIOR01_R = crate::BitReader;
#[doc = "Field `GPIOR01` writer - General Purpose I/O Register 0 Value"]
pub type GPIOR01_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOR02` reader - General Purpose I/O Register 0 Value"]
pub type GPIOR02_R = crate::BitReader;
#[doc = "Field `GPIOR02` writer - General Purpose I/O Register 0 Value"]
pub type GPIOR02_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOR03` reader - General Purpose I/O Register 0 Value"]
pub type GPIOR03_R = crate::BitReader;
#[doc = "Field `GPIOR03` writer - General Purpose I/O Register 0 Value"]
pub type GPIOR03_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOR04` reader - General Purpose I/O Register 0 Value"]
pub type GPIOR04_R = crate::BitReader;
#[doc = "Field `GPIOR04` writer - General Purpose I/O Register 0 Value"]
pub type GPIOR04_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOR05` reader - General Purpose I/O Register 0 Value"]
pub type GPIOR05_R = crate::BitReader;
#[doc = "Field `GPIOR05` writer - General Purpose I/O Register 0 Value"]
pub type GPIOR05_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOR06` reader - General Purpose I/O Register 0 Value"]
pub type GPIOR06_R = crate::BitReader;
#[doc = "Field `GPIOR06` writer - General Purpose I/O Register 0 Value"]
pub type GPIOR06_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOR07` reader - General Purpose I/O Register 0 Value"]
pub type GPIOR07_R = crate::BitReader;
#[doc = "Field `GPIOR07` writer - General Purpose I/O Register 0 Value"]
pub type GPIOR07_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    pub fn gpior00(&self) -> GPIOR00_R {
        GPIOR00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    pub fn gpior01(&self) -> GPIOR01_R {
        GPIOR01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    pub fn gpior02(&self) -> GPIOR02_R {
        GPIOR02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    pub fn gpior03(&self) -> GPIOR03_R {
        GPIOR03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    pub fn gpior04(&self) -> GPIOR04_R {
        GPIOR04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    pub fn gpior05(&self) -> GPIOR05_R {
        GPIOR05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    pub fn gpior06(&self) -> GPIOR06_R {
        GPIOR06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    pub fn gpior07(&self) -> GPIOR07_R {
        GPIOR07_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    #[must_use]
    pub fn gpior00(&mut self) -> GPIOR00_W<GPIOR0_SPEC> {
        GPIOR00_W::new(self, 0)
    }
    #[doc = "Bit 1 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    #[must_use]
    pub fn gpior01(&mut self) -> GPIOR01_W<GPIOR0_SPEC> {
        GPIOR01_W::new(self, 1)
    }
    #[doc = "Bit 2 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    #[must_use]
    pub fn gpior02(&mut self) -> GPIOR02_W<GPIOR0_SPEC> {
        GPIOR02_W::new(self, 2)
    }
    #[doc = "Bit 3 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    #[must_use]
    pub fn gpior03(&mut self) -> GPIOR03_W<GPIOR0_SPEC> {
        GPIOR03_W::new(self, 3)
    }
    #[doc = "Bit 4 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    #[must_use]
    pub fn gpior04(&mut self) -> GPIOR04_W<GPIOR0_SPEC> {
        GPIOR04_W::new(self, 4)
    }
    #[doc = "Bit 5 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    #[must_use]
    pub fn gpior05(&mut self) -> GPIOR05_W<GPIOR0_SPEC> {
        GPIOR05_W::new(self, 5)
    }
    #[doc = "Bit 6 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    #[must_use]
    pub fn gpior06(&mut self) -> GPIOR06_W<GPIOR0_SPEC> {
        GPIOR06_W::new(self, 6)
    }
    #[doc = "Bit 7 - General Purpose I/O Register 0 Value"]
    #[inline(always)]
    #[must_use]
    pub fn gpior07(&mut self) -> GPIOR07_W<GPIOR0_SPEC> {
        GPIOR07_W::new(self, 7)
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
#[doc = "General Purpose IO Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpior0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpior0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOR0_SPEC;
impl crate::RegisterSpec for GPIOR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`gpior0::R`](R) reader structure"]
impl crate::Readable for GPIOR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpior0::W`](W) writer structure"]
impl crate::Writable for GPIOR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOR0 to value 0"]
impl crate::Resettable for GPIOR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
