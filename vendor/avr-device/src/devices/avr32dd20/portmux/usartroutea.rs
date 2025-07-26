#[doc = "Register `USARTROUTEA` reader"]
pub type R = crate::R<USARTROUTEA_SPEC>;
#[doc = "Register `USARTROUTEA` writer"]
pub type W = crate::W<USARTROUTEA_SPEC>;
#[doc = "Field `USART0` reader - USART0 Signals"]
pub type USART0_R = crate::FieldReader<USART0_A>;
#[doc = "USART0 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART0_A {
    #[doc = "0: TxD: PA0, RxD: PA1, XCK: PA2, XDIR: PA3"]
    DEFAULT = 0,
    #[doc = "1: TxD: PA4, RxD: PA5, XCK: PA6, XDIR: PA7"]
    ALT1 = 1,
    #[doc = "2: TxD: PA2, RxD: PA3, XCK: -, XDIR: -"]
    ALT2 = 2,
    #[doc = "3: TxD: PD4, RxD: PD5, XCK: PD6, XDIR: PD7"]
    ALT3 = 3,
    #[doc = "4: TxD: PC1, RxD: PC2, XCK: PC3, XDIR: -"]
    ALT4 = 4,
    #[doc = "5: Not connected to any pins"]
    NONE = 5,
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
            2 => Some(USART0_A::ALT2),
            3 => Some(USART0_A::ALT3),
            4 => Some(USART0_A::ALT4),
            5 => Some(USART0_A::NONE),
            _ => None,
        }
    }
    #[doc = "TxD: PA0, RxD: PA1, XCK: PA2, XDIR: PA3"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == USART0_A::DEFAULT
    }
    #[doc = "TxD: PA4, RxD: PA5, XCK: PA6, XDIR: PA7"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == USART0_A::ALT1
    }
    #[doc = "TxD: PA2, RxD: PA3, XCK: -, XDIR: -"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == USART0_A::ALT2
    }
    #[doc = "TxD: PD4, RxD: PD5, XCK: PD6, XDIR: PD7"]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        *self == USART0_A::ALT3
    }
    #[doc = "TxD: PC1, RxD: PC2, XCK: PC3, XDIR: -"]
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        *self == USART0_A::ALT4
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == USART0_A::NONE
    }
}
#[doc = "Field `USART0` writer - USART0 Signals"]
pub type USART0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, USART0_A>;
impl<'a, REG> USART0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TxD: PA0, RxD: PA1, XCK: PA2, XDIR: PA3"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_A::DEFAULT)
    }
    #[doc = "TxD: PA4, RxD: PA5, XCK: PA6, XDIR: PA7"]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_A::ALT1)
    }
    #[doc = "TxD: PA2, RxD: PA3, XCK: -, XDIR: -"]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_A::ALT2)
    }
    #[doc = "TxD: PD4, RxD: PD5, XCK: PD6, XDIR: PD7"]
    #[inline(always)]
    pub fn alt3(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_A::ALT3)
    }
    #[doc = "TxD: PC1, RxD: PC2, XCK: PC3, XDIR: -"]
    #[inline(always)]
    pub fn alt4(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_A::ALT4)
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(USART0_A::NONE)
    }
}
#[doc = "Field `USART1` reader - USART1 Signals"]
pub type USART1_R = crate::FieldReader<USART1_A>;
#[doc = "USART1 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1_A {
    #[doc = "0: TxD: -, RxD: PC1, XCK: PC2, XDIR: PC3"]
    DEFAULT = 0,
    #[doc = "2: TxD: PD6, RxD: PD7, XCK: -, XDIR: -"]
    ALT2 = 2,
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
            2 => Some(USART1_A::ALT2),
            3 => Some(USART1_A::NONE),
            _ => None,
        }
    }
    #[doc = "TxD: -, RxD: PC1, XCK: PC2, XDIR: PC3"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == USART1_A::DEFAULT
    }
    #[doc = "TxD: PD6, RxD: PD7, XCK: -, XDIR: -"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == USART1_A::ALT2
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == USART1_A::NONE
    }
}
#[doc = "Field `USART1` writer - USART1 Signals"]
pub type USART1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, USART1_A>;
impl<'a, REG> USART1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TxD: -, RxD: PC1, XCK: PC2, XDIR: PC3"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_A::DEFAULT)
    }
    #[doc = "TxD: PD6, RxD: PD7, XCK: -, XDIR: -"]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_A::ALT2)
    }
    #[doc = "Not connected to any pins"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - USART0 Signals"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:4 - USART1 Signals"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new((self.bits >> 3) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2 - USART0 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> USART0_W<USARTROUTEA_SPEC> {
        USART0_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - USART1 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<USARTROUTEA_SPEC> {
        USART1_W::new(self, 3)
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
#[doc = "USART route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usartroutea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usartroutea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
