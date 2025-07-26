#[doc = "Register `IEEE_ADDR_1` reader"]
pub type R = crate::R<IEEE_ADDR_1_SPEC>;
#[doc = "Register `IEEE_ADDR_1` writer"]
pub type W = crate::W<IEEE_ADDR_1_SPEC>;
#[doc = "Field `IEEE_ADDR_` reader - MAC IEEE Address"]
pub type IEEE_ADDR__R = crate::FieldReader;
#[doc = "Field `IEEE_ADDR_` writer - MAC IEEE Address"]
pub type IEEE_ADDR__W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MAC IEEE Address"]
    #[inline(always)]
    pub fn ieee_addr_(&self) -> IEEE_ADDR__R {
        IEEE_ADDR__R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - MAC IEEE Address"]
    #[inline(always)]
    #[must_use]
    pub fn ieee_addr_(&mut self) -> IEEE_ADDR__W<IEEE_ADDR_1_SPEC> {
        IEEE_ADDR__W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transceiver MAC IEEE Address Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ieee_addr_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ieee_addr_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEEE_ADDR_1_SPEC;
impl crate::RegisterSpec for IEEE_ADDR_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ieee_addr_1::R`](R) reader structure"]
impl crate::Readable for IEEE_ADDR_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ieee_addr_1::W`](W) writer structure"]
impl crate::Writable for IEEE_ADDR_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEEE_ADDR_1 to value 0"]
impl crate::Resettable for IEEE_ADDR_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
