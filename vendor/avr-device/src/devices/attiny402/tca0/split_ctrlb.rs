#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<SPLIT_CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<SPLIT_CTRLB_SPEC>;
#[doc = "Field `LCMP0EN` reader - Low Compare 0 Enable"]
pub type LCMP0EN_R = crate::BitReader;
#[doc = "Field `LCMP0EN` writer - Low Compare 0 Enable"]
pub type LCMP0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCMP1EN` reader - Low Compare 1 Enable"]
pub type LCMP1EN_R = crate::BitReader;
#[doc = "Field `LCMP1EN` writer - Low Compare 1 Enable"]
pub type LCMP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCMP2EN` reader - Low Compare 2 Enable"]
pub type LCMP2EN_R = crate::BitReader;
#[doc = "Field `LCMP2EN` writer - Low Compare 2 Enable"]
pub type LCMP2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCMP0EN` reader - High Compare 0 Enable"]
pub type HCMP0EN_R = crate::BitReader;
#[doc = "Field `HCMP0EN` writer - High Compare 0 Enable"]
pub type HCMP0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCMP1EN` reader - High Compare 1 Enable"]
pub type HCMP1EN_R = crate::BitReader;
#[doc = "Field `HCMP1EN` writer - High Compare 1 Enable"]
pub type HCMP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCMP2EN` reader - High Compare 2 Enable"]
pub type HCMP2EN_R = crate::BitReader;
#[doc = "Field `HCMP2EN` writer - High Compare 2 Enable"]
pub type HCMP2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Compare 0 Enable"]
    #[inline(always)]
    pub fn lcmp0en(&self) -> LCMP0EN_R {
        LCMP0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Compare 1 Enable"]
    #[inline(always)]
    pub fn lcmp1en(&self) -> LCMP1EN_R {
        LCMP1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Compare 2 Enable"]
    #[inline(always)]
    pub fn lcmp2en(&self) -> LCMP2EN_R {
        LCMP2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - High Compare 0 Enable"]
    #[inline(always)]
    pub fn hcmp0en(&self) -> HCMP0EN_R {
        HCMP0EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High Compare 1 Enable"]
    #[inline(always)]
    pub fn hcmp1en(&self) -> HCMP1EN_R {
        HCMP1EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High Compare 2 Enable"]
    #[inline(always)]
    pub fn hcmp2en(&self) -> HCMP2EN_R {
        HCMP2EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Compare 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp0en(&mut self) -> LCMP0EN_W<SPLIT_CTRLB_SPEC> {
        LCMP0EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Low Compare 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp1en(&mut self) -> LCMP1EN_W<SPLIT_CTRLB_SPEC> {
        LCMP1EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Low Compare 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp2en(&mut self) -> LCMP2EN_W<SPLIT_CTRLB_SPEC> {
        LCMP2EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - High Compare 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hcmp0en(&mut self) -> HCMP0EN_W<SPLIT_CTRLB_SPEC> {
        HCMP0EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - High Compare 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hcmp1en(&mut self) -> HCMP1EN_W<SPLIT_CTRLB_SPEC> {
        HCMP1EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - High Compare 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hcmp2en(&mut self) -> HCMP2EN_W<SPLIT_CTRLB_SPEC> {
        HCMP2EN_W::new(self, 6)
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
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPLIT_CTRLB_SPEC;
impl crate::RegisterSpec for SPLIT_CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`split_ctrlb::R`](R) reader structure"]
impl crate::Readable for SPLIT_CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`split_ctrlb::W`](W) writer structure"]
impl crate::Writable for SPLIT_CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for SPLIT_CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
