#[doc = "Register `XDIV` reader"]
pub type R = crate::R<XDIV_SPEC>;
#[doc = "Register `XDIV` writer"]
pub type W = crate::W<XDIV_SPEC>;
#[doc = "Field `XDIV` reader - XTAl Divide Select Bits"]
pub type XDIV_R = crate::FieldReader;
#[doc = "Field `XDIV` writer - XTAl Divide Select Bits"]
pub type XDIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `XDIVEN` reader - XTAL Divide Enable"]
pub type XDIVEN_R = crate::BitReader;
#[doc = "Field `XDIVEN` writer - XTAL Divide Enable"]
pub type XDIVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - XTAl Divide Select Bits"]
    #[inline(always)]
    pub fn xdiv(&self) -> XDIV_R {
        XDIV_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - XTAL Divide Enable"]
    #[inline(always)]
    pub fn xdiven(&self) -> XDIVEN_R {
        XDIVEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - XTAl Divide Select Bits"]
    #[inline(always)]
    #[must_use]
    pub fn xdiv(&mut self) -> XDIV_W<XDIV_SPEC> {
        XDIV_W::new(self, 0)
    }
    #[doc = "Bit 7 - XTAL Divide Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xdiven(&mut self) -> XDIVEN_W<XDIV_SPEC> {
        XDIVEN_W::new(self, 7)
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
#[doc = "XTAL Divide Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XDIV_SPEC;
impl crate::RegisterSpec for XDIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xdiv::R`](R) reader structure"]
impl crate::Readable for XDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xdiv::W`](W) writer structure"]
impl crate::Writable for XDIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XDIV to value 0"]
impl crate::Resettable for XDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
