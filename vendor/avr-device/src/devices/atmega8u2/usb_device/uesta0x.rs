#[doc = "Register `UESTA0X` reader"]
pub type R = crate::R<UESTA0X_SPEC>;
#[doc = "Register `UESTA0X` writer"]
pub type W = crate::W<UESTA0X_SPEC>;
#[doc = "Field `NBUSYBK` reader - Busy Bank Flag"]
pub type NBUSYBK_R = crate::FieldReader;
#[doc = "Field `NBUSYBK` writer - Busy Bank Flag"]
pub type NBUSYBK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `DTSEQ` reader - Data Toggle Sequencing Flag"]
pub type DTSEQ_R = crate::FieldReader;
#[doc = "Field `DTSEQ` writer - Data Toggle Sequencing Flag"]
pub type DTSEQ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "Field `UNDERFI` reader - Underflow Error Interrupt Flag"]
pub type UNDERFI_R = crate::BitReader;
#[doc = "Field `UNDERFI` writer - Underflow Error Interrupt Flag"]
pub type UNDERFI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFI` reader - Overflow Error Interrupt Flag"]
pub type OVERFI_R = crate::BitReader;
#[doc = "Field `OVERFI` writer - Overflow Error Interrupt Flag"]
pub type OVERFI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGOK` reader - Configuration Status Flag"]
pub type CFGOK_R = crate::BitReader;
#[doc = "Field `CFGOK` writer - Configuration Status Flag"]
pub type CFGOK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Busy Bank Flag"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NBUSYBK_R {
        NBUSYBK_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Data Toggle Sequencing Flag"]
    #[inline(always)]
    pub fn dtseq(&self) -> DTSEQ_R {
        DTSEQ_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 5 - Underflow Error Interrupt Flag"]
    #[inline(always)]
    pub fn underfi(&self) -> UNDERFI_R {
        UNDERFI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overflow Error Interrupt Flag"]
    #[inline(always)]
    pub fn overfi(&self) -> OVERFI_R {
        OVERFI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configuration Status Flag"]
    #[inline(always)]
    pub fn cfgok(&self) -> CFGOK_R {
        CFGOK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Busy Bank Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nbusybk(&mut self) -> NBUSYBK_W<UESTA0X_SPEC> {
        NBUSYBK_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Data Toggle Sequencing Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dtseq(&mut self) -> DTSEQ_W<UESTA0X_SPEC> {
        DTSEQ_W::new(self, 2)
    }
    #[doc = "Bit 5 - Underflow Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn underfi(&mut self) -> UNDERFI_W<UESTA0X_SPEC> {
        UNDERFI_W::new(self, 5)
    }
    #[doc = "Bit 6 - Overflow Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn overfi(&mut self) -> OVERFI_W<UESTA0X_SPEC> {
        OVERFI_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configuration Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cfgok(&mut self) -> CFGOK_W<UESTA0X_SPEC> {
        CFGOK_W::new(self, 7)
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
#[doc = "USB Endpoint Status 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uesta0x::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uesta0x::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UESTA0X_SPEC;
impl crate::RegisterSpec for UESTA0X_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uesta0x::R`](R) reader structure"]
impl crate::Readable for UESTA0X_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uesta0x::W`](W) writer structure"]
impl crate::Writable for UESTA0X_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UESTA0X to value 0"]
impl crate::Resettable for UESTA0X_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
