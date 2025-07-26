#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CTRLA_SPEC>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CTRLA_SPEC>;
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::FieldReader<CMD_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: No Command"]
    NONE = 0,
    #[doc = "1: Write page"]
    PAGEWRITE = 1,
    #[doc = "2: Erase page"]
    PAGEERASE = 2,
    #[doc = "3: Erase and write page"]
    PAGEERASEWRITE = 3,
    #[doc = "4: Page buffer clear"]
    PAGEBUFCLR = 4,
    #[doc = "5: Chip erase"]
    CHIPERASE = 5,
    #[doc = "6: EEPROM erase"]
    EEERASE = 6,
    #[doc = "7: Write fuse (PDI only)"]
    FUSEWRITE = 7,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMD_A {
    type Ux = u8;
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD_A {
        match self.bits {
            0 => CMD_A::NONE,
            1 => CMD_A::PAGEWRITE,
            2 => CMD_A::PAGEERASE,
            3 => CMD_A::PAGEERASEWRITE,
            4 => CMD_A::PAGEBUFCLR,
            5 => CMD_A::CHIPERASE,
            6 => CMD_A::EEERASE,
            7 => CMD_A::FUSEWRITE,
            _ => unreachable!(),
        }
    }
    #[doc = "No Command"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMD_A::NONE
    }
    #[doc = "Write page"]
    #[inline(always)]
    pub fn is_pagewrite(&self) -> bool {
        *self == CMD_A::PAGEWRITE
    }
    #[doc = "Erase page"]
    #[inline(always)]
    pub fn is_pageerase(&self) -> bool {
        *self == CMD_A::PAGEERASE
    }
    #[doc = "Erase and write page"]
    #[inline(always)]
    pub fn is_pageerasewrite(&self) -> bool {
        *self == CMD_A::PAGEERASEWRITE
    }
    #[doc = "Page buffer clear"]
    #[inline(always)]
    pub fn is_pagebufclr(&self) -> bool {
        *self == CMD_A::PAGEBUFCLR
    }
    #[doc = "Chip erase"]
    #[inline(always)]
    pub fn is_chiperase(&self) -> bool {
        *self == CMD_A::CHIPERASE
    }
    #[doc = "EEPROM erase"]
    #[inline(always)]
    pub fn is_eeerase(&self) -> bool {
        *self == CMD_A::EEERASE
    }
    #[doc = "Write fuse (PDI only)"]
    #[inline(always)]
    pub fn is_fusewrite(&self) -> bool {
        *self == CMD_A::FUSEWRITE
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, CMD_A>;
impl<'a, REG> CMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Command"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::NONE)
    }
    #[doc = "Write page"]
    #[inline(always)]
    pub fn pagewrite(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::PAGEWRITE)
    }
    #[doc = "Erase page"]
    #[inline(always)]
    pub fn pageerase(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::PAGEERASE)
    }
    #[doc = "Erase and write page"]
    #[inline(always)]
    pub fn pageerasewrite(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::PAGEERASEWRITE)
    }
    #[doc = "Page buffer clear"]
    #[inline(always)]
    pub fn pagebufclr(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::PAGEBUFCLR)
    }
    #[doc = "Chip erase"]
    #[inline(always)]
    pub fn chiperase(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::CHIPERASE)
    }
    #[doc = "EEPROM erase"]
    #[inline(always)]
    pub fn eeerase(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::EEERASE)
    }
    #[doc = "Write fuse (PDI only)"]
    #[inline(always)]
    pub fn fusewrite(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::FUSEWRITE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<CTRLA_SPEC> {
        CMD_W::new(self, 0)
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
#[doc = "Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
