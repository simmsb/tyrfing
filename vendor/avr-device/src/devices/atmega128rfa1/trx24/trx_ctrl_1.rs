#[doc = "Register `TRX_CTRL_1` reader"]
pub type R = crate::R<TRX_CTRL_1_SPEC>;
#[doc = "Register `TRX_CTRL_1` writer"]
pub type W = crate::W<TRX_CTRL_1_SPEC>;
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `TX_AUTO_CRC_ON` reader - Enable Automatic CRC Calculation"]
pub type TX_AUTO_CRC_ON_R = crate::BitReader;
#[doc = "Field `TX_AUTO_CRC_ON` writer - Enable Automatic CRC Calculation"]
pub type TX_AUTO_CRC_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_2_EXT_EN` reader - Connect Frame Start IRQ to TC1"]
pub type IRQ_2_EXT_EN_R = crate::BitReader;
#[doc = "Field `IRQ_2_EXT_EN` writer - Connect Frame Start IRQ to TC1"]
pub type IRQ_2_EXT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA_EXT_EN` reader - External PA support enable"]
pub type PA_EXT_EN_R = crate::BitReader;
#[doc = "Field `PA_EXT_EN` writer - External PA support enable"]
pub type PA_EXT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Enable Automatic CRC Calculation"]
    #[inline(always)]
    pub fn tx_auto_crc_on(&self) -> TX_AUTO_CRC_ON_R {
        TX_AUTO_CRC_ON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Connect Frame Start IRQ to TC1"]
    #[inline(always)]
    pub fn irq_2_ext_en(&self) -> IRQ_2_EXT_EN_R {
        IRQ_2_EXT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External PA support enable"]
    #[inline(always)]
    pub fn pa_ext_en(&self) -> PA_EXT_EN_R {
        PA_EXT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<TRX_CTRL_1_SPEC> {
        RES_W::new(self, 0)
    }
    #[doc = "Bit 5 - Enable Automatic CRC Calculation"]
    #[inline(always)]
    #[must_use]
    pub fn tx_auto_crc_on(&mut self) -> TX_AUTO_CRC_ON_W<TRX_CTRL_1_SPEC> {
        TX_AUTO_CRC_ON_W::new(self, 5)
    }
    #[doc = "Bit 6 - Connect Frame Start IRQ to TC1"]
    #[inline(always)]
    #[must_use]
    pub fn irq_2_ext_en(&mut self) -> IRQ_2_EXT_EN_W<TRX_CTRL_1_SPEC> {
        IRQ_2_EXT_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - External PA support enable"]
    #[inline(always)]
    #[must_use]
    pub fn pa_ext_en(&mut self) -> PA_EXT_EN_W<TRX_CTRL_1_SPEC> {
        PA_EXT_EN_W::new(self, 7)
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
#[doc = "Transceiver Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trx_ctrl_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trx_ctrl_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRX_CTRL_1_SPEC;
impl crate::RegisterSpec for TRX_CTRL_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`trx_ctrl_1::R`](R) reader structure"]
impl crate::Readable for TRX_CTRL_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trx_ctrl_1::W`](W) writer structure"]
impl crate::Writable for TRX_CTRL_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRX_CTRL_1 to value 0"]
impl crate::Resettable for TRX_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
