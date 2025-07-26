#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `SRC` reader - CRC Source"]
pub type SRC_R = crate::FieldReader<SRC_A>;
#[doc = "CRC Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: CRC on entire flash"]
    FLASH = 0,
    #[doc = "1: CRC on boot and appl section of flash"]
    APPLICATION = 1,
    #[doc = "2: CRC on boot section of flash"]
    BOOT = 2,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRC_A {
    type Ux = u8;
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            0 => Some(SRC_A::FLASH),
            1 => Some(SRC_A::APPLICATION),
            2 => Some(SRC_A::BOOT),
            _ => None,
        }
    }
    #[doc = "CRC on entire flash"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == SRC_A::FLASH
    }
    #[doc = "CRC on boot and appl section of flash"]
    #[inline(always)]
    pub fn is_application(&self) -> bool {
        *self == SRC_A::APPLICATION
    }
    #[doc = "CRC on boot section of flash"]
    #[inline(always)]
    pub fn is_boot(&self) -> bool {
        *self == SRC_A::BOOT
    }
}
#[doc = "Field `SRC` writer - CRC Source"]
pub type SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SRC_A>;
impl<'a, REG> SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CRC on entire flash"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::FLASH)
    }
    #[doc = "CRC on boot and appl section of flash"]
    #[inline(always)]
    pub fn application(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::APPLICATION)
    }
    #[doc = "CRC on boot section of flash"]
    #[inline(always)]
    pub fn boot(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_A::BOOT)
    }
}
#[doc = "Field `MODE` reader - CRC Flash Access Mode"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "CRC Flash Access Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Priority to flash"]
    PRIORITY = 0,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::PRIORITY),
            _ => None,
        }
    }
    #[doc = "Priority to flash"]
    #[inline(always)]
    pub fn is_priority(&self) -> bool {
        *self == MODE_A::PRIORITY
    }
}
#[doc = "Field `MODE` writer - CRC Flash Access Mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Priority to flash"]
    #[inline(always)]
    pub fn priority(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::PRIORITY)
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC Source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - CRC Flash Access Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC Source"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<CTRLB_SPEC> {
        SRC_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - CRC Flash Access Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRLB_SPEC> {
        MODE_W::new(self, 4)
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
