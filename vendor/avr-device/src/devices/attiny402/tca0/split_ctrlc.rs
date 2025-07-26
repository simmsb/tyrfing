#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<SPLIT_CTRLC_SPEC>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<SPLIT_CTRLC_SPEC>;
#[doc = "Field `LCMP0OV` reader - Low Compare 0 Output Value"]
pub type LCMP0OV_R = crate::BitReader;
#[doc = "Field `LCMP0OV` writer - Low Compare 0 Output Value"]
pub type LCMP0OV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCMP1OV` reader - Low Compare 1 Output Value"]
pub type LCMP1OV_R = crate::BitReader;
#[doc = "Field `LCMP1OV` writer - Low Compare 1 Output Value"]
pub type LCMP1OV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCMP2OV` reader - Low Compare 2 Output Value"]
pub type LCMP2OV_R = crate::BitReader;
#[doc = "Field `LCMP2OV` writer - Low Compare 2 Output Value"]
pub type LCMP2OV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCMP0OV` reader - High Compare 0 Output Value"]
pub type HCMP0OV_R = crate::BitReader;
#[doc = "Field `HCMP0OV` writer - High Compare 0 Output Value"]
pub type HCMP0OV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCMP1OV` reader - High Compare 1 Output Value"]
pub type HCMP1OV_R = crate::BitReader;
#[doc = "Field `HCMP1OV` writer - High Compare 1 Output Value"]
pub type HCMP1OV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCMP2OV` reader - High Compare 2 Output Value"]
pub type HCMP2OV_R = crate::BitReader;
#[doc = "Field `HCMP2OV` writer - High Compare 2 Output Value"]
pub type HCMP2OV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low Compare 0 Output Value"]
    #[inline(always)]
    pub fn lcmp0ov(&self) -> LCMP0OV_R {
        LCMP0OV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Compare 1 Output Value"]
    #[inline(always)]
    pub fn lcmp1ov(&self) -> LCMP1OV_R {
        LCMP1OV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Compare 2 Output Value"]
    #[inline(always)]
    pub fn lcmp2ov(&self) -> LCMP2OV_R {
        LCMP2OV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - High Compare 0 Output Value"]
    #[inline(always)]
    pub fn hcmp0ov(&self) -> HCMP0OV_R {
        HCMP0OV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High Compare 1 Output Value"]
    #[inline(always)]
    pub fn hcmp1ov(&self) -> HCMP1OV_R {
        HCMP1OV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High Compare 2 Output Value"]
    #[inline(always)]
    pub fn hcmp2ov(&self) -> HCMP2OV_R {
        HCMP2OV_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Compare 0 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp0ov(&mut self) -> LCMP0OV_W<SPLIT_CTRLC_SPEC> {
        LCMP0OV_W::new(self, 0)
    }
    #[doc = "Bit 1 - Low Compare 1 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp1ov(&mut self) -> LCMP1OV_W<SPLIT_CTRLC_SPEC> {
        LCMP1OV_W::new(self, 1)
    }
    #[doc = "Bit 2 - Low Compare 2 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn lcmp2ov(&mut self) -> LCMP2OV_W<SPLIT_CTRLC_SPEC> {
        LCMP2OV_W::new(self, 2)
    }
    #[doc = "Bit 4 - High Compare 0 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn hcmp0ov(&mut self) -> HCMP0OV_W<SPLIT_CTRLC_SPEC> {
        HCMP0OV_W::new(self, 4)
    }
    #[doc = "Bit 5 - High Compare 1 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn hcmp1ov(&mut self) -> HCMP1OV_W<SPLIT_CTRLC_SPEC> {
        HCMP1OV_W::new(self, 5)
    }
    #[doc = "Bit 6 - High Compare 2 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn hcmp2ov(&mut self) -> HCMP2OV_W<SPLIT_CTRLC_SPEC> {
        HCMP2OV_W::new(self, 6)
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
#[doc = "Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_ctrlc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_ctrlc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPLIT_CTRLC_SPEC;
impl crate::RegisterSpec for SPLIT_CTRLC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`split_ctrlc::R`](R) reader structure"]
impl crate::Readable for SPLIT_CTRLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`split_ctrlc::W`](W) writer structure"]
impl crate::Writable for SPLIT_CTRLC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for SPLIT_CTRLC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
