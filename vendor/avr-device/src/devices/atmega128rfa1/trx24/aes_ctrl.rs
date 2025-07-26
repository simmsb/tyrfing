#[doc = "Register `AES_CTRL` reader"]
pub type R = crate::R<AES_CTRL_SPEC>;
#[doc = "Register `AES_CTRL` writer"]
pub type W = crate::W<AES_CTRL_SPEC>;
#[doc = "Field `AES_IM` reader - AES Interrupt Enable"]
pub type AES_IM_R = crate::BitReader;
#[doc = "Field `AES_IM` writer - AES Interrupt Enable"]
pub type AES_IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_DIR` reader - Set AES Operation Direction"]
pub type AES_DIR_R = crate::BitReader<AES_DIR_A>;
#[doc = "Set AES Operation Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AES_DIR_A {
    #[doc = "0: AES operation is encryption."]
    AES_DIR_ENC = 0,
    #[doc = "1: AES operation is decryption."]
    AES_DIR_DEC = 1,
}
impl From<AES_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: AES_DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl AES_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AES_DIR_A {
        match self.bits {
            false => AES_DIR_A::AES_DIR_ENC,
            true => AES_DIR_A::AES_DIR_DEC,
        }
    }
    #[doc = "AES operation is encryption."]
    #[inline(always)]
    pub fn is_aes_dir_enc(&self) -> bool {
        *self == AES_DIR_A::AES_DIR_ENC
    }
    #[doc = "AES operation is decryption."]
    #[inline(always)]
    pub fn is_aes_dir_dec(&self) -> bool {
        *self == AES_DIR_A::AES_DIR_DEC
    }
}
#[doc = "Field `AES_DIR` writer - Set AES Operation Direction"]
pub type AES_DIR_W<'a, REG> = crate::BitWriter<'a, REG, AES_DIR_A>;
impl<'a, REG> AES_DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AES operation is encryption."]
    #[inline(always)]
    pub fn aes_dir_enc(self) -> &'a mut crate::W<REG> {
        self.variant(AES_DIR_A::AES_DIR_ENC)
    }
    #[doc = "AES operation is decryption."]
    #[inline(always)]
    pub fn aes_dir_dec(self) -> &'a mut crate::W<REG> {
        self.variant(AES_DIR_A::AES_DIR_DEC)
    }
}
#[doc = "Field `AES_MODE` reader - Set AES Operation Mode"]
pub type AES_MODE_R = crate::BitReader<AES_MODE_A>;
#[doc = "Set AES Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AES_MODE_A {
    #[doc = "0: AES Mode is ECB (Electronic Code Book)."]
    AES_MODE_ECB = 0,
    #[doc = "1: AES Mode is CBC (Cipher Block Chaining)."]
    AES_MODE_CBC = 1,
}
impl From<AES_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: AES_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl AES_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AES_MODE_A {
        match self.bits {
            false => AES_MODE_A::AES_MODE_ECB,
            true => AES_MODE_A::AES_MODE_CBC,
        }
    }
    #[doc = "AES Mode is ECB (Electronic Code Book)."]
    #[inline(always)]
    pub fn is_aes_mode_ecb(&self) -> bool {
        *self == AES_MODE_A::AES_MODE_ECB
    }
    #[doc = "AES Mode is CBC (Cipher Block Chaining)."]
    #[inline(always)]
    pub fn is_aes_mode_cbc(&self) -> bool {
        *self == AES_MODE_A::AES_MODE_CBC
    }
}
#[doc = "Field `AES_MODE` writer - Set AES Operation Mode"]
pub type AES_MODE_W<'a, REG> = crate::BitWriter<'a, REG, AES_MODE_A>;
impl<'a, REG> AES_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AES Mode is ECB (Electronic Code Book)."]
    #[inline(always)]
    pub fn aes_mode_ecb(self) -> &'a mut crate::W<REG> {
        self.variant(AES_MODE_A::AES_MODE_ECB)
    }
    #[doc = "AES Mode is CBC (Cipher Block Chaining)."]
    #[inline(always)]
    pub fn aes_mode_cbc(self) -> &'a mut crate::W<REG> {
        self.variant(AES_MODE_A::AES_MODE_CBC)
    }
}
#[doc = "Field `AES_REQUEST` reader - Request AES Operation."]
pub type AES_REQUEST_R = crate::BitReader;
#[doc = "Field `AES_REQUEST` writer - Request AES Operation."]
pub type AES_REQUEST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - AES Interrupt Enable"]
    #[inline(always)]
    pub fn aes_im(&self) -> AES_IM_R {
        AES_IM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set AES Operation Direction"]
    #[inline(always)]
    pub fn aes_dir(&self) -> AES_DIR_R {
        AES_DIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Set AES Operation Mode"]
    #[inline(always)]
    pub fn aes_mode(&self) -> AES_MODE_R {
        AES_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Request AES Operation."]
    #[inline(always)]
    pub fn aes_request(&self) -> AES_REQUEST_R {
        AES_REQUEST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - AES Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aes_im(&mut self) -> AES_IM_W<AES_CTRL_SPEC> {
        AES_IM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set AES Operation Direction"]
    #[inline(always)]
    #[must_use]
    pub fn aes_dir(&mut self) -> AES_DIR_W<AES_CTRL_SPEC> {
        AES_DIR_W::new(self, 3)
    }
    #[doc = "Bit 5 - Set AES Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn aes_mode(&mut self) -> AES_MODE_W<AES_CTRL_SPEC> {
        AES_MODE_W::new(self, 5)
    }
    #[doc = "Bit 7 - Request AES Operation."]
    #[inline(always)]
    #[must_use]
    pub fn aes_request(&mut self) -> AES_REQUEST_W<AES_CTRL_SPEC> {
        AES_REQUEST_W::new(self, 7)
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
#[doc = "AES Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AES_CTRL_SPEC;
impl crate::RegisterSpec for AES_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aes_ctrl::R`](R) reader structure"]
impl crate::Readable for AES_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aes_ctrl::W`](W) writer structure"]
impl crate::Writable for AES_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_CTRL to value 0"]
impl crate::Resettable for AES_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
