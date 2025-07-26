#[doc = "Register `PHY_TX_PWR` reader"]
pub type R = crate::R<PHY_TX_PWR_SPEC>;
#[doc = "Register `PHY_TX_PWR` writer"]
pub type W = crate::W<PHY_TX_PWR_SPEC>;
#[doc = "Field `TX_PWR` reader - Transmit Power Setting"]
pub type TX_PWR_R = crate::FieldReader<TX_PWR_A>;
#[doc = "Transmit Power Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_PWR_A {
    #[doc = "0: 3.0 dBm"]
    _3_0_DBM = 0,
    #[doc = "1: 2.8 dBm"]
    _2_8_DBM = 1,
    #[doc = "2: 2.3 dBm"]
    _2_3_DBM = 2,
    #[doc = "3: 1.8 dBm"]
    _1_8_DBM = 3,
    #[doc = "4: 1.3 dBm"]
    _1_3_DBM = 4,
    #[doc = "5: 0.7 dBm"]
    _0_7_DBM = 5,
    #[doc = "6: 0.0 dBm"]
    _0_0_DBM = 6,
    #[doc = "7: -1 dBm"]
    _1_DBM = 7,
    #[doc = "8: -2 dBm"]
    _2_DBM = 8,
    #[doc = "9: -3 dBm"]
    _3_DBM = 9,
    #[doc = "10: -4 dBm"]
    _4_DBM = 10,
    #[doc = "11: -5 dBm"]
    _5_DBM = 11,
    #[doc = "12: -7 dBm"]
    _7_DBM = 12,
    #[doc = "13: -9 dBm"]
    _9_DBM = 13,
    #[doc = "14: -12 dBm"]
    _12_DBM = 14,
    #[doc = "15: -17 dBm"]
    _17_DBM = 15,
}
impl From<TX_PWR_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PWR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX_PWR_A {
    type Ux = u8;
}
impl TX_PWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_PWR_A {
        match self.bits {
            0 => TX_PWR_A::_3_0_DBM,
            1 => TX_PWR_A::_2_8_DBM,
            2 => TX_PWR_A::_2_3_DBM,
            3 => TX_PWR_A::_1_8_DBM,
            4 => TX_PWR_A::_1_3_DBM,
            5 => TX_PWR_A::_0_7_DBM,
            6 => TX_PWR_A::_0_0_DBM,
            7 => TX_PWR_A::_1_DBM,
            8 => TX_PWR_A::_2_DBM,
            9 => TX_PWR_A::_3_DBM,
            10 => TX_PWR_A::_4_DBM,
            11 => TX_PWR_A::_5_DBM,
            12 => TX_PWR_A::_7_DBM,
            13 => TX_PWR_A::_9_DBM,
            14 => TX_PWR_A::_12_DBM,
            15 => TX_PWR_A::_17_DBM,
            _ => unreachable!(),
        }
    }
    #[doc = "3.0 dBm"]
    #[inline(always)]
    pub fn is_3_0_dbm(&self) -> bool {
        *self == TX_PWR_A::_3_0_DBM
    }
    #[doc = "2.8 dBm"]
    #[inline(always)]
    pub fn is_2_8_dbm(&self) -> bool {
        *self == TX_PWR_A::_2_8_DBM
    }
    #[doc = "2.3 dBm"]
    #[inline(always)]
    pub fn is_2_3_dbm(&self) -> bool {
        *self == TX_PWR_A::_2_3_DBM
    }
    #[doc = "1.8 dBm"]
    #[inline(always)]
    pub fn is_1_8_dbm(&self) -> bool {
        *self == TX_PWR_A::_1_8_DBM
    }
    #[doc = "1.3 dBm"]
    #[inline(always)]
    pub fn is_1_3_dbm(&self) -> bool {
        *self == TX_PWR_A::_1_3_DBM
    }
    #[doc = "0.7 dBm"]
    #[inline(always)]
    pub fn is_0_7_dbm(&self) -> bool {
        *self == TX_PWR_A::_0_7_DBM
    }
    #[doc = "0.0 dBm"]
    #[inline(always)]
    pub fn is_0_0_dbm(&self) -> bool {
        *self == TX_PWR_A::_0_0_DBM
    }
    #[doc = "-1 dBm"]
    #[inline(always)]
    pub fn is_1_dbm(&self) -> bool {
        *self == TX_PWR_A::_1_DBM
    }
    #[doc = "-2 dBm"]
    #[inline(always)]
    pub fn is_2_dbm(&self) -> bool {
        *self == TX_PWR_A::_2_DBM
    }
    #[doc = "-3 dBm"]
    #[inline(always)]
    pub fn is_3_dbm(&self) -> bool {
        *self == TX_PWR_A::_3_DBM
    }
    #[doc = "-4 dBm"]
    #[inline(always)]
    pub fn is_4_dbm(&self) -> bool {
        *self == TX_PWR_A::_4_DBM
    }
    #[doc = "-5 dBm"]
    #[inline(always)]
    pub fn is_5_dbm(&self) -> bool {
        *self == TX_PWR_A::_5_DBM
    }
    #[doc = "-7 dBm"]
    #[inline(always)]
    pub fn is_7_dbm(&self) -> bool {
        *self == TX_PWR_A::_7_DBM
    }
    #[doc = "-9 dBm"]
    #[inline(always)]
    pub fn is_9_dbm(&self) -> bool {
        *self == TX_PWR_A::_9_DBM
    }
    #[doc = "-12 dBm"]
    #[inline(always)]
    pub fn is_12_dbm(&self) -> bool {
        *self == TX_PWR_A::_12_DBM
    }
    #[doc = "-17 dBm"]
    #[inline(always)]
    pub fn is_17_dbm(&self) -> bool {
        *self == TX_PWR_A::_17_DBM
    }
}
#[doc = "Field `TX_PWR` writer - Transmit Power Setting"]
pub type TX_PWR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, TX_PWR_A>;
impl<'a, REG> TX_PWR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3.0 dBm"]
    #[inline(always)]
    pub fn _3_0_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_3_0_DBM)
    }
    #[doc = "2.8 dBm"]
    #[inline(always)]
    pub fn _2_8_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_2_8_DBM)
    }
    #[doc = "2.3 dBm"]
    #[inline(always)]
    pub fn _2_3_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_2_3_DBM)
    }
    #[doc = "1.8 dBm"]
    #[inline(always)]
    pub fn _1_8_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_1_8_DBM)
    }
    #[doc = "1.3 dBm"]
    #[inline(always)]
    pub fn _1_3_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_1_3_DBM)
    }
    #[doc = "0.7 dBm"]
    #[inline(always)]
    pub fn _0_7_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_0_7_DBM)
    }
    #[doc = "0.0 dBm"]
    #[inline(always)]
    pub fn _0_0_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_0_0_DBM)
    }
    #[doc = "-1 dBm"]
    #[inline(always)]
    pub fn _1_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_1_DBM)
    }
    #[doc = "-2 dBm"]
    #[inline(always)]
    pub fn _2_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_2_DBM)
    }
    #[doc = "-3 dBm"]
    #[inline(always)]
    pub fn _3_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_3_DBM)
    }
    #[doc = "-4 dBm"]
    #[inline(always)]
    pub fn _4_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_4_DBM)
    }
    #[doc = "-5 dBm"]
    #[inline(always)]
    pub fn _5_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_5_DBM)
    }
    #[doc = "-7 dBm"]
    #[inline(always)]
    pub fn _7_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_7_DBM)
    }
    #[doc = "-9 dBm"]
    #[inline(always)]
    pub fn _9_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_9_DBM)
    }
    #[doc = "-12 dBm"]
    #[inline(always)]
    pub fn _12_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_12_DBM)
    }
    #[doc = "-17 dBm"]
    #[inline(always)]
    pub fn _17_dbm(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PWR_A::_17_DBM)
    }
}
#[doc = "Field `PA_LT` reader - Power Amplifier Lead Time"]
pub type PA_LT_R = crate::FieldReader<PA_LT_A>;
#[doc = "Power Amplifier Lead Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PA_LT_A {
    #[doc = "0: 2 us"]
    PA_LT_2US = 0,
    #[doc = "1: 4 us"]
    PA_LT_4US = 1,
    #[doc = "2: 6 us"]
    PA_LT_6US = 2,
    #[doc = "3: 8 us"]
    PA_LT_8US = 3,
}
impl From<PA_LT_A> for u8 {
    #[inline(always)]
    fn from(variant: PA_LT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PA_LT_A {
    type Ux = u8;
}
impl PA_LT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PA_LT_A {
        match self.bits {
            0 => PA_LT_A::PA_LT_2US,
            1 => PA_LT_A::PA_LT_4US,
            2 => PA_LT_A::PA_LT_6US,
            3 => PA_LT_A::PA_LT_8US,
            _ => unreachable!(),
        }
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn is_pa_lt_2us(&self) -> bool {
        *self == PA_LT_A::PA_LT_2US
    }
    #[doc = "4 us"]
    #[inline(always)]
    pub fn is_pa_lt_4us(&self) -> bool {
        *self == PA_LT_A::PA_LT_4US
    }
    #[doc = "6 us"]
    #[inline(always)]
    pub fn is_pa_lt_6us(&self) -> bool {
        *self == PA_LT_A::PA_LT_6US
    }
    #[doc = "8 us"]
    #[inline(always)]
    pub fn is_pa_lt_8us(&self) -> bool {
        *self == PA_LT_A::PA_LT_8US
    }
}
#[doc = "Field `PA_LT` writer - Power Amplifier Lead Time"]
pub type PA_LT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PA_LT_A>;
impl<'a, REG> PA_LT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2 us"]
    #[inline(always)]
    pub fn pa_lt_2us(self) -> &'a mut crate::W<REG> {
        self.variant(PA_LT_A::PA_LT_2US)
    }
    #[doc = "4 us"]
    #[inline(always)]
    pub fn pa_lt_4us(self) -> &'a mut crate::W<REG> {
        self.variant(PA_LT_A::PA_LT_4US)
    }
    #[doc = "6 us"]
    #[inline(always)]
    pub fn pa_lt_6us(self) -> &'a mut crate::W<REG> {
        self.variant(PA_LT_A::PA_LT_6US)
    }
    #[doc = "8 us"]
    #[inline(always)]
    pub fn pa_lt_8us(self) -> &'a mut crate::W<REG> {
        self.variant(PA_LT_A::PA_LT_8US)
    }
}
#[doc = "Field `PA_BUF_LT` reader - Power Amplifier Buffer Lead Time"]
pub type PA_BUF_LT_R = crate::FieldReader<PA_BUF_LT_A>;
#[doc = "Power Amplifier Buffer Lead Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PA_BUF_LT_A {
    #[doc = "0: 0 us"]
    PA_BUF_LT_0US = 0,
    #[doc = "1: 2 us"]
    PA_BUF_LT_2US = 1,
    #[doc = "2: 4 us"]
    PA_BUF_LT_4US = 2,
    #[doc = "3: 6 us"]
    PA_BUF_LT_6US = 3,
}
impl From<PA_BUF_LT_A> for u8 {
    #[inline(always)]
    fn from(variant: PA_BUF_LT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PA_BUF_LT_A {
    type Ux = u8;
}
impl PA_BUF_LT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PA_BUF_LT_A {
        match self.bits {
            0 => PA_BUF_LT_A::PA_BUF_LT_0US,
            1 => PA_BUF_LT_A::PA_BUF_LT_2US,
            2 => PA_BUF_LT_A::PA_BUF_LT_4US,
            3 => PA_BUF_LT_A::PA_BUF_LT_6US,
            _ => unreachable!(),
        }
    }
    #[doc = "0 us"]
    #[inline(always)]
    pub fn is_pa_buf_lt_0us(&self) -> bool {
        *self == PA_BUF_LT_A::PA_BUF_LT_0US
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn is_pa_buf_lt_2us(&self) -> bool {
        *self == PA_BUF_LT_A::PA_BUF_LT_2US
    }
    #[doc = "4 us"]
    #[inline(always)]
    pub fn is_pa_buf_lt_4us(&self) -> bool {
        *self == PA_BUF_LT_A::PA_BUF_LT_4US
    }
    #[doc = "6 us"]
    #[inline(always)]
    pub fn is_pa_buf_lt_6us(&self) -> bool {
        *self == PA_BUF_LT_A::PA_BUF_LT_6US
    }
}
#[doc = "Field `PA_BUF_LT` writer - Power Amplifier Buffer Lead Time"]
pub type PA_BUF_LT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PA_BUF_LT_A>;
impl<'a, REG> PA_BUF_LT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 us"]
    #[inline(always)]
    pub fn pa_buf_lt_0us(self) -> &'a mut crate::W<REG> {
        self.variant(PA_BUF_LT_A::PA_BUF_LT_0US)
    }
    #[doc = "2 us"]
    #[inline(always)]
    pub fn pa_buf_lt_2us(self) -> &'a mut crate::W<REG> {
        self.variant(PA_BUF_LT_A::PA_BUF_LT_2US)
    }
    #[doc = "4 us"]
    #[inline(always)]
    pub fn pa_buf_lt_4us(self) -> &'a mut crate::W<REG> {
        self.variant(PA_BUF_LT_A::PA_BUF_LT_4US)
    }
    #[doc = "6 us"]
    #[inline(always)]
    pub fn pa_buf_lt_6us(self) -> &'a mut crate::W<REG> {
        self.variant(PA_BUF_LT_A::PA_BUF_LT_6US)
    }
}
impl R {
    #[doc = "Bits 0:3 - Transmit Power Setting"]
    #[inline(always)]
    pub fn tx_pwr(&self) -> TX_PWR_R {
        TX_PWR_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - Power Amplifier Lead Time"]
    #[inline(always)]
    pub fn pa_lt(&self) -> PA_LT_R {
        PA_LT_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Power Amplifier Buffer Lead Time"]
    #[inline(always)]
    pub fn pa_buf_lt(&self) -> PA_BUF_LT_R {
        PA_BUF_LT_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmit Power Setting"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pwr(&mut self) -> TX_PWR_W<PHY_TX_PWR_SPEC> {
        TX_PWR_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Power Amplifier Lead Time"]
    #[inline(always)]
    #[must_use]
    pub fn pa_lt(&mut self) -> PA_LT_W<PHY_TX_PWR_SPEC> {
        PA_LT_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Power Amplifier Buffer Lead Time"]
    #[inline(always)]
    #[must_use]
    pub fn pa_buf_lt(&mut self) -> PA_BUF_LT_W<PHY_TX_PWR_SPEC> {
        PA_BUF_LT_W::new(self, 6)
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
#[doc = "Transceiver Transmit Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tx_pwr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tx_pwr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_TX_PWR_SPEC;
impl crate::RegisterSpec for PHY_TX_PWR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_tx_pwr::R`](R) reader structure"]
impl crate::Readable for PHY_TX_PWR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_tx_pwr::W`](W) writer structure"]
impl crate::Writable for PHY_TX_PWR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHY_TX_PWR to value 0"]
impl crate::Resettable for PHY_TX_PWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
