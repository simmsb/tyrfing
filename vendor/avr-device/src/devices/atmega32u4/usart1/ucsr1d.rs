#[doc = "Register `UCSR1D` reader"]
pub type R = crate::R<UCSR1D_SPEC>;
#[doc = "Register `UCSR1D` writer"]
pub type W = crate::W<UCSR1D_SPEC>;
#[doc = "Field `RTSEN` reader - RTS Enable"]
pub type RTSEN_R = crate::BitReader;
#[doc = "Field `RTSEN` writer - RTS Enable"]
pub type RTSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - CTS Enable"]
pub type CTSEN_R = crate::BitReader;
#[doc = "Field `CTSEN` writer - CTS Enable"]
pub type CTSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTS Enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTS Enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RTSEN_W<UCSR1D_SPEC> {
        RTSEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CTS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CTSEN_W<UCSR1D_SPEC> {
        CTSEN_W::new(self, 1)
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
#[doc = "USART Control and Status Register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucsr1d::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucsr1d::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCSR1D_SPEC;
impl crate::RegisterSpec for UCSR1D_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucsr1d::R`](R) reader structure"]
impl crate::Readable for UCSR1D_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ucsr1d::W`](W) writer structure"]
impl crate::Writable for UCSR1D_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCSR1D to value 0"]
impl crate::Resettable for UCSR1D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
