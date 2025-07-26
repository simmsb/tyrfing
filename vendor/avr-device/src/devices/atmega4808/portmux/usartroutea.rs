#[doc = "Register `USARTROUTEA` reader"]
pub type R = crate::R<USARTROUTEA_SPEC>;
#[doc = "Register `USARTROUTEA` writer"]
pub type W = crate::W<USARTROUTEA_SPEC>;
#[doc = "Field `USART0` reader - Port Multiplexer USART0"]
pub type USART0_R = crate::FieldReader<USART0_A>;
#[doc = "Port Multiplexer USART0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART0_A {
    #[doc = "0: USART0 on PA\\[3:0\\]"]
    DEFAULT = 0,
    #[doc = "1: USART0 on PA\\[7:4\\]"]
    ALT1 = 1,
    #[doc = "3: Not connected to any pins"]
    NONE = 3,
}
impl From<USART0_A> for u8 {
    #[inline(always)]
    fn from(variant: USART0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART0_A {
    type Ux = u8;
}
impl USART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART0_A> {
        match self.bits {
            0 => Some(USART0_A::DEFAULT),
            1 => Some(USART0_A::ALT1),
            3 => Some(USART0_A::NONE),
            _ => None,
        }
    }
    #[doc = "USART0 on PA\\[3:0\\]"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == USART0_A::DEFAULT
    }
    #[doc = "USART0 on PA\\[7:4\\]"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == USART0_A::ALT1
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == USART0_A::NONE
    }
}
#[doc = "Field `USART0` writer - Port Multiplexer USART0"]
pub type USART0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USART0_A>;
impl<'a, REG> USART0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART0 on PA\\[3:0\\]"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_A::DEFAULT)
    }
    #[doc = "USART0 on PA\\[7:4\\]"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_A::ALT1)
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_A::NONE)
    }
}
#[doc = "Field `USART1` reader - Port Multiplexer USART1"]
pub type USART1_R = crate::FieldReader<USART1_A>;
#[doc = "Port Multiplexer USART1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1_A {
    #[doc = "0: USART1 on PC\\[3:0\\]"]
    DEFAULT = 0,
    #[doc = "1: USART1 on PC\\[7:4\\]"]
    ALT1 = 1,
    #[doc = "3: Not connected to any pins"]
    NONE = 3,
}
impl From<USART1_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART1_A {
    type Ux = u8;
}
impl USART1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART1_A> {
        match self.bits {
            0 => Some(USART1_A::DEFAULT),
            1 => Some(USART1_A::ALT1),
            3 => Some(USART1_A::NONE),
            _ => None,
        }
    }
    #[doc = "USART1 on PC\\[3:0\\]"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == USART1_A::DEFAULT
    }
    #[doc = "USART1 on PC\\[7:4\\]"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == USART1_A::ALT1
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == USART1_A::NONE
    }
}
#[doc = "Field `USART1` writer - Port Multiplexer USART1"]
pub type USART1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USART1_A>;
impl<'a, REG> USART1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART1 on PC\\[3:0\\]"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_A::DEFAULT)
    }
    #[doc = "USART1 on PC\\[7:4\\]"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_A::ALT1)
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_A::NONE)
    }
}
#[doc = "Field `USART2` reader - Port Multiplexer USART2"]
pub type USART2_R = crate::FieldReader<USART2_A>;
#[doc = "Port Multiplexer USART2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART2_A {
    #[doc = "0: USART2 on PF\\[3:0\\]"]
    DEFAULT = 0,
    #[doc = "1: USART2 on PF\\[6:4\\]"]
    ALT1 = 1,
    #[doc = "3: Not connected to any pins"]
    NONE = 3,
}
impl From<USART2_A> for u8 {
    #[inline(always)]
    fn from(variant: USART2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART2_A {
    type Ux = u8;
}
impl USART2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART2_A> {
        match self.bits {
            0 => Some(USART2_A::DEFAULT),
            1 => Some(USART2_A::ALT1),
            3 => Some(USART2_A::NONE),
            _ => None,
        }
    }
    #[doc = "USART2 on PF\\[3:0\\]"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == USART2_A::DEFAULT
    }
    #[doc = "USART2 on PF\\[6:4\\]"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == USART2_A::ALT1
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == USART2_A::NONE
    }
}
#[doc = "Field `USART2` writer - Port Multiplexer USART2"]
pub type USART2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USART2_A>;
impl<'a, REG> USART2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART2 on PF\\[3:0\\]"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(USART2_A::DEFAULT)
    }
    #[doc = "USART2 on PF\\[6:4\\]"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(USART2_A::ALT1)
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(USART2_A::NONE)
    }
}
#[doc = "Field `USART3` reader - Port Multiplexer USART3"]
pub type USART3_R = crate::FieldReader<USART3_A>;
#[doc = "Port Multiplexer USART3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART3_A {
    #[doc = "0: USART3 on PB\\[3:0\\]"]
    DEFAULT = 0,
    #[doc = "1: USART3 on PB\\[5:4\\]"]
    ALT1 = 1,
    #[doc = "3: Not connected to any pins"]
    NONE = 3,
}
impl From<USART3_A> for u8 {
    #[inline(always)]
    fn from(variant: USART3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART3_A {
    type Ux = u8;
}
impl USART3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART3_A> {
        match self.bits {
            0 => Some(USART3_A::DEFAULT),
            1 => Some(USART3_A::ALT1),
            3 => Some(USART3_A::NONE),
            _ => None,
        }
    }
    #[doc = "USART3 on PB\\[3:0\\]"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == USART3_A::DEFAULT
    }
    #[doc = "USART3 on PB\\[5:4\\]"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == USART3_A::ALT1
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == USART3_A::NONE
    }
}
#[doc = "Field `USART3` writer - Port Multiplexer USART3"]
pub type USART3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USART3_A>;
impl<'a, REG> USART3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART3 on PB\\[3:0\\]"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(USART3_A::DEFAULT)
    }
    #[doc = "USART3 on PB\\[5:4\\]"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(USART3_A::ALT1)
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(USART3_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Port Multiplexer USART0"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Port Multiplexer USART1"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Port Multiplexer USART2"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Port Multiplexer USART3"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port Multiplexer USART0"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> USART0_W<USARTROUTEA_SPEC> {
        USART0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port Multiplexer USART1"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<USARTROUTEA_SPEC> {
        USART1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port Multiplexer USART2"]
    #[inline(always)]
    #[must_use]
    pub fn usart2(&mut self) -> USART2_W<USARTROUTEA_SPEC> {
        USART2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port Multiplexer USART3"]
    #[inline(always)]
    #[must_use]
    pub fn usart3(&mut self) -> USART3_W<USARTROUTEA_SPEC> {
        USART3_W::new(self, 6)
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
#[doc = "Port Multiplexer USART register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usartroutea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usartroutea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USARTROUTEA_SPEC;
impl crate::RegisterSpec for USARTROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usartroutea::R`](R) reader structure"]
impl crate::Readable for USARTROUTEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usartroutea::W`](W) writer structure"]
impl crate::Writable for USARTROUTEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USARTROUTEA to value 0"]
impl crate::Resettable for USARTROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
