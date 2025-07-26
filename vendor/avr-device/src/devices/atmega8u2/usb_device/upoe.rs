#[doc = "Register `UPOE` reader"]
pub type R = crate::R<UPOE_SPEC>;
#[doc = "Register `UPOE` writer"]
pub type W = crate::W<UPOE_SPEC>;
#[doc = "Field `DMI` reader - D- Input value"]
pub type DMI_R = crate::BitReader;
#[doc = "Field `DMI` writer - D- Input value"]
pub type DMI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPI` reader - D+ Input value"]
pub type DPI_R = crate::BitReader;
#[doc = "Field `DPI` writer - D+ Input value"]
pub type DPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDRV` reader - USB direct drive values"]
pub type UPDRV_R = crate::FieldReader;
#[doc = "Field `UPDRV` writer - USB direct drive values"]
pub type UPDRV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `UPWE` reader - USB Buffers Direct Drive enable configuration"]
pub type UPWE_R = crate::FieldReader;
#[doc = "Field `UPWE` writer - USB Buffers Direct Drive enable configuration"]
pub type UPWE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - D- Input value"]
    #[inline(always)]
    pub fn dmi(&self) -> DMI_R {
        DMI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - D+ Input value"]
    #[inline(always)]
    pub fn dpi(&self) -> DPI_R {
        DPI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USB direct drive values"]
    #[inline(always)]
    pub fn updrv(&self) -> UPDRV_R {
        UPDRV_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - USB Buffers Direct Drive enable configuration"]
    #[inline(always)]
    pub fn upwe(&self) -> UPWE_R {
        UPWE_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - D- Input value"]
    #[inline(always)]
    #[must_use]
    pub fn dmi(&mut self) -> DMI_W<UPOE_SPEC> {
        DMI_W::new(self, 0)
    }
    #[doc = "Bit 1 - D+ Input value"]
    #[inline(always)]
    #[must_use]
    pub fn dpi(&mut self) -> DPI_W<UPOE_SPEC> {
        DPI_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - USB direct drive values"]
    #[inline(always)]
    #[must_use]
    pub fn updrv(&mut self) -> UPDRV_W<UPOE_SPEC> {
        UPDRV_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - USB Buffers Direct Drive enable configuration"]
    #[inline(always)]
    #[must_use]
    pub fn upwe(&mut self) -> UPWE_W<UPOE_SPEC> {
        UPWE_W::new(self, 6)
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
#[doc = "USB Software Output Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upoe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upoe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPOE_SPEC;
impl crate::RegisterSpec for UPOE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`upoe::R`](R) reader structure"]
impl crate::Readable for UPOE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`upoe::W`](W) writer structure"]
impl crate::Writable for UPOE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPOE to value 0"]
impl crate::Resettable for UPOE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
