#[doc = "Register `MAN_ID_0` reader"]
pub type R = crate::R<MAN_ID_0_SPEC>;
#[doc = "Register `MAN_ID_0` writer"]
pub type W = crate::W<MAN_ID_0_SPEC>;
#[doc = "Field `MAN_ID_0` reader - Manufacturer ID (Low Byte)"]
pub type MAN_ID_0_R = crate::FieldReader;
#[doc = "Field `MAN_ID_0` writer - Manufacturer ID (Low Byte)"]
pub type MAN_ID_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Manufacturer ID (Low Byte)"]
    #[inline(always)]
    pub fn man_id_0(&self) -> MAN_ID_0_R {
        MAN_ID_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Manufacturer ID (Low Byte)"]
    #[inline(always)]
    #[must_use]
    pub fn man_id_0(&mut self) -> MAN_ID_0_W<MAN_ID_0_SPEC> {
        MAN_ID_0_W::new(self, 0)
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
#[doc = "Device Identification Register (Manufacture ID Low Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`man_id_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`man_id_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAN_ID_0_SPEC;
impl crate::RegisterSpec for MAN_ID_0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`man_id_0::R`](R) reader structure"]
impl crate::Readable for MAN_ID_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`man_id_0::W`](W) writer structure"]
impl crate::Writable for MAN_ID_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAN_ID_0 to value 0"]
impl crate::Resettable for MAN_ID_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
