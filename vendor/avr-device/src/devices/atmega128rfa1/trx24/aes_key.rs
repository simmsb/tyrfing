#[doc = "Register `AES_KEY` reader"]
pub type R = crate::R<AES_KEY_SPEC>;
#[doc = "Register `AES_KEY` writer"]
pub type W = crate::W<AES_KEY_SPEC>;
#[doc = "Field `AES_KEY` reader - AES Encryption/Decryption Key Buffer"]
pub type AES_KEY_R = crate::FieldReader;
#[doc = "Field `AES_KEY` writer - AES Encryption/Decryption Key Buffer"]
pub type AES_KEY_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - AES Encryption/Decryption Key Buffer"]
    #[inline(always)]
    pub fn aes_key(&self) -> AES_KEY_R {
        AES_KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - AES Encryption/Decryption Key Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn aes_key(&mut self) -> AES_KEY_W<AES_KEY_SPEC> {
        AES_KEY_W::new(self, 0)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AES Encryption and Decryption Key Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_key::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_key::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AES_KEY_SPEC;
impl crate::RegisterSpec for AES_KEY_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aes_key::R`](R) reader structure"]
impl crate::Readable for AES_KEY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aes_key::W`](W) writer structure"]
impl crate::Writable for AES_KEY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_KEY to value 0"]
impl crate::Resettable for AES_KEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
