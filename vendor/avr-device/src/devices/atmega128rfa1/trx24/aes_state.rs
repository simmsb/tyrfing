#[doc = "Register `AES_STATE` reader"]
pub type R = crate::R<AES_STATE_SPEC>;
#[doc = "Register `AES_STATE` writer"]
pub type W = crate::W<AES_STATE_SPEC>;
#[doc = "Field `AES_STATE` reader - AES Plain and Cipher Text Buffer"]
pub type AES_STATE_R = crate::FieldReader;
#[doc = "Field `AES_STATE` writer - AES Plain and Cipher Text Buffer"]
pub type AES_STATE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - AES Plain and Cipher Text Buffer"]
    #[inline(always)]
    pub fn aes_state(&self) -> AES_STATE_R {
        AES_STATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - AES Plain and Cipher Text Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn aes_state(&mut self) -> AES_STATE_W<AES_STATE_SPEC> {
        AES_STATE_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AES Plain and Cipher Text Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AES_STATE_SPEC;
impl crate::RegisterSpec for AES_STATE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aes_state::R`](R) reader structure"]
impl crate::Readable for AES_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aes_state::W`](W) writer structure"]
impl crate::Writable for AES_STATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_STATE to value 0"]
impl crate::Resettable for AES_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
