#[doc = "Register `HIGH` reader"]
pub type R = crate::R<HIGH_SPEC>;
#[doc = "Register `HIGH` writer"]
pub type W = crate::W<HIGH_SPEC>;
#[doc = "Field `BOOTRST` reader - Boot Reset vector Enabled"]
pub type BOOTRST_R = crate::BitReader;
#[doc = "Field `BOOTRST` writer - Boot Reset vector Enabled"]
pub type BOOTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTSZ` reader - Select Boot Size"]
pub type BOOTSZ_R = crate::FieldReader<BOOTSZ_A>;
#[doc = "Select Boot Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOTSZ_A {
    #[doc = "0: Boot Flash size=1024 words Boot address=$0C00"]
    _1024W_0C00 = 0,
    #[doc = "1: Boot Flash size=512 words Boot address=$0E00"]
    _512W_0E00 = 1,
    #[doc = "2: Boot Flash size=256 words Boot address=$0F00"]
    _256W_0F00 = 2,
    #[doc = "3: Boot Flash size=128 words Boot address=$0F80"]
    _128W_0F80 = 3,
}
impl From<BOOTSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOTSZ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOOTSZ_A {
    type Ux = u8;
}
impl BOOTSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOOTSZ_A {
        match self.bits {
            0 => BOOTSZ_A::_1024W_0C00,
            1 => BOOTSZ_A::_512W_0E00,
            2 => BOOTSZ_A::_256W_0F00,
            3 => BOOTSZ_A::_128W_0F80,
            _ => unreachable!(),
        }
    }
    #[doc = "Boot Flash size=1024 words Boot address=$0C00"]
    #[inline(always)]
    pub fn is_1024w_0c00(&self) -> bool {
        *self == BOOTSZ_A::_1024W_0C00
    }
    #[doc = "Boot Flash size=512 words Boot address=$0E00"]
    #[inline(always)]
    pub fn is_512w_0e00(&self) -> bool {
        *self == BOOTSZ_A::_512W_0E00
    }
    #[doc = "Boot Flash size=256 words Boot address=$0F00"]
    #[inline(always)]
    pub fn is_256w_0f00(&self) -> bool {
        *self == BOOTSZ_A::_256W_0F00
    }
    #[doc = "Boot Flash size=128 words Boot address=$0F80"]
    #[inline(always)]
    pub fn is_128w_0f80(&self) -> bool {
        *self == BOOTSZ_A::_128W_0F80
    }
}
#[doc = "Field `BOOTSZ` writer - Select Boot Size"]
pub type BOOTSZ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BOOTSZ_A>;
impl<'a, REG> BOOTSZ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Boot Flash size=1024 words Boot address=$0C00"]
    #[inline(always)]
    pub fn _1024w_0c00(self) -> &'a mut crate::W<REG> {
        self.variant(BOOTSZ_A::_1024W_0C00)
    }
    #[doc = "Boot Flash size=512 words Boot address=$0E00"]
    #[inline(always)]
    pub fn _512w_0e00(self) -> &'a mut crate::W<REG> {
        self.variant(BOOTSZ_A::_512W_0E00)
    }
    #[doc = "Boot Flash size=256 words Boot address=$0F00"]
    #[inline(always)]
    pub fn _256w_0f00(self) -> &'a mut crate::W<REG> {
        self.variant(BOOTSZ_A::_256W_0F00)
    }
    #[doc = "Boot Flash size=128 words Boot address=$0F80"]
    #[inline(always)]
    pub fn _128w_0f80(self) -> &'a mut crate::W<REG> {
        self.variant(BOOTSZ_A::_128W_0F80)
    }
}
#[doc = "Field `EESAVE` reader - Preserve EEPROM through the Chip Erase cycle"]
pub type EESAVE_R = crate::BitReader;
#[doc = "Field `EESAVE` writer - Preserve EEPROM through the Chip Erase cycle"]
pub type EESAVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKOPT` reader - CKOPT fuse (operation dependent of CKSEL fuses)"]
pub type CKOPT_R = crate::BitReader;
#[doc = "Field `CKOPT` writer - CKOPT fuse (operation dependent of CKSEL fuses)"]
pub type CKOPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIEN` reader - Serial program downloading (SPI) enabled"]
pub type SPIEN_R = crate::BitReader;
#[doc = "Field `SPIEN` writer - Serial program downloading (SPI) enabled"]
pub type SPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTON` reader - Watch-dog Timer always on"]
pub type WDTON_R = crate::BitReader;
#[doc = "Field `WDTON` writer - Watch-dog Timer always on"]
pub type WDTON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDISBL` reader - Reset Disabled (Enable PC6 as i/o pin)"]
pub type RSTDISBL_R = crate::BitReader;
#[doc = "Field `RSTDISBL` writer - Reset Disabled (Enable PC6 as i/o pin)"]
pub type RSTDISBL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Boot Reset vector Enabled"]
    #[inline(always)]
    pub fn bootrst(&self) -> BOOTRST_R {
        BOOTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Select Boot Size"]
    #[inline(always)]
    pub fn bootsz(&self) -> BOOTSZ_R {
        BOOTSZ_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Preserve EEPROM through the Chip Erase cycle"]
    #[inline(always)]
    pub fn eesave(&self) -> EESAVE_R {
        EESAVE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CKOPT fuse (operation dependent of CKSEL fuses)"]
    #[inline(always)]
    pub fn ckopt(&self) -> CKOPT_R {
        CKOPT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Serial program downloading (SPI) enabled"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watch-dog Timer always on"]
    #[inline(always)]
    pub fn wdton(&self) -> WDTON_R {
        WDTON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset Disabled (Enable PC6 as i/o pin)"]
    #[inline(always)]
    pub fn rstdisbl(&self) -> RSTDISBL_R {
        RSTDISBL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Reset vector Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bootrst(&mut self) -> BOOTRST_W<HIGH_SPEC> {
        BOOTRST_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Select Boot Size"]
    #[inline(always)]
    #[must_use]
    pub fn bootsz(&mut self) -> BOOTSZ_W<HIGH_SPEC> {
        BOOTSZ_W::new(self, 1)
    }
    #[doc = "Bit 3 - Preserve EEPROM through the Chip Erase cycle"]
    #[inline(always)]
    #[must_use]
    pub fn eesave(&mut self) -> EESAVE_W<HIGH_SPEC> {
        EESAVE_W::new(self, 3)
    }
    #[doc = "Bit 4 - CKOPT fuse (operation dependent of CKSEL fuses)"]
    #[inline(always)]
    #[must_use]
    pub fn ckopt(&mut self) -> CKOPT_W<HIGH_SPEC> {
        CKOPT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Serial program downloading (SPI) enabled"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<HIGH_SPEC> {
        SPIEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Watch-dog Timer always on"]
    #[inline(always)]
    #[must_use]
    pub fn wdton(&mut self) -> WDTON_W<HIGH_SPEC> {
        WDTON_W::new(self, 6)
    }
    #[doc = "Bit 7 - Reset Disabled (Enable PC6 as i/o pin)"]
    #[inline(always)]
    #[must_use]
    pub fn rstdisbl(&mut self) -> RSTDISBL_W<HIGH_SPEC> {
        RSTDISBL_W::new(self, 7)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIGH_SPEC;
impl crate::RegisterSpec for HIGH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`high::R`](R) reader structure"]
impl crate::Readable for HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`high::W`](W) writer structure"]
impl crate::Writable for HIGH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIGH to value 0"]
impl crate::Resettable for HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
