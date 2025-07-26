#[doc = "Register `MAN_ID_1` reader"]
pub type R = crate::R<MAN_ID_1_SPEC>;
#[doc = "Register `MAN_ID_1` writer"]
pub type W = crate::W<MAN_ID_1_SPEC>;
#[doc = "Field `MAN_ID_` reader - Manufacturer ID (High Byte)"]
pub type MAN_ID__R = crate::FieldReader<MAN_ID__A>;
#[doc = "Manufacturer ID (High Byte)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAN_ID__A {
    #[doc = "0: Atmel JEDEC manufacturer ID, bits \\[15:8\\]
of 32 bit manufacturer ID: 00 00 00 1F"]
    ATMEL_BYTE_1 = 0,
}
impl From<MAN_ID__A> for u8 {
    #[inline(always)]
    fn from(variant: MAN_ID__A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MAN_ID__A {
    type Ux = u8;
}
impl MAN_ID__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MAN_ID__A> {
        match self.bits {
            0 => Some(MAN_ID__A::ATMEL_BYTE_1),
            _ => None,
        }
    }
    #[doc = "Atmel JEDEC manufacturer ID, bits \\[15:8\\]
of 32 bit manufacturer ID: 00 00 00 1F"]
    #[inline(always)]
    pub fn is_atmel_byte_1(&self) -> bool {
        *self == MAN_ID__A::ATMEL_BYTE_1
    }
}
#[doc = "Field `MAN_ID_` writer - Manufacturer ID (High Byte)"]
pub type MAN_ID__W<'a, REG> = crate::FieldWriter<'a, REG, 8, MAN_ID__A>;
impl<'a, REG> MAN_ID__W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Atmel JEDEC manufacturer ID, bits \\[15:8\\]
of 32 bit manufacturer ID: 00 00 00 1F"]
    #[inline(always)]
    pub fn atmel_byte_1(self) -> &'a mut crate::W<REG> {
        self.variant(MAN_ID__A::ATMEL_BYTE_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Manufacturer ID (High Byte)"]
    #[inline(always)]
    pub fn man_id_(&self) -> MAN_ID__R {
        MAN_ID__R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Manufacturer ID (High Byte)"]
    #[inline(always)]
    #[must_use]
    pub fn man_id_(&mut self) -> MAN_ID__W<MAN_ID_1_SPEC> {
        MAN_ID__W::new(self, 0)
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
#[doc = "Device Identification Register (Manufacture ID High Byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`man_id_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`man_id_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAN_ID_1_SPEC;
impl crate::RegisterSpec for MAN_ID_1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`man_id_1::R`](R) reader structure"]
impl crate::Readable for MAN_ID_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`man_id_1::W`](W) writer structure"]
impl crate::Writable for MAN_ID_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAN_ID_1 to value 0"]
impl crate::Resettable for MAN_ID_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
