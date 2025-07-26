#[doc = "Register `TXDATAH` reader"]
pub type R = crate::R<TXDATAH_SPEC>;
#[doc = "Register `TXDATAH` writer"]
pub type W = crate::W<TXDATAH_SPEC>;
#[doc = "Field `DATA8` reader - Transmit Data Register (CHSIZE=9bit)"]
pub type DATA8_R = crate::BitReader;
#[doc = "Field `DATA8` writer - Transmit Data Register (CHSIZE=9bit)"]
pub type DATA8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Data Register (CHSIZE=9bit)"]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Register (CHSIZE=9bit)"]
    #[inline(always)]
    #[must_use]
    pub fn data8(&mut self) -> DATA8_W<TXDATAH_SPEC> {
        DATA8_W::new(self, 0)
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
#[doc = "Transmit Data High Byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdatah::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdatah::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDATAH_SPEC;
impl crate::RegisterSpec for TXDATAH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txdatah::R`](R) reader structure"]
impl crate::Readable for TXDATAH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdatah::W`](W) writer structure"]
impl crate::Writable for TXDATAH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXDATAH to value 0"]
impl crate::Resettable for TXDATAH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
