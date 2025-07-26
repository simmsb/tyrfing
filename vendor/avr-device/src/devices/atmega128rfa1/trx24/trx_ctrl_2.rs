#[doc = "Register `TRX_CTRL_2` reader"]
pub type R = crate::R<TRX_CTRL_2_SPEC>;
#[doc = "Register `TRX_CTRL_2` writer"]
pub type W = crate::W<TRX_CTRL_2_SPEC>;
#[doc = "Field `OQPSK_DATA_RATE` reader - Data Rate Selection"]
pub type OQPSK_DATA_RATE_R = crate::FieldReader<OQPSK_DATA_RATE_A>;
#[doc = "Data Rate Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OQPSK_DATA_RATE_A {
    #[doc = "0: 250 kb/s (IEEE 802.15.4 compliant)"]
    RATE_250KB = 0,
    #[doc = "1: 500 kb/s"]
    RATE_500KB = 1,
    #[doc = "2: 1000 kb/s"]
    RATE_1000KB = 2,
    #[doc = "3: 2000 kb/s"]
    RATE_2000KB = 3,
}
impl From<OQPSK_DATA_RATE_A> for u8 {
    #[inline(always)]
    fn from(variant: OQPSK_DATA_RATE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OQPSK_DATA_RATE_A {
    type Ux = u8;
}
impl OQPSK_DATA_RATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OQPSK_DATA_RATE_A {
        match self.bits {
            0 => OQPSK_DATA_RATE_A::RATE_250KB,
            1 => OQPSK_DATA_RATE_A::RATE_500KB,
            2 => OQPSK_DATA_RATE_A::RATE_1000KB,
            3 => OQPSK_DATA_RATE_A::RATE_2000KB,
            _ => unreachable!(),
        }
    }
    #[doc = "250 kb/s (IEEE 802.15.4 compliant)"]
    #[inline(always)]
    pub fn is_rate_250kb(&self) -> bool {
        *self == OQPSK_DATA_RATE_A::RATE_250KB
    }
    #[doc = "500 kb/s"]
    #[inline(always)]
    pub fn is_rate_500kb(&self) -> bool {
        *self == OQPSK_DATA_RATE_A::RATE_500KB
    }
    #[doc = "1000 kb/s"]
    #[inline(always)]
    pub fn is_rate_1000kb(&self) -> bool {
        *self == OQPSK_DATA_RATE_A::RATE_1000KB
    }
    #[doc = "2000 kb/s"]
    #[inline(always)]
    pub fn is_rate_2000kb(&self) -> bool {
        *self == OQPSK_DATA_RATE_A::RATE_2000KB
    }
}
#[doc = "Field `OQPSK_DATA_RATE` writer - Data Rate Selection"]
pub type OQPSK_DATA_RATE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, OQPSK_DATA_RATE_A>;
impl<'a, REG> OQPSK_DATA_RATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "250 kb/s (IEEE 802.15.4 compliant)"]
    #[inline(always)]
    pub fn rate_250kb(self) -> &'a mut crate::W<REG> {
        self.variant(OQPSK_DATA_RATE_A::RATE_250KB)
    }
    #[doc = "500 kb/s"]
    #[inline(always)]
    pub fn rate_500kb(self) -> &'a mut crate::W<REG> {
        self.variant(OQPSK_DATA_RATE_A::RATE_500KB)
    }
    #[doc = "1000 kb/s"]
    #[inline(always)]
    pub fn rate_1000kb(self) -> &'a mut crate::W<REG> {
        self.variant(OQPSK_DATA_RATE_A::RATE_1000KB)
    }
    #[doc = "2000 kb/s"]
    #[inline(always)]
    pub fn rate_2000kb(self) -> &'a mut crate::W<REG> {
        self.variant(OQPSK_DATA_RATE_A::RATE_2000KB)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `RX_SAFE_MODE` reader - RX Safe Mode"]
pub type RX_SAFE_MODE_R = crate::BitReader;
#[doc = "Field `RX_SAFE_MODE` writer - RX Safe Mode"]
pub type RX_SAFE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Data Rate Selection"]
    #[inline(always)]
    pub fn oqpsk_data_rate(&self) -> OQPSK_DATA_RATE_R {
        OQPSK_DATA_RATE_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:6 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 2) & 0x1f)
    }
    #[doc = "Bit 7 - RX Safe Mode"]
    #[inline(always)]
    pub fn rx_safe_mode(&self) -> RX_SAFE_MODE_R {
        RX_SAFE_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data Rate Selection"]
    #[inline(always)]
    #[must_use]
    pub fn oqpsk_data_rate(&mut self) -> OQPSK_DATA_RATE_W<TRX_CTRL_2_SPEC> {
        OQPSK_DATA_RATE_W::new(self, 0)
    }
    #[doc = "Bits 2:6 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<TRX_CTRL_2_SPEC> {
        RES_W::new(self, 2)
    }
    #[doc = "Bit 7 - RX Safe Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rx_safe_mode(&mut self) -> RX_SAFE_MODE_W<TRX_CTRL_2_SPEC> {
        RX_SAFE_MODE_W::new(self, 7)
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
#[doc = "Transceiver Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trx_ctrl_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trx_ctrl_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRX_CTRL_2_SPEC;
impl crate::RegisterSpec for TRX_CTRL_2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`trx_ctrl_2::R`](R) reader structure"]
impl crate::Readable for TRX_CTRL_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trx_ctrl_2::W`](W) writer structure"]
impl crate::Writable for TRX_CTRL_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRX_CTRL_2 to value 0"]
impl crate::Resettable for TRX_CTRL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
