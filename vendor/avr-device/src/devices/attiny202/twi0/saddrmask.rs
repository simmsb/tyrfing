#[doc = "Register `SADDRMASK` reader"]
pub type R = crate::R<SADDRMASK_SPEC>;
#[doc = "Register `SADDRMASK` writer"]
pub type W = crate::W<SADDRMASK_SPEC>;
#[doc = "Field `ADDREN` reader - Address Enable"]
pub type ADDREN_R = crate::BitReader;
#[doc = "Field `ADDREN` writer - Address Enable"]
pub type ADDREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRMASK` reader - Address Mask"]
pub type ADDRMASK_R = crate::FieldReader;
#[doc = "Field `ADDRMASK` writer - Address Mask"]
pub type ADDRMASK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Address Enable"]
    #[inline(always)]
    pub fn addren(&self) -> ADDREN_R {
        ADDREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> ADDRMASK_R {
        ADDRMASK_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bit 0 - Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn addren(&mut self) -> ADDREN_W<SADDRMASK_SPEC> {
        ADDREN_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn addrmask(&mut self) -> ADDRMASK_W<SADDRMASK_SPEC> {
        ADDRMASK_W::new(self, 1)
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
#[doc = "Slave Address Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saddrmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saddrmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SADDRMASK_SPEC;
impl crate::RegisterSpec for SADDRMASK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`saddrmask::R`](R) reader structure"]
impl crate::Readable for SADDRMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saddrmask::W`](W) writer structure"]
impl crate::Writable for SADDRMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SADDRMASK to value 0"]
impl crate::Resettable for SADDRMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
