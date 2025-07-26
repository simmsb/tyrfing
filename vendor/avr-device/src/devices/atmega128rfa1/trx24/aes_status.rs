#[doc = "Register `AES_STATUS` reader"]
pub type R = crate::R<AES_STATUS_SPEC>;
#[doc = "Register `AES_STATUS` writer"]
pub type W = crate::W<AES_STATUS_SPEC>;
#[doc = "Field `AES_DONE` reader - AES Operation Finished with Success"]
pub type AES_DONE_R = crate::BitReader;
#[doc = "Field `AES_DONE` writer - AES Operation Finished with Success"]
pub type AES_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "Field `AES_ER` reader - AES Operation Finished with Error"]
pub type AES_ER_R = crate::BitReader;
#[doc = "Field `AES_ER` writer - AES Operation Finished with Error"]
pub type AES_ER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES Operation Finished with Success"]
    #[inline(always)]
    pub fn aes_done(&self) -> AES_DONE_R {
        AES_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 1) & 0x3f)
    }
    #[doc = "Bit 7 - AES Operation Finished with Error"]
    #[inline(always)]
    pub fn aes_er(&self) -> AES_ER_R {
        AES_ER_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Operation Finished with Success"]
    #[inline(always)]
    #[must_use]
    pub fn aes_done(&mut self) -> AES_DONE_W<AES_STATUS_SPEC> {
        AES_DONE_W::new(self, 0)
    }
    #[doc = "Bits 1:6 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<AES_STATUS_SPEC> {
        RES_W::new(self, 1)
    }
    #[doc = "Bit 7 - AES Operation Finished with Error"]
    #[inline(always)]
    #[must_use]
    pub fn aes_er(&mut self) -> AES_ER_W<AES_STATUS_SPEC> {
        AES_ER_W::new(self, 7)
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
#[doc = "AES Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AES_STATUS_SPEC;
impl crate::RegisterSpec for AES_STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aes_status::R`](R) reader structure"]
impl crate::Readable for AES_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aes_status::W`](W) writer structure"]
impl crate::Writable for AES_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_STATUS to value 0"]
impl crate::Resettable for AES_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
