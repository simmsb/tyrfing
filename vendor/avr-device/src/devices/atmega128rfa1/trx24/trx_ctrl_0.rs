#[doc = "Register `TRX_CTRL_0` reader"]
pub type R = crate::R<TRX_CTRL_0_SPEC>;
#[doc = "Register `TRX_CTRL_0` writer"]
pub type W = crate::W<TRX_CTRL_0_SPEC>;
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<TRX_CTRL_0_SPEC> {
        RES_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trx_ctrl_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trx_ctrl_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRX_CTRL_0_SPEC;
impl crate::RegisterSpec for TRX_CTRL_0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`trx_ctrl_0::R`](R) reader structure"]
impl crate::Readable for TRX_CTRL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trx_ctrl_0::W`](W) writer structure"]
impl crate::Writable for TRX_CTRL_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRX_CTRL_0 to value 0"]
impl crate::Resettable for TRX_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
