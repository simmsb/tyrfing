#[doc = "Register `PHY_ED_LEVEL` reader"]
pub type R = crate::R<PHY_ED_LEVEL_SPEC>;
#[doc = "Register `PHY_ED_LEVEL` writer"]
pub type W = crate::W<PHY_ED_LEVEL_SPEC>;
#[doc = "Field `ED_LEVEL` reader - Energy Detection Level"]
pub type ED_LEVEL_R = crate::FieldReader<ED_LEVEL_A>;
#[doc = "Energy Detection Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ED_LEVEL_A {
    #[doc = "0: Minimum result of last ED measurement"]
    ED_MIN = 0,
    #[doc = "1: P(RF) = RSSI_BASE_VAL+ED \\[dBm\\]"]
    ED_MIN_PLUS_1D_B = 1,
    #[doc = "2: ..."]
    VAL_0X02 = 2,
    #[doc = "84: Maximum result of last ED measurement"]
    ED_MAX = 84,
    #[doc = "255: Reset value"]
    ED_RESET = 255,
}
impl From<ED_LEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ED_LEVEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ED_LEVEL_A {
    type Ux = u8;
}
impl ED_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ED_LEVEL_A> {
        match self.bits {
            0 => Some(ED_LEVEL_A::ED_MIN),
            1 => Some(ED_LEVEL_A::ED_MIN_PLUS_1D_B),
            2 => Some(ED_LEVEL_A::VAL_0X02),
            84 => Some(ED_LEVEL_A::ED_MAX),
            255 => Some(ED_LEVEL_A::ED_RESET),
            _ => None,
        }
    }
    #[doc = "Minimum result of last ED measurement"]
    #[inline(always)]
    pub fn is_ed_min(&self) -> bool {
        *self == ED_LEVEL_A::ED_MIN
    }
    #[doc = "P(RF) = RSSI_BASE_VAL+ED \\[dBm\\]"]
    #[inline(always)]
    pub fn is_ed_min_plus_1d_b(&self) -> bool {
        *self == ED_LEVEL_A::ED_MIN_PLUS_1D_B
    }
    #[doc = "..."]
    #[inline(always)]
    pub fn is_val_0x02(&self) -> bool {
        *self == ED_LEVEL_A::VAL_0X02
    }
    #[doc = "Maximum result of last ED measurement"]
    #[inline(always)]
    pub fn is_ed_max(&self) -> bool {
        *self == ED_LEVEL_A::ED_MAX
    }
    #[doc = "Reset value"]
    #[inline(always)]
    pub fn is_ed_reset(&self) -> bool {
        *self == ED_LEVEL_A::ED_RESET
    }
}
#[doc = "Field `ED_LEVEL` writer - Energy Detection Level"]
pub type ED_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, ED_LEVEL_A>;
impl<'a, REG> ED_LEVEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minimum result of last ED measurement"]
    #[inline(always)]
    pub fn ed_min(self) -> &'a mut crate::W<REG> {
        self.variant(ED_LEVEL_A::ED_MIN)
    }
    #[doc = "P(RF) = RSSI_BASE_VAL+ED \\[dBm\\]"]
    #[inline(always)]
    pub fn ed_min_plus_1d_b(self) -> &'a mut crate::W<REG> {
        self.variant(ED_LEVEL_A::ED_MIN_PLUS_1D_B)
    }
    #[doc = "..."]
    #[inline(always)]
    pub fn val_0x02(self) -> &'a mut crate::W<REG> {
        self.variant(ED_LEVEL_A::VAL_0X02)
    }
    #[doc = "Maximum result of last ED measurement"]
    #[inline(always)]
    pub fn ed_max(self) -> &'a mut crate::W<REG> {
        self.variant(ED_LEVEL_A::ED_MAX)
    }
    #[doc = "Reset value"]
    #[inline(always)]
    pub fn ed_reset(self) -> &'a mut crate::W<REG> {
        self.variant(ED_LEVEL_A::ED_RESET)
    }
}
impl R {
    #[doc = "Bits 0:7 - Energy Detection Level"]
    #[inline(always)]
    pub fn ed_level(&self) -> ED_LEVEL_R {
        ED_LEVEL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Energy Detection Level"]
    #[inline(always)]
    #[must_use]
    pub fn ed_level(&mut self) -> ED_LEVEL_W<PHY_ED_LEVEL_SPEC> {
        ED_LEVEL_W::new(self, 0)
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
#[doc = "Transceiver Energy Detection Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_ed_level::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_ed_level::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_ED_LEVEL_SPEC;
impl crate::RegisterSpec for PHY_ED_LEVEL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_ed_level::R`](R) reader structure"]
impl crate::Readable for PHY_ED_LEVEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_ed_level::W`](W) writer structure"]
impl crate::Writable for PHY_ED_LEVEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHY_ED_LEVEL to value 0"]
impl crate::Resettable for PHY_ED_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
