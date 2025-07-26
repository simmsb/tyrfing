#[doc = "Register `SCSR` reader"]
pub type R = crate::R<SCSR_SPEC>;
#[doc = "Register `SCSR` writer"]
pub type W = crate::W<SCSR_SPEC>;
#[doc = "Field `SCBSY` reader - Symbol Counter busy"]
pub type SCBSY_R = crate::BitReader;
#[doc = "Field `SCBSY` writer - Symbol Counter busy"]
pub type SCBSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Res` reader - Reserved Bit"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved Bit"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Symbol Counter busy"]
    #[inline(always)]
    pub fn scbsy(&self) -> SCBSY_R {
        SCBSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved Bit"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - Symbol Counter busy"]
    #[inline(always)]
    #[must_use]
    pub fn scbsy(&mut self) -> SCBSY_W<SCSR_SPEC> {
        SCBSY_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Reserved Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<SCSR_SPEC> {
        RES_W::new(self, 1)
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
#[doc = "Symbol Counter Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCSR_SPEC;
impl crate::RegisterSpec for SCSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scsr::R`](R) reader structure"]
impl crate::Readable for SCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scsr::W`](W) writer structure"]
impl crate::Writable for SCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for SCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
