#[doc = "Register `SHORT_ADDR_1` reader"]
pub type R = crate::R<SHORT_ADDR_1_SPEC>;
#[doc = "Register `SHORT_ADDR_1` writer"]
pub type W = crate::W<SHORT_ADDR_1_SPEC>;
#[doc = "Field `SHORT_ADDR_` reader - MAC Short Address"]
pub type SHORT_ADDR__R = crate::FieldReader;
#[doc = "Field `SHORT_ADDR_` writer - MAC Short Address"]
pub type SHORT_ADDR__W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MAC Short Address"]
    #[inline(always)]
    pub fn short_addr_(&self) -> SHORT_ADDR__R {
        SHORT_ADDR__R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - MAC Short Address"]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_(&mut self) -> SHORT_ADDR__W<SHORT_ADDR_1_SPEC> {
        SHORT_ADDR__W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transceiver MAC Short Address Register (High Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`short_addr_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`short_addr_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHORT_ADDR_1_SPEC;
impl crate::RegisterSpec for SHORT_ADDR_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`short_addr_1::R`](R) reader structure"]
impl crate::Readable for SHORT_ADDR_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`short_addr_1::W`](W) writer structure"]
impl crate::Writable for SHORT_ADDR_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHORT_ADDR_1 to value 0"]
impl crate::Resettable for SHORT_ADDR_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
