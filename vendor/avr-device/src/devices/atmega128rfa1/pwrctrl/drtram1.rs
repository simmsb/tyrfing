#[doc = "Register `DRTRAM1` reader"]
pub type R = crate::R<DRTRAM1_SPEC>;
#[doc = "Register `DRTRAM1` writer"]
pub type W = crate::W<DRTRAM1_SPEC>;
#[doc = "Field `ENDRT` reader - Enable SRAM Data Retention"]
pub type ENDRT_R = crate::BitReader;
#[doc = "Field `ENDRT` writer - Enable SRAM Data Retention"]
pub type ENDRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRTSWOK` reader - DRT Switch OK"]
pub type DRTSWOK_R = crate::BitReader;
#[doc = "Field `DRTSWOK` writer - DRT Switch OK"]
pub type DRTSWOK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
impl R {
    #[doc = "Bit 4 - Enable SRAM Data Retention"]
    #[inline(always)]
    pub fn endrt(&self) -> ENDRT_R {
        ENDRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DRT Switch OK"]
    #[inline(always)]
    pub fn drtswok(&self) -> DRTSWOK_R {
        DRTSWOK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 4 - Enable SRAM Data Retention"]
    #[inline(always)]
    #[must_use]
    pub fn endrt(&mut self) -> ENDRT_W<DRTRAM1_SPEC> {
        ENDRT_W::new(self, 4)
    }
    #[doc = "Bit 5 - DRT Switch OK"]
    #[inline(always)]
    #[must_use]
    pub fn drtswok(&mut self) -> DRTSWOK_W<DRTRAM1_SPEC> {
        DRTSWOK_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<DRTRAM1_SPEC> {
        RES_W::new(self, 6)
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
#[doc = "Data Retention Configuration Register of SRAM 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drtram1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drtram1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRTRAM1_SPEC;
impl crate::RegisterSpec for DRTRAM1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`drtram1::R`](R) reader structure"]
impl crate::Readable for DRTRAM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`drtram1::W`](W) writer structure"]
impl crate::Writable for DRTRAM1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DRTRAM1 to value 0"]
impl crate::Resettable for DRTRAM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
