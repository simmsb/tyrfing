#[doc = "Register `TWAR` reader"]
pub type R = crate::R<TWAR_SPEC>;
#[doc = "Register `TWAR` writer"]
pub type W = crate::W<TWAR_SPEC>;
#[doc = "Field `TWGCE` reader - TWI General Call Recognition Enable Bit"]
pub type TWGCE_R = crate::BitReader;
#[doc = "Field `TWGCE` writer - TWI General Call Recognition Enable Bit"]
pub type TWGCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWA` reader - TWI (Slave) Address register Bits"]
pub type TWA_R = crate::FieldReader;
#[doc = "Field `TWA` writer - TWI (Slave) Address register Bits"]
pub type TWA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - TWI General Call Recognition Enable Bit"]
    #[inline(always)]
    pub fn twgce(&self) -> TWGCE_R {
        TWGCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - TWI (Slave) Address register Bits"]
    #[inline(always)]
    pub fn twa(&self) -> TWA_R {
        TWA_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - TWI General Call Recognition Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn twgce(&mut self) -> TWGCE_W<TWAR_SPEC> {
        TWGCE_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - TWI (Slave) Address register Bits"]
    #[inline(always)]
    #[must_use]
    pub fn twa(&mut self) -> TWA_W<TWAR_SPEC> {
        TWA_W::new(self, 1)
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
#[doc = "TWI (Slave) Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWAR_SPEC;
impl crate::RegisterSpec for TWAR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`twar::R`](R) reader structure"]
impl crate::Readable for TWAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twar::W`](W) writer structure"]
impl crate::Writable for TWAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWAR to value 0"]
impl crate::Resettable for TWAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
