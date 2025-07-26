#[doc = "Register `CTRLESET` reader"]
pub type R = crate::R<SPLIT_CTRLESET_SPEC>;
#[doc = "Register `CTRLESET` writer"]
pub type W = crate::W<SPLIT_CTRLESET_SPEC>;
#[doc = "Field `CMDEN` reader - Command Enable"]
pub type CMDEN_R = crate::FieldReader<CMDEN_A>;
#[doc = "Command Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDEN_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "3: Both low byte and high byte counter"]
    BOTH = 3,
}
impl From<CMDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMDEN_A {
    type Ux = u8;
}
impl CMDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMDEN_A> {
        match self.bits {
            0 => Some(CMDEN_A::NONE),
            3 => Some(CMDEN_A::BOTH),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMDEN_A::NONE
    }
    #[doc = "Both low byte and high byte counter"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CMDEN_A::BOTH
    }
}
#[doc = "Field `CMDEN` writer - Command Enable"]
pub type CMDEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CMDEN_A>;
impl<'a, REG> CMDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CMDEN_A::NONE)
    }
    #[doc = "Both low byte and high byte counter"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(CMDEN_A::BOTH)
    }
}
#[doc = "Field `CMD` reader - Command"]
pub type CMD_R = crate::FieldReader<CMD_A>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: No Command"]
    NONE = 0,
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
    pub const fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            0 => Some(CMD_A::NONE),
            2 => Some(CMD_A::RESTART),
            3 => Some(CMD_A::RESET),
            _ => None,
        }
    }
    #[doc = "No Command"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CMD_A::NONE
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
pub type CMD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CMD_A>;
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
    #[doc = "Bits 0:1 - Command Enable"]
    #[inline(always)]
    pub fn cmden(&self) -> CMDEN_R {
        CMDEN_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Command Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmden(&mut self) -> CMDEN_W<SPLIT_CTRLESET_SPEC> {
        CMDEN_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<SPLIT_CTRLESET_SPEC> {
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
#[doc = "Control E Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`split_ctrleset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`split_ctrleset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPLIT_CTRLESET_SPEC;
impl crate::RegisterSpec for SPLIT_CTRLESET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`split_ctrleset::R`](R) reader structure"]
impl crate::Readable for SPLIT_CTRLESET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`split_ctrleset::W`](W) writer structure"]
impl crate::Writable for SPLIT_CTRLESET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLESET to value 0"]
impl crate::Resettable for SPLIT_CTRLESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
