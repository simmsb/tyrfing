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
    #[doc = "0: No command"]
    NONE = 0,
    #[doc = "1: Write page"]
    WP = 1,
    #[doc = "2: Erase page"]
    ER = 2,
    #[doc = "3: Erase and write page"]
    ERWP = 3,
    #[doc = "4: Page buffer clear"]
    PBC = 4,
    #[doc = "5: Chip erase"]
    CHER = 5,
    #[doc = "6: EEPROM erase"]
    EEER = 6,
    #[doc = "7: Write fuse (PDI only)"]
    WFU = 7,
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
            1 => CMD_A::WP,
            2 => CMD_A::ER,
            3 => CMD_A::ERWP,
            4 => CMD_A::PBC,
            5 => CMD_A::CHER,
            6 => CMD_A::EEER,
            7 => CMD_A::WFU,
            _ => unreachable!(),
        }
    }
    #[doc = "No command"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMD_A::NONE
    }
    #[doc = "Write page"]
    #[inline(always)]
    pub fn is_wp(&self) -> bool {
        *self == CMD_A::WP
    }
    #[doc = "Erase page"]
    #[inline(always)]
    pub fn is_er(&self) -> bool {
        *self == CMD_A::ER
    }
    #[doc = "Erase and write page"]
    #[inline(always)]
    pub fn is_erwp(&self) -> bool {
        *self == CMD_A::ERWP
    }
    #[doc = "Page buffer clear"]
    #[inline(always)]
    pub fn is_pbc(&self) -> bool {
        *self == CMD_A::PBC
    }
    #[doc = "Chip erase"]
    #[inline(always)]
    pub fn is_cher(&self) -> bool {
        *self == CMD_A::CHER
    }
    #[doc = "EEPROM erase"]
    #[inline(always)]
    pub fn is_eeer(&self) -> bool {
        *self == CMD_A::EEER
    }
    #[doc = "Write fuse (PDI only)"]
    #[inline(always)]
    pub fn is_wfu(&self) -> bool {
        *self == CMD_A::WFU
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, CMD_A>;
impl<'a, REG> CMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No command"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::NONE)
    }
    #[doc = "Write page"]
    #[inline(always)]
    pub fn wp(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::WP)
    }
    #[doc = "Erase page"]
    #[inline(always)]
    pub fn er(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::ER)
    }
    #[doc = "Erase and write page"]
    #[inline(always)]
    pub fn erwp(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::ERWP)
    }
    #[doc = "Page buffer clear"]
    #[inline(always)]
    pub fn pbc(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::PBC)
    }
    #[doc = "Chip erase"]
    #[inline(always)]
    pub fn cher(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::CHER)
    }
    #[doc = "EEPROM erase"]
    #[inline(always)]
    pub fn eeer(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::EEER)
    }
    #[doc = "Write fuse (PDI only)"]
    #[inline(always)]
    pub fn wfu(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::WFU)
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
