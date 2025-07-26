#[doc = "Register `TST_CTRL_DIGI` reader"]
pub type R = crate::R<TST_CTRL_DIGI_SPEC>;
#[doc = "Register `TST_CTRL_DIGI` writer"]
pub type W = crate::W<TST_CTRL_DIGI_SPEC>;
#[doc = "Field `TST_CTRL_DIG` reader - Digital Test Controller Register"]
pub type TST_CTRL_DIG_R = crate::FieldReader<TST_CTRL_DIG_A>;
#[doc = "Digital Test Controller Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TST_CTRL_DIG_A {
    #[doc = "0: NORMAL (no test is active)"]
    NORMAL_NO_TEST_IS_ACTIVE = 0,
    #[doc = "15: TST_CONT_TX (continuous transmit)"]
    TST_CONT_TX_CONTINUOUS_TRANSMIT = 15,
}
impl From<TST_CTRL_DIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TST_CTRL_DIG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TST_CTRL_DIG_A {
    type Ux = u8;
}
impl TST_CTRL_DIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TST_CTRL_DIG_A> {
        match self.bits {
            0 => Some(TST_CTRL_DIG_A::NORMAL_NO_TEST_IS_ACTIVE),
            15 => Some(TST_CTRL_DIG_A::TST_CONT_TX_CONTINUOUS_TRANSMIT),
            _ => None,
        }
    }
    #[doc = "NORMAL (no test is active)"]
    #[inline(always)]
    pub fn is_normal_no_test_is_active(&self) -> bool {
        *self == TST_CTRL_DIG_A::NORMAL_NO_TEST_IS_ACTIVE
    }
    #[doc = "TST_CONT_TX (continuous transmit)"]
    #[inline(always)]
    pub fn is_tst_cont_tx_continuous_transmit(&self) -> bool {
        *self == TST_CTRL_DIG_A::TST_CONT_TX_CONTINUOUS_TRANSMIT
    }
}
#[doc = "Field `TST_CTRL_DIG` writer - Digital Test Controller Register"]
pub type TST_CTRL_DIG_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TST_CTRL_DIG_A>;
impl<'a, REG> TST_CTRL_DIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NORMAL (no test is active)"]
    #[inline(always)]
    pub fn normal_no_test_is_active(self) -> &'a mut crate::W<REG> {
        self.variant(TST_CTRL_DIG_A::NORMAL_NO_TEST_IS_ACTIVE)
    }
    #[doc = "TST_CONT_TX (continuous transmit)"]
    #[inline(always)]
    pub fn tst_cont_tx_continuous_transmit(self) -> &'a mut crate::W<REG> {
        self.variant(TST_CTRL_DIG_A::TST_CONT_TX_CONTINUOUS_TRANSMIT)
    }
}
impl R {
    #[doc = "Bits 0:3 - Digital Test Controller Register"]
    #[inline(always)]
    pub fn tst_ctrl_dig(&self) -> TST_CTRL_DIG_R {
        TST_CTRL_DIG_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Digital Test Controller Register"]
    #[inline(always)]
    #[must_use]
    pub fn tst_ctrl_dig(&mut self) -> TST_CTRL_DIG_W<TST_CTRL_DIGI_SPEC> {
        TST_CTRL_DIG_W::new(self, 0)
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
#[doc = "Transceiver Digital Test Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tst_ctrl_digi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tst_ctrl_digi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TST_CTRL_DIGI_SPEC;
impl crate::RegisterSpec for TST_CTRL_DIGI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tst_ctrl_digi::R`](R) reader structure"]
impl crate::Readable for TST_CTRL_DIGI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tst_ctrl_digi::W`](W) writer structure"]
impl crate::Writable for TST_CTRL_DIGI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TST_CTRL_DIGI to value 0"]
impl crate::Resettable for TST_CTRL_DIGI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
