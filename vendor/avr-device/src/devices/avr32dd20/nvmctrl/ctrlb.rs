#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `APPCODEWP` reader - Application Code Write Protect"]
pub type APPCODEWP_R = crate::BitReader;
#[doc = "Field `APPCODEWP` writer - Application Code Write Protect"]
pub type APPCODEWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTRP` reader - Boot Read Protect"]
pub type BOOTRP_R = crate::BitReader;
#[doc = "Field `BOOTRP` writer - Boot Read Protect"]
pub type BOOTRP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPDATAWP` reader - Application Data Write Protect"]
pub type APPDATAWP_R = crate::BitReader;
#[doc = "Field `APPDATAWP` writer - Application Data Write Protect"]
pub type APPDATAWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLMAP` reader - Flash Mapping in Data space"]
pub type FLMAP_R = crate::FieldReader<FLMAP_A>;
#[doc = "Flash Mapping in Data space\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLMAP_A {
    #[doc = "0: Flash section 0"]
    SECTION0 = 0,
    #[doc = "1: Flash section 1"]
    SECTION1 = 1,
    #[doc = "2: Flash section 2"]
    SECTION2 = 2,
    #[doc = "3: Flash section 3"]
    SECTION3 = 3,
}
impl From<FLMAP_A> for u8 {
    #[inline(always)]
    fn from(variant: FLMAP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLMAP_A {
    type Ux = u8;
}
impl FLMAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLMAP_A {
        match self.bits {
            0 => FLMAP_A::SECTION0,
            1 => FLMAP_A::SECTION1,
            2 => FLMAP_A::SECTION2,
            3 => FLMAP_A::SECTION3,
            _ => unreachable!(),
        }
    }
    #[doc = "Flash section 0"]
    #[inline(always)]
    pub fn is_section0(&self) -> bool {
        *self == FLMAP_A::SECTION0
    }
    #[doc = "Flash section 1"]
    #[inline(always)]
    pub fn is_section1(&self) -> bool {
        *self == FLMAP_A::SECTION1
    }
    #[doc = "Flash section 2"]
    #[inline(always)]
    pub fn is_section2(&self) -> bool {
        *self == FLMAP_A::SECTION2
    }
    #[doc = "Flash section 3"]
    #[inline(always)]
    pub fn is_section3(&self) -> bool {
        *self == FLMAP_A::SECTION3
    }
}
#[doc = "Field `FLMAP` writer - Flash Mapping in Data space"]
pub type FLMAP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FLMAP_A>;
impl<'a, REG> FLMAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash section 0"]
    #[inline(always)]
    pub fn section0(self) -> &'a mut crate::W<REG> {
        self.variant(FLMAP_A::SECTION0)
    }
    #[doc = "Flash section 1"]
    #[inline(always)]
    pub fn section1(self) -> &'a mut crate::W<REG> {
        self.variant(FLMAP_A::SECTION1)
    }
    #[doc = "Flash section 2"]
    #[inline(always)]
    pub fn section2(self) -> &'a mut crate::W<REG> {
        self.variant(FLMAP_A::SECTION2)
    }
    #[doc = "Flash section 3"]
    #[inline(always)]
    pub fn section3(self) -> &'a mut crate::W<REG> {
        self.variant(FLMAP_A::SECTION3)
    }
}
#[doc = "Field `FLMAPLOCK` reader - Flash Mapping Lock"]
pub type FLMAPLOCK_R = crate::BitReader;
#[doc = "Field `FLMAPLOCK` writer - Flash Mapping Lock"]
pub type FLMAPLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Application Code Write Protect"]
    #[inline(always)]
    pub fn appcodewp(&self) -> APPCODEWP_R {
        APPCODEWP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Boot Read Protect"]
    #[inline(always)]
    pub fn bootrp(&self) -> BOOTRP_R {
        BOOTRP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Application Data Write Protect"]
    #[inline(always)]
    pub fn appdatawp(&self) -> APPDATAWP_R {
        APPDATAWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Flash Mapping in Data space"]
    #[inline(always)]
    pub fn flmap(&self) -> FLMAP_R {
        FLMAP_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - Flash Mapping Lock"]
    #[inline(always)]
    pub fn flmaplock(&self) -> FLMAPLOCK_R {
        FLMAPLOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Application Code Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn appcodewp(&mut self) -> APPCODEWP_W<CTRLB_SPEC> {
        APPCODEWP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Boot Read Protect"]
    #[inline(always)]
    #[must_use]
    pub fn bootrp(&mut self) -> BOOTRP_W<CTRLB_SPEC> {
        BOOTRP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Application Data Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn appdatawp(&mut self) -> APPDATAWP_W<CTRLB_SPEC> {
        APPDATAWP_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Flash Mapping in Data space"]
    #[inline(always)]
    #[must_use]
    pub fn flmap(&mut self) -> FLMAP_W<CTRLB_SPEC> {
        FLMAP_W::new(self, 4)
    }
    #[doc = "Bit 7 - Flash Mapping Lock"]
    #[inline(always)]
    #[must_use]
    pub fn flmaplock(&mut self) -> FLMAPLOCK_W<CTRLB_SPEC> {
        FLMAPLOCK_W::new(self, 7)
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
#[doc = "Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
