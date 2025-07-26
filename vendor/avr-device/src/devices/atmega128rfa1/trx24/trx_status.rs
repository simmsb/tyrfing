#[doc = "Register `TRX_STATUS` reader"]
pub type R = crate::R<TRX_STATUS_SPEC>;
#[doc = "Register `TRX_STATUS` writer"]
pub type W = crate::W<TRX_STATUS_SPEC>;
#[doc = "Field `TRX_STATUS` reader - Transceiver Main Status"]
pub type TRX_STATUS_R = crate::FieldReader<TRX_STATUS_A>;
#[doc = "Transceiver Main Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRX_STATUS_A {
    #[doc = "0: P_ON"]
    P_ON = 0,
    #[doc = "1: BUSY_RX"]
    BUSY_RX = 1,
    #[doc = "2: BUSY_TX"]
    BUSY_TX = 2,
    #[doc = "6: RX_ON"]
    RX_ON = 6,
    #[doc = "8: TRX_OFF"]
    TRX_OFF = 8,
    #[doc = "9: PLL_ON"]
    PLL_ON = 9,
    #[doc = "15: SLEEP"]
    SLEEP = 15,
    #[doc = "17: BUSY_RX_AACK"]
    BUSY_RX_AACK = 17,
    #[doc = "18: BUSY_TX_ARET"]
    BUSY_TX_ARET = 18,
    #[doc = "22: RX_AACK_ON"]
    RX_AACK_ON = 22,
    #[doc = "25: TX_ARET_ON"]
    TX_ARET_ON = 25,
    #[doc = "31: STATE_TRANSITION_IN_PROGRESS"]
    STATE_TRANSITION_IN_PROGRESS = 31,
}
impl From<TRX_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: TRX_STATUS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRX_STATUS_A {
    type Ux = u8;
}
impl TRX_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRX_STATUS_A> {
        match self.bits {
            0 => Some(TRX_STATUS_A::P_ON),
            1 => Some(TRX_STATUS_A::BUSY_RX),
            2 => Some(TRX_STATUS_A::BUSY_TX),
            6 => Some(TRX_STATUS_A::RX_ON),
            8 => Some(TRX_STATUS_A::TRX_OFF),
            9 => Some(TRX_STATUS_A::PLL_ON),
            15 => Some(TRX_STATUS_A::SLEEP),
            17 => Some(TRX_STATUS_A::BUSY_RX_AACK),
            18 => Some(TRX_STATUS_A::BUSY_TX_ARET),
            22 => Some(TRX_STATUS_A::RX_AACK_ON),
            25 => Some(TRX_STATUS_A::TX_ARET_ON),
            31 => Some(TRX_STATUS_A::STATE_TRANSITION_IN_PROGRESS),
            _ => None,
        }
    }
    #[doc = "P_ON"]
    #[inline(always)]
    pub fn is_p_on(&self) -> bool {
        *self == TRX_STATUS_A::P_ON
    }
    #[doc = "BUSY_RX"]
    #[inline(always)]
    pub fn is_busy_rx(&self) -> bool {
        *self == TRX_STATUS_A::BUSY_RX
    }
    #[doc = "BUSY_TX"]
    #[inline(always)]
    pub fn is_busy_tx(&self) -> bool {
        *self == TRX_STATUS_A::BUSY_TX
    }
    #[doc = "RX_ON"]
    #[inline(always)]
    pub fn is_rx_on(&self) -> bool {
        *self == TRX_STATUS_A::RX_ON
    }
    #[doc = "TRX_OFF"]
    #[inline(always)]
    pub fn is_trx_off(&self) -> bool {
        *self == TRX_STATUS_A::TRX_OFF
    }
    #[doc = "PLL_ON"]
    #[inline(always)]
    pub fn is_pll_on(&self) -> bool {
        *self == TRX_STATUS_A::PLL_ON
    }
    #[doc = "SLEEP"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == TRX_STATUS_A::SLEEP
    }
    #[doc = "BUSY_RX_AACK"]
    #[inline(always)]
    pub fn is_busy_rx_aack(&self) -> bool {
        *self == TRX_STATUS_A::BUSY_RX_AACK
    }
    #[doc = "BUSY_TX_ARET"]
    #[inline(always)]
    pub fn is_busy_tx_aret(&self) -> bool {
        *self == TRX_STATUS_A::BUSY_TX_ARET
    }
    #[doc = "RX_AACK_ON"]
    #[inline(always)]
    pub fn is_rx_aack_on(&self) -> bool {
        *self == TRX_STATUS_A::RX_AACK_ON
    }
    #[doc = "TX_ARET_ON"]
    #[inline(always)]
    pub fn is_tx_aret_on(&self) -> bool {
        *self == TRX_STATUS_A::TX_ARET_ON
    }
    #[doc = "STATE_TRANSITION_IN_PROGRESS"]
    #[inline(always)]
    pub fn is_state_transition_in_progress(&self) -> bool {
        *self == TRX_STATUS_A::STATE_TRANSITION_IN_PROGRESS
    }
}
#[doc = "Field `TRX_STATUS` writer - Transceiver Main Status"]
pub type TRX_STATUS_W<'a, REG> = crate::FieldWriter<'a, REG, 5, TRX_STATUS_A>;
impl<'a, REG> TRX_STATUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "P_ON"]
    #[inline(always)]
    pub fn p_on(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_STATUS_A::P_ON)
    }
    #[doc = "BUSY_RX"]
    #[inline(always)]
    pub fn busy_rx(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_STATUS_A::BUSY_RX)
    }
    #[doc = "BUSY_TX"]
    #[inline(always)]
    pub fn busy_tx(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_STATUS_A::BUSY_TX)
    }
    #[doc = "RX_ON"]
    #[inline(always)]
    pub fn rx_on(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_STATUS_A::RX_ON)
    }
    #[doc = "TRX_OFF"]
    #[inline(always)]
    pub fn trx_off(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_STATUS_A::TRX_OFF)
    }
    #[doc = "PLL_ON"]
    #[inline(always)]
    pub fn pll_on(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_STATUS_A::PLL_ON)
    }
    #[doc = "SLEEP"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_STATUS_A::SLEEP)
    }
    #[doc = "BUSY_RX_AACK"]
    #[inline(always)]
    pub fn busy_rx_aack(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_STATUS_A::BUSY_RX_AACK)
    }
    #[doc = "BUSY_TX_ARET"]
    #[inline(always)]
    pub fn busy_tx_aret(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_STATUS_A::BUSY_TX_ARET)
    }
    #[doc = "RX_AACK_ON"]
    #[inline(always)]
    pub fn rx_aack_on(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_STATUS_A::RX_AACK_ON)
    }
    #[doc = "TX_ARET_ON"]
    #[inline(always)]
    pub fn tx_aret_on(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_STATUS_A::TX_ARET_ON)
    }
    #[doc = "STATE_TRANSITION_IN_PROGRESS"]
    #[inline(always)]
    pub fn state_transition_in_progress(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_STATUS_A::STATE_TRANSITION_IN_PROGRESS)
    }
}
#[doc = "Field `TST_STATUS` reader - Test mode status"]
pub type TST_STATUS_R = crate::BitReader<TST_STATUS_A>;
#[doc = "Test mode status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TST_STATUS_A {
    #[doc = "0: Test mode is disabled."]
    TST_DISABLED = 0,
    #[doc = "1: Test mode is active."]
    TST_ENABLED = 1,
}
impl From<TST_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: TST_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl TST_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TST_STATUS_A {
        match self.bits {
            false => TST_STATUS_A::TST_DISABLED,
            true => TST_STATUS_A::TST_ENABLED,
        }
    }
    #[doc = "Test mode is disabled."]
    #[inline(always)]
    pub fn is_tst_disabled(&self) -> bool {
        *self == TST_STATUS_A::TST_DISABLED
    }
    #[doc = "Test mode is active."]
    #[inline(always)]
    pub fn is_tst_enabled(&self) -> bool {
        *self == TST_STATUS_A::TST_ENABLED
    }
}
#[doc = "Field `TST_STATUS` writer - Test mode status"]
pub type TST_STATUS_W<'a, REG> = crate::BitWriter<'a, REG, TST_STATUS_A>;
impl<'a, REG> TST_STATUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Test mode is disabled."]
    #[inline(always)]
    pub fn tst_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TST_STATUS_A::TST_DISABLED)
    }
    #[doc = "Test mode is active."]
    #[inline(always)]
    pub fn tst_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TST_STATUS_A::TST_ENABLED)
    }
}
#[doc = "Field `CCA_STATUS` reader - CCA Status Result"]
pub type CCA_STATUS_R = crate::BitReader<CCA_STATUS_A>;
#[doc = "CCA Status Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCA_STATUS_A {
    #[doc = "0: Channel indicated as busy."]
    CCA_BUSY = 0,
    #[doc = "1: Channel indicated as idle."]
    CCA_IDLE = 1,
}
impl From<CCA_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: CCA_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCA_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCA_STATUS_A {
        match self.bits {
            false => CCA_STATUS_A::CCA_BUSY,
            true => CCA_STATUS_A::CCA_IDLE,
        }
    }
    #[doc = "Channel indicated as busy."]
    #[inline(always)]
    pub fn is_cca_busy(&self) -> bool {
        *self == CCA_STATUS_A::CCA_BUSY
    }
    #[doc = "Channel indicated as idle."]
    #[inline(always)]
    pub fn is_cca_idle(&self) -> bool {
        *self == CCA_STATUS_A::CCA_IDLE
    }
}
#[doc = "Field `CCA_STATUS` writer - CCA Status Result"]
pub type CCA_STATUS_W<'a, REG> = crate::BitWriter<'a, REG, CCA_STATUS_A>;
impl<'a, REG> CCA_STATUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel indicated as busy."]
    #[inline(always)]
    pub fn cca_busy(self) -> &'a mut crate::W<REG> {
        self.variant(CCA_STATUS_A::CCA_BUSY)
    }
    #[doc = "Channel indicated as idle."]
    #[inline(always)]
    pub fn cca_idle(self) -> &'a mut crate::W<REG> {
        self.variant(CCA_STATUS_A::CCA_IDLE)
    }
}
#[doc = "Field `CCA_DONE` reader - CCA Algorithm Status"]
pub type CCA_DONE_R = crate::BitReader<CCA_DONE_A>;
#[doc = "CCA Algorithm Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCA_DONE_A {
    #[doc = "0: CCA calculation not finished"]
    CCA_NOT_FIN = 0,
    #[doc = "1: CCA calculation finished"]
    CCA_FIN = 1,
}
impl From<CCA_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: CCA_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCA_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCA_DONE_A {
        match self.bits {
            false => CCA_DONE_A::CCA_NOT_FIN,
            true => CCA_DONE_A::CCA_FIN,
        }
    }
    #[doc = "CCA calculation not finished"]
    #[inline(always)]
    pub fn is_cca_not_fin(&self) -> bool {
        *self == CCA_DONE_A::CCA_NOT_FIN
    }
    #[doc = "CCA calculation finished"]
    #[inline(always)]
    pub fn is_cca_fin(&self) -> bool {
        *self == CCA_DONE_A::CCA_FIN
    }
}
#[doc = "Field `CCA_DONE` writer - CCA Algorithm Status"]
pub type CCA_DONE_W<'a, REG> = crate::BitWriter<'a, REG, CCA_DONE_A>;
impl<'a, REG> CCA_DONE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCA calculation not finished"]
    #[inline(always)]
    pub fn cca_not_fin(self) -> &'a mut crate::W<REG> {
        self.variant(CCA_DONE_A::CCA_NOT_FIN)
    }
    #[doc = "CCA calculation finished"]
    #[inline(always)]
    pub fn cca_fin(self) -> &'a mut crate::W<REG> {
        self.variant(CCA_DONE_A::CCA_FIN)
    }
}
impl R {
    #[doc = "Bits 0:4 - Transceiver Main Status"]
    #[inline(always)]
    pub fn trx_status(&self) -> TRX_STATUS_R {
        TRX_STATUS_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Test mode status"]
    #[inline(always)]
    pub fn tst_status(&self) -> TST_STATUS_R {
        TST_STATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CCA Status Result"]
    #[inline(always)]
    pub fn cca_status(&self) -> CCA_STATUS_R {
        CCA_STATUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CCA Algorithm Status"]
    #[inline(always)]
    pub fn cca_done(&self) -> CCA_DONE_R {
        CCA_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transceiver Main Status"]
    #[inline(always)]
    #[must_use]
    pub fn trx_status(&mut self) -> TRX_STATUS_W<TRX_STATUS_SPEC> {
        TRX_STATUS_W::new(self, 0)
    }
    #[doc = "Bit 5 - Test mode status"]
    #[inline(always)]
    #[must_use]
    pub fn tst_status(&mut self) -> TST_STATUS_W<TRX_STATUS_SPEC> {
        TST_STATUS_W::new(self, 5)
    }
    #[doc = "Bit 6 - CCA Status Result"]
    #[inline(always)]
    #[must_use]
    pub fn cca_status(&mut self) -> CCA_STATUS_W<TRX_STATUS_SPEC> {
        CCA_STATUS_W::new(self, 6)
    }
    #[doc = "Bit 7 - CCA Algorithm Status"]
    #[inline(always)]
    #[must_use]
    pub fn cca_done(&mut self) -> CCA_DONE_W<TRX_STATUS_SPEC> {
        CCA_DONE_W::new(self, 7)
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
#[doc = "Transceiver Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trx_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trx_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRX_STATUS_SPEC;
impl crate::RegisterSpec for TRX_STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`trx_status::R`](R) reader structure"]
impl crate::Readable for TRX_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trx_status::W`](W) writer structure"]
impl crate::Writable for TRX_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRX_STATUS to value 0"]
impl crate::Resettable for TRX_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
