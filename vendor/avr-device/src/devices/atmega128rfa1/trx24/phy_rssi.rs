#[doc = "Register `PHY_RSSI` reader"]
pub type R = crate::R<PHY_RSSI_SPEC>;
#[doc = "Register `PHY_RSSI` writer"]
pub type W = crate::W<PHY_RSSI_SPEC>;
#[doc = "Field `RSSI` reader - Receiver Signal Strength Indicator"]
pub type RSSI_R = crate::FieldReader<RSSI_A>;
#[doc = "Receiver Signal Strength Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSSI_A {
    #[doc = "0: Minimum RSSI value: P(RF) &lt; -90 dBm"]
    RSSI_MIN = 0,
    #[doc = "1: P(RF) = RSSI_BASE_VAL+3 · (RSSI-1) \\[dBm\\]"]
    RSSI_MIN_PLUS_3D_B = 1,
    #[doc = "2: ..."]
    VAL_2 = 2,
    #[doc = "28: Maximum RSSI value: P(RF) ≥ -10 dBm"]
    RSSI_MAX = 28,
}
impl From<RSSI_A> for u8 {
    #[inline(always)]
    fn from(variant: RSSI_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSSI_A {
    type Ux = u8;
}
impl RSSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RSSI_A> {
        match self.bits {
            0 => Some(RSSI_A::RSSI_MIN),
            1 => Some(RSSI_A::RSSI_MIN_PLUS_3D_B),
            2 => Some(RSSI_A::VAL_2),
            28 => Some(RSSI_A::RSSI_MAX),
            _ => None,
        }
    }
    #[doc = "Minimum RSSI value: P(RF) &lt; -90 dBm"]
    #[inline(always)]
    pub fn is_rssi_min(&self) -> bool {
        *self == RSSI_A::RSSI_MIN
    }
    #[doc = "P(RF) = RSSI_BASE_VAL+3 · (RSSI-1) \\[dBm\\]"]
    #[inline(always)]
    pub fn is_rssi_min_plus_3d_b(&self) -> bool {
        *self == RSSI_A::RSSI_MIN_PLUS_3D_B
    }
    #[doc = "..."]
    #[inline(always)]
    pub fn is_val_2(&self) -> bool {
        *self == RSSI_A::VAL_2
    }
    #[doc = "Maximum RSSI value: P(RF) ≥ -10 dBm"]
    #[inline(always)]
    pub fn is_rssi_max(&self) -> bool {
        *self == RSSI_A::RSSI_MAX
    }
}
#[doc = "Field `RSSI` writer - Receiver Signal Strength Indicator"]
pub type RSSI_W<'a, REG> = crate::FieldWriter<'a, REG, 5, RSSI_A>;
impl<'a, REG> RSSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minimum RSSI value: P(RF) &lt; -90 dBm"]
    #[inline(always)]
    pub fn rssi_min(self) -> &'a mut crate::W<REG> {
        self.variant(RSSI_A::RSSI_MIN)
    }
    #[doc = "P(RF) = RSSI_BASE_VAL+3 · (RSSI-1) \\[dBm\\]"]
    #[inline(always)]
    pub fn rssi_min_plus_3d_b(self) -> &'a mut crate::W<REG> {
        self.variant(RSSI_A::RSSI_MIN_PLUS_3D_B)
    }
    #[doc = "..."]
    #[inline(always)]
    pub fn val_2(self) -> &'a mut crate::W<REG> {
        self.variant(RSSI_A::VAL_2)
    }
    #[doc = "Maximum RSSI value: P(RF) ≥ -10 dBm"]
    #[inline(always)]
    pub fn rssi_max(self) -> &'a mut crate::W<REG> {
        self.variant(RSSI_A::RSSI_MAX)
    }
}
#[doc = "Field `RND_VALUE` reader - Random Value"]
pub type RND_VALUE_R = crate::FieldReader;
#[doc = "Field `RND_VALUE` writer - Random Value"]
pub type RND_VALUE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `RX_CRC_VALID` reader - Received Frame CRC Status"]
pub type RX_CRC_VALID_R = crate::BitReader<RX_CRC_VALID_A>;
#[doc = "Received Frame CRC Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_CRC_VALID_A {
    #[doc = "0: CRC (FCS) not valid"]
    CRC_INVALID = 0,
    #[doc = "1: CRC (FCS) valid"]
    CRC_VALID = 1,
}
impl From<RX_CRC_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: RX_CRC_VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_CRC_VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_CRC_VALID_A {
        match self.bits {
            false => RX_CRC_VALID_A::CRC_INVALID,
            true => RX_CRC_VALID_A::CRC_VALID,
        }
    }
    #[doc = "CRC (FCS) not valid"]
    #[inline(always)]
    pub fn is_crc_invalid(&self) -> bool {
        *self == RX_CRC_VALID_A::CRC_INVALID
    }
    #[doc = "CRC (FCS) valid"]
    #[inline(always)]
    pub fn is_crc_valid(&self) -> bool {
        *self == RX_CRC_VALID_A::CRC_VALID
    }
}
#[doc = "Field `RX_CRC_VALID` writer - Received Frame CRC Status"]
pub type RX_CRC_VALID_W<'a, REG> = crate::BitWriter<'a, REG, RX_CRC_VALID_A>;
impl<'a, REG> RX_CRC_VALID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CRC (FCS) not valid"]
    #[inline(always)]
    pub fn crc_invalid(self) -> &'a mut crate::W<REG> {
        self.variant(RX_CRC_VALID_A::CRC_INVALID)
    }
    #[doc = "CRC (FCS) valid"]
    #[inline(always)]
    pub fn crc_valid(self) -> &'a mut crate::W<REG> {
        self.variant(RX_CRC_VALID_A::CRC_VALID)
    }
}
impl R {
    #[doc = "Bits 0:4 - Receiver Signal Strength Indicator"]
    #[inline(always)]
    pub fn rssi(&self) -> RSSI_R {
        RSSI_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:6 - Random Value"]
    #[inline(always)]
    pub fn rnd_value(&self) -> RND_VALUE_R {
        RND_VALUE_R::new((self.bits >> 5) & 3)
    }
    #[doc = "Bit 7 - Received Frame CRC Status"]
    #[inline(always)]
    pub fn rx_crc_valid(&self) -> RX_CRC_VALID_R {
        RX_CRC_VALID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Receiver Signal Strength Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn rssi(&mut self) -> RSSI_W<PHY_RSSI_SPEC> {
        RSSI_W::new(self, 0)
    }
    #[doc = "Bits 5:6 - Random Value"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_value(&mut self) -> RND_VALUE_W<PHY_RSSI_SPEC> {
        RND_VALUE_W::new(self, 5)
    }
    #[doc = "Bit 7 - Received Frame CRC Status"]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_valid(&mut self) -> RX_CRC_VALID_W<PHY_RSSI_SPEC> {
        RX_CRC_VALID_W::new(self, 7)
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
#[doc = "Receiver Signal Strength Indicator Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_rssi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_rssi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_RSSI_SPEC;
impl crate::RegisterSpec for PHY_RSSI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_rssi::R`](R) reader structure"]
impl crate::Readable for PHY_RSSI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_rssi::W`](W) writer structure"]
impl crate::Writable for PHY_RSSI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHY_RSSI to value 0"]
impl crate::Resettable for PHY_RSSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
