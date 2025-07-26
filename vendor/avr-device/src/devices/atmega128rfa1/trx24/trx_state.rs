#[doc = "Register `TRX_STATE` reader"]
pub type R = crate::R<TRX_STATE_SPEC>;
#[doc = "Register `TRX_STATE` writer"]
pub type W = crate::W<TRX_STATE_SPEC>;
#[doc = "Field `TRX_CMD` reader - State Control Command"]
pub type TRX_CMD_R = crate::FieldReader<TRX_CMD_A>;
#[doc = "State Control Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRX_CMD_A {
    #[doc = "0: NOP"]
    CMD_NOP = 0,
    #[doc = "2: TX_START"]
    CMD_TX_START = 2,
    #[doc = "3: FORCE_TRX_OFF"]
    CMD_FORCE_TRX_OFF = 3,
    #[doc = "4: FORCE_PLL_ON"]
    CMD_FORCE_PLL_ON = 4,
    #[doc = "6: RX_ON"]
    CMD_RX_ON = 6,
    #[doc = "8: TRX_OFF"]
    CMD_TRX_OFF = 8,
    #[doc = "9: PLL_ON (TX_ON)"]
    CMD_PLL_ON = 9,
    #[doc = "22: RX_AACK_ON"]
    CMD_RX_AACK_ON = 22,
    #[doc = "25: TX_ARET_ON"]
    CMD_TX_ARET_ON = 25,
}
impl From<TRX_CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: TRX_CMD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRX_CMD_A {
    type Ux = u8;
}
impl TRX_CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRX_CMD_A> {
        match self.bits {
            0 => Some(TRX_CMD_A::CMD_NOP),
            2 => Some(TRX_CMD_A::CMD_TX_START),
            3 => Some(TRX_CMD_A::CMD_FORCE_TRX_OFF),
            4 => Some(TRX_CMD_A::CMD_FORCE_PLL_ON),
            6 => Some(TRX_CMD_A::CMD_RX_ON),
            8 => Some(TRX_CMD_A::CMD_TRX_OFF),
            9 => Some(TRX_CMD_A::CMD_PLL_ON),
            22 => Some(TRX_CMD_A::CMD_RX_AACK_ON),
            25 => Some(TRX_CMD_A::CMD_TX_ARET_ON),
            _ => None,
        }
    }
    #[doc = "NOP"]
    #[inline(always)]
    pub fn is_cmd_nop(&self) -> bool {
        *self == TRX_CMD_A::CMD_NOP
    }
    #[doc = "TX_START"]
    #[inline(always)]
    pub fn is_cmd_tx_start(&self) -> bool {
        *self == TRX_CMD_A::CMD_TX_START
    }
    #[doc = "FORCE_TRX_OFF"]
    #[inline(always)]
    pub fn is_cmd_force_trx_off(&self) -> bool {
        *self == TRX_CMD_A::CMD_FORCE_TRX_OFF
    }
    #[doc = "FORCE_PLL_ON"]
    #[inline(always)]
    pub fn is_cmd_force_pll_on(&self) -> bool {
        *self == TRX_CMD_A::CMD_FORCE_PLL_ON
    }
    #[doc = "RX_ON"]
    #[inline(always)]
    pub fn is_cmd_rx_on(&self) -> bool {
        *self == TRX_CMD_A::CMD_RX_ON
    }
    #[doc = "TRX_OFF"]
    #[inline(always)]
    pub fn is_cmd_trx_off(&self) -> bool {
        *self == TRX_CMD_A::CMD_TRX_OFF
    }
    #[doc = "PLL_ON (TX_ON)"]
    #[inline(always)]
    pub fn is_cmd_pll_on(&self) -> bool {
        *self == TRX_CMD_A::CMD_PLL_ON
    }
    #[doc = "RX_AACK_ON"]
    #[inline(always)]
    pub fn is_cmd_rx_aack_on(&self) -> bool {
        *self == TRX_CMD_A::CMD_RX_AACK_ON
    }
    #[doc = "TX_ARET_ON"]
    #[inline(always)]
    pub fn is_cmd_tx_aret_on(&self) -> bool {
        *self == TRX_CMD_A::CMD_TX_ARET_ON
    }
}
#[doc = "Field `TRX_CMD` writer - State Control Command"]
pub type TRX_CMD_W<'a, REG> = crate::FieldWriter<'a, REG, 5, TRX_CMD_A>;
impl<'a, REG> TRX_CMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NOP"]
    #[inline(always)]
    pub fn cmd_nop(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_CMD_A::CMD_NOP)
    }
    #[doc = "TX_START"]
    #[inline(always)]
    pub fn cmd_tx_start(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_CMD_A::CMD_TX_START)
    }
    #[doc = "FORCE_TRX_OFF"]
    #[inline(always)]
    pub fn cmd_force_trx_off(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_CMD_A::CMD_FORCE_TRX_OFF)
    }
    #[doc = "FORCE_PLL_ON"]
    #[inline(always)]
    pub fn cmd_force_pll_on(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_CMD_A::CMD_FORCE_PLL_ON)
    }
    #[doc = "RX_ON"]
    #[inline(always)]
    pub fn cmd_rx_on(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_CMD_A::CMD_RX_ON)
    }
    #[doc = "TRX_OFF"]
    #[inline(always)]
    pub fn cmd_trx_off(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_CMD_A::CMD_TRX_OFF)
    }
    #[doc = "PLL_ON (TX_ON)"]
    #[inline(always)]
    pub fn cmd_pll_on(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_CMD_A::CMD_PLL_ON)
    }
    #[doc = "RX_AACK_ON"]
    #[inline(always)]
    pub fn cmd_rx_aack_on(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_CMD_A::CMD_RX_AACK_ON)
    }
    #[doc = "TX_ARET_ON"]
    #[inline(always)]
    pub fn cmd_tx_aret_on(self) -> &'a mut crate::W<REG> {
        self.variant(TRX_CMD_A::CMD_TX_ARET_ON)
    }
}
#[doc = "Field `TRAC_STATUS` reader - Transaction Status"]
pub type TRAC_STATUS_R = crate::FieldReader<TRAC_STATUS_A>;
#[doc = "Transaction Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRAC_STATUS_A {
    #[doc = "0: SUCCESS (RX_AACK, TX_ARET)"]
    TRAC_SUCCESS = 0,
    #[doc = "1: SUCCESS_DATA_PENDING (TX_ARET)"]
    TRAC_SUCCESS_DATA_PENDING = 1,
    #[doc = "2: SUCCESS_WAIT_FOR_ACK (RX_AACK)"]
    TRAC_SUCCESS_WAIT_FOR_ACK = 2,
    #[doc = "3: CHANNEL_ACCESS_FAILURE (TX_ARET)"]
    TRAC_CHANNEL_ACCESS_FAILURE = 3,
    #[doc = "5: NO_ACK (TX_ARET)"]
    TRAC_NO_ACK = 5,
    #[doc = "7: INVALID (RX_AACK, TX_ARET)"]
    TRAC_INVALID = 7,
}
impl From<TRAC_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: TRAC_STATUS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRAC_STATUS_A {
    type Ux = u8;
}
impl TRAC_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRAC_STATUS_A> {
        match self.bits {
            0 => Some(TRAC_STATUS_A::TRAC_SUCCESS),
            1 => Some(TRAC_STATUS_A::TRAC_SUCCESS_DATA_PENDING),
            2 => Some(TRAC_STATUS_A::TRAC_SUCCESS_WAIT_FOR_ACK),
            3 => Some(TRAC_STATUS_A::TRAC_CHANNEL_ACCESS_FAILURE),
            5 => Some(TRAC_STATUS_A::TRAC_NO_ACK),
            7 => Some(TRAC_STATUS_A::TRAC_INVALID),
            _ => None,
        }
    }
    #[doc = "SUCCESS (RX_AACK, TX_ARET)"]
    #[inline(always)]
    pub fn is_trac_success(&self) -> bool {
        *self == TRAC_STATUS_A::TRAC_SUCCESS
    }
    #[doc = "SUCCESS_DATA_PENDING (TX_ARET)"]
    #[inline(always)]
    pub fn is_trac_success_data_pending(&self) -> bool {
        *self == TRAC_STATUS_A::TRAC_SUCCESS_DATA_PENDING
    }
    #[doc = "SUCCESS_WAIT_FOR_ACK (RX_AACK)"]
    #[inline(always)]
    pub fn is_trac_success_wait_for_ack(&self) -> bool {
        *self == TRAC_STATUS_A::TRAC_SUCCESS_WAIT_FOR_ACK
    }
    #[doc = "CHANNEL_ACCESS_FAILURE (TX_ARET)"]
    #[inline(always)]
    pub fn is_trac_channel_access_failure(&self) -> bool {
        *self == TRAC_STATUS_A::TRAC_CHANNEL_ACCESS_FAILURE
    }
    #[doc = "NO_ACK (TX_ARET)"]
    #[inline(always)]
    pub fn is_trac_no_ack(&self) -> bool {
        *self == TRAC_STATUS_A::TRAC_NO_ACK
    }
    #[doc = "INVALID (RX_AACK, TX_ARET)"]
    #[inline(always)]
    pub fn is_trac_invalid(&self) -> bool {
        *self == TRAC_STATUS_A::TRAC_INVALID
    }
}
#[doc = "Field `TRAC_STATUS` writer - Transaction Status"]
pub type TRAC_STATUS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TRAC_STATUS_A>;
impl<'a, REG> TRAC_STATUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SUCCESS (RX_AACK, TX_ARET)"]
    #[inline(always)]
    pub fn trac_success(self) -> &'a mut crate::W<REG> {
        self.variant(TRAC_STATUS_A::TRAC_SUCCESS)
    }
    #[doc = "SUCCESS_DATA_PENDING (TX_ARET)"]
    #[inline(always)]
    pub fn trac_success_data_pending(self) -> &'a mut crate::W<REG> {
        self.variant(TRAC_STATUS_A::TRAC_SUCCESS_DATA_PENDING)
    }
    #[doc = "SUCCESS_WAIT_FOR_ACK (RX_AACK)"]
    #[inline(always)]
    pub fn trac_success_wait_for_ack(self) -> &'a mut crate::W<REG> {
        self.variant(TRAC_STATUS_A::TRAC_SUCCESS_WAIT_FOR_ACK)
    }
    #[doc = "CHANNEL_ACCESS_FAILURE (TX_ARET)"]
    #[inline(always)]
    pub fn trac_channel_access_failure(self) -> &'a mut crate::W<REG> {
        self.variant(TRAC_STATUS_A::TRAC_CHANNEL_ACCESS_FAILURE)
    }
    #[doc = "NO_ACK (TX_ARET)"]
    #[inline(always)]
    pub fn trac_no_ack(self) -> &'a mut crate::W<REG> {
        self.variant(TRAC_STATUS_A::TRAC_NO_ACK)
    }
    #[doc = "INVALID (RX_AACK, TX_ARET)"]
    #[inline(always)]
    pub fn trac_invalid(self) -> &'a mut crate::W<REG> {
        self.variant(TRAC_STATUS_A::TRAC_INVALID)
    }
}
impl R {
    #[doc = "Bits 0:4 - State Control Command"]
    #[inline(always)]
    pub fn trx_cmd(&self) -> TRX_CMD_R {
        TRX_CMD_R::new(self.bits & 0x1f)
    }
    #[doc = "Bits 5:7 - Transaction Status"]
    #[inline(always)]
    pub fn trac_status(&self) -> TRAC_STATUS_R {
        TRAC_STATUS_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bits 0:4 - State Control Command"]
    #[inline(always)]
    #[must_use]
    pub fn trx_cmd(&mut self) -> TRX_CMD_W<TRX_STATE_SPEC> {
        TRX_CMD_W::new(self, 0)
    }
    #[doc = "Bits 5:7 - Transaction Status"]
    #[inline(always)]
    #[must_use]
    pub fn trac_status(&mut self) -> TRAC_STATUS_W<TRX_STATE_SPEC> {
        TRAC_STATUS_W::new(self, 5)
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
#[doc = "Transceiver State Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trx_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trx_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRX_STATE_SPEC;
impl crate::RegisterSpec for TRX_STATE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`trx_state::R`](R) reader structure"]
impl crate::Readable for TRX_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trx_state::W`](W) writer structure"]
impl crate::Writable for TRX_STATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRX_STATE to value 0"]
impl crate::Resettable for TRX_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
