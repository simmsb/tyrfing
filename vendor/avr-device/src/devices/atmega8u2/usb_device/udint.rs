#[doc = "Register `UDINT` reader"]
pub type R = crate::R<UDINT_SPEC>;
#[doc = "Register `UDINT` writer"]
pub type W = crate::W<UDINT_SPEC>;
#[doc = "Field `SUSPI` reader - Suspend Interrupt Flag"]
pub type SUSPI_R = crate::BitReader;
#[doc = "Field `SUSPI` writer - Suspend Interrupt Flag"]
pub type SUSPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFI` reader - Start Of Frame Interrupt Flag"]
pub type SOFI_R = crate::BitReader;
#[doc = "Field `SOFI` writer - Start Of Frame Interrupt Flag"]
pub type SOFI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSTI` reader - End Of Reset Interrupt Flag"]
pub type EORSTI_R = crate::BitReader;
#[doc = "Field `EORSTI` writer - End Of Reset Interrupt Flag"]
pub type EORSTI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPI` reader - Wake-up CPU Interrupt Flag"]
pub type WAKEUPI_R = crate::BitReader;
#[doc = "Field `WAKEUPI` writer - Wake-up CPU Interrupt Flag"]
pub type WAKEUPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EORSMI` reader - End Of Resume Interrupt Flag"]
pub type EORSMI_R = crate::BitReader;
#[doc = "Field `EORSMI` writer - End Of Resume Interrupt Flag"]
pub type EORSMI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPRSMI` reader - Upstream Resume Interrupt Flag"]
pub type UPRSMI_R = crate::BitReader;
#[doc = "Field `UPRSMI` writer - Upstream Resume Interrupt Flag"]
pub type UPRSMI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Suspend Interrupt Flag"]
    #[inline(always)]
    pub fn suspi(&self) -> SUSPI_R {
        SUSPI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Start Of Frame Interrupt Flag"]
    #[inline(always)]
    pub fn sofi(&self) -> SOFI_R {
        SOFI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End Of Reset Interrupt Flag"]
    #[inline(always)]
    pub fn eorsti(&self) -> EORSTI_R {
        EORSTI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-up CPU Interrupt Flag"]
    #[inline(always)]
    pub fn wakeupi(&self) -> WAKEUPI_R {
        WAKEUPI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End Of Resume Interrupt Flag"]
    #[inline(always)]
    pub fn eorsmi(&self) -> EORSMI_R {
        EORSMI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Flag"]
    #[inline(always)]
    pub fn uprsmi(&self) -> UPRSMI_R {
        UPRSMI_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn suspi(&mut self) -> SUSPI_W<UDINT_SPEC> {
        SUSPI_W::new(self, 0)
    }
    #[doc = "Bit 2 - Start Of Frame Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sofi(&mut self) -> SOFI_W<UDINT_SPEC> {
        SOFI_W::new(self, 2)
    }
    #[doc = "Bit 3 - End Of Reset Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn eorsti(&mut self) -> EORSTI_W<UDINT_SPEC> {
        EORSTI_W::new(self, 3)
    }
    #[doc = "Bit 4 - Wake-up CPU Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupi(&mut self) -> WAKEUPI_W<UDINT_SPEC> {
        WAKEUPI_W::new(self, 4)
    }
    #[doc = "Bit 5 - End Of Resume Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn eorsmi(&mut self) -> EORSMI_W<UDINT_SPEC> {
        EORSMI_W::new(self, 5)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn uprsmi(&mut self) -> UPRSMI_W<UDINT_SPEC> {
        UPRSMI_W::new(self, 6)
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
#[doc = "USB Device Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDINT_SPEC;
impl crate::RegisterSpec for UDINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`udint::R`](R) reader structure"]
impl crate::Readable for UDINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udint::W`](W) writer structure"]
impl crate::Writable for UDINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDINT to value 0"]
impl crate::Resettable for UDINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
