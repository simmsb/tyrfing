#[doc = "Register `RXPLCTRL` reader"]
pub type R = crate::R<RXPLCTRL_SPEC>;
#[doc = "Register `RXPLCTRL` writer"]
pub type W = crate::W<RXPLCTRL_SPEC>;
#[doc = "Field `RXPL` reader - Receiver Pulse Lenght"]
pub type RXPL_R = crate::FieldReader;
#[doc = "Field `RXPL` writer - Receiver Pulse Lenght"]
pub type RXPL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Receiver Pulse Lenght"]
    #[inline(always)]
    pub fn rxpl(&self) -> RXPL_R {
        RXPL_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Receiver Pulse Lenght"]
    #[inline(always)]
    #[must_use]
    pub fn rxpl(&mut self) -> RXPL_W<RXPLCTRL_SPEC> {
        RXPL_W::new(self, 0)
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
#[doc = "IRCOM Receiver Pulse Length Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxplctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxplctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXPLCTRL_SPEC;
impl crate::RegisterSpec for RXPLCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxplctrl::R`](R) reader structure"]
impl crate::Readable for RXPLCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxplctrl::W`](W) writer structure"]
impl crate::Writable for RXPLCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXPLCTRL to value 0"]
impl crate::Resettable for RXPLCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
