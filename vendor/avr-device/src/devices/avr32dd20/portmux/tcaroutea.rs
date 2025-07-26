#[doc = "Register `TCAROUTEA` reader"]
pub type R = crate::R<TCAROUTEA_SPEC>;
#[doc = "Register `TCAROUTEA` writer"]
pub type W = crate::W<TCAROUTEA_SPEC>;
#[doc = "Field `TCA0` reader - TCA0 Signals"]
pub type TCA0_R = crate::FieldReader<TCA0_A>;
#[doc = "TCA0 Signals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCA0_A {
    #[doc = "0: WOn: PA0, PA1, PA2, PA3, PA4, PA5"]
    PORTA = 0,
    #[doc = "2: WOn: -, PC1, PC2, PC3, -, -"]
    PORTC = 2,
    #[doc = "3: WOn: -, -, -, -, PD4, PD5"]
    PORTD = 3,
}
impl From<TCA0_A> for u8 {
    #[inline(always)]
    fn from(variant: TCA0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCA0_A {
    type Ux = u8;
}
impl TCA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCA0_A> {
        match self.bits {
            0 => Some(TCA0_A::PORTA),
            2 => Some(TCA0_A::PORTC),
            3 => Some(TCA0_A::PORTD),
            _ => None,
        }
    }
    #[doc = "WOn: PA0, PA1, PA2, PA3, PA4, PA5"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == TCA0_A::PORTA
    }
    #[doc = "WOn: -, PC1, PC2, PC3, -, -"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == TCA0_A::PORTC
    }
    #[doc = "WOn: -, -, -, -, PD4, PD5"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == TCA0_A::PORTD
    }
}
#[doc = "Field `TCA0` writer - TCA0 Signals"]
pub type TCA0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TCA0_A>;
impl<'a, REG> TCA0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "WOn: PA0, PA1, PA2, PA3, PA4, PA5"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(TCA0_A::PORTA)
    }
    #[doc = "WOn: -, PC1, PC2, PC3, -, -"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(TCA0_A::PORTC)
    }
    #[doc = "WOn: -, -, -, -, PD4, PD5"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(TCA0_A::PORTD)
    }
}
impl R {
    #[doc = "Bits 0:2 - TCA0 Signals"]
    #[inline(always)]
    pub fn tca0(&self) -> TCA0_R {
        TCA0_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - TCA0 Signals"]
    #[inline(always)]
    #[must_use]
    pub fn tca0(&mut self) -> TCA0_W<TCAROUTEA_SPEC> {
        TCA0_W::new(self, 0)
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
#[doc = "TCA route A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcaroutea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcaroutea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCAROUTEA_SPEC;
impl crate::RegisterSpec for TCAROUTEA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tcaroutea::R`](R) reader structure"]
impl crate::Readable for TCAROUTEA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcaroutea::W`](W) writer structure"]
impl crate::Writable for TCAROUTEA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCAROUTEA to value 0"]
impl crate::Resettable for TCAROUTEA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
