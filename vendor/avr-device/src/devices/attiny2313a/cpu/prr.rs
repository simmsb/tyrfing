#[doc = "Register `PRR` reader"]
pub type R = crate::R<PRR_SPEC>;
#[doc = "Register `PRR` writer"]
pub type W = crate::W<PRR_SPEC>;
#[doc = "Field `PRUSART` reader - No Description."]
pub type PRUSART_R = crate::BitReader;
#[doc = "Field `PRUSART` writer - No Description."]
pub type PRUSART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRUSI` reader - No Description."]
pub type PRUSI_R = crate::BitReader;
#[doc = "Field `PRUSI` writer - No Description."]
pub type PRUSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTIM` reader - No Description."]
pub type PRTIM_R = crate::FieldReader;
#[doc = "Field `PRTIM` writer - No Description."]
pub type PRTIM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    pub fn prusart(&self) -> PRUSART_R {
        PRUSART_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    pub fn prusi(&self) -> PRUSI_R {
        PRUSI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - No Description."]
    #[inline(always)]
    pub fn prtim(&self) -> PRTIM_R {
        PRTIM_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn prusart(&mut self) -> PRUSART_W<PRR_SPEC> {
        PRUSART_W::new(self, 0)
    }
    #[doc = "Bit 1 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn prusi(&mut self) -> PRUSI_W<PRR_SPEC> {
        PRUSI_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn prtim(&mut self) -> PRTIM_W<PRR_SPEC> {
        PRTIM_W::new(self, 2)
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
#[doc = "Power reduction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRR_SPEC;
impl crate::RegisterSpec for PRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prr::R`](R) reader structure"]
impl crate::Readable for PRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prr::W`](W) writer structure"]
impl crate::Writable for PRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRR to value 0"]
impl crate::Resettable for PRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
