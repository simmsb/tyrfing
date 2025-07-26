#[doc = "Register `CTRLECLR` reader"]
pub type R = crate::R<SINGLE_CTRLECLR_SPEC>;
#[doc = "Register `CTRLECLR` writer"]
pub type W = crate::W<SINGLE_CTRLECLR_SPEC>;
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Direction"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUPD` reader - Lock Update"]
pub type LUPD_R = crate::BitReader;
#[doc = "Field `LUPD` writer - Lock Update"]
pub type LUPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::FieldReader<CMD_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: No Command"]
    NONE = 0,
    #[doc = "1: Force Update"]
    UPDATE = 1,
    #[doc = "2: Force Restart"]
    RESTART = 2,
    #[doc = "3: Force Hard Reset"]
    RESET = 3,
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
            1 => CMD_A::UPDATE,
            2 => CMD_A::RESTART,
            3 => CMD_A::RESET,
            _ => unreachable!(),
        }
    }
    #[doc = "No Command"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMD_A::NONE
    }
    #[doc = "Force Update"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == CMD_A::UPDATE
    }
    #[doc = "Force Restart"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == CMD_A::RESTART
    }
    #[doc = "Force Hard Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CMD_A::RESET
    }
}
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CMD_A>;
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
    #[doc = "Force Update"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::UPDATE)
    }
    #[doc = "Force Restart"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::RESTART)
    }
    #[doc = "Force Hard Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::RESET)
    }
}
impl R {
    #[doc = "Bit 0 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    pub fn lupd(&self) -> LUPD_R {
        LUPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<SINGLE_CTRLECLR_SPEC> {
        DIR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Lock Update"]
    #[inline(always)]
    #[must_use]
    pub fn lupd(&mut self) -> LUPD_W<SINGLE_CTRLECLR_SPEC> {
        LUPD_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<SINGLE_CTRLECLR_SPEC> {
        CMD_W::new(self, 2)
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
#[doc = "Control E Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_ctrleclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_ctrleclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLE_CTRLECLR_SPEC;
impl crate::RegisterSpec for SINGLE_CTRLECLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`single_ctrleclr::R`](R) reader structure"]
impl crate::Readable for SINGLE_CTRLECLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`single_ctrleclr::W`](W) writer structure"]
impl crate::Writable for SINGLE_CTRLECLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLECLR to value 0"]
impl crate::Resettable for SINGLE_CTRLECLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
