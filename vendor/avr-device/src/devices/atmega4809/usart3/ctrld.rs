#[doc = "Register `CTRLD` reader"]
pub type R = crate::R<CTRLD_SPEC>;
#[doc = "Register `CTRLD` writer"]
pub type W = crate::W<CTRLD_SPEC>;
#[doc = "Field `ABW` reader - Auto Baud Window"]
pub type ABW_R = crate::FieldReader<ABW_A>;
#[doc = "Auto Baud Window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABW_A {
    #[doc = "0: 18% tolerance"]
    WDW0 = 0,
    #[doc = "1: 15% tolerance"]
    WDW1 = 1,
    #[doc = "2: 21% tolerance"]
    WDW2 = 2,
    #[doc = "3: 25% tolerance"]
    WDW3 = 3,
}
impl From<ABW_A> for u8 {
    #[inline(always)]
    fn from(variant: ABW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ABW_A {
    type Ux = u8;
}
impl ABW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABW_A {
        match self.bits {
            0 => ABW_A::WDW0,
            1 => ABW_A::WDW1,
            2 => ABW_A::WDW2,
            3 => ABW_A::WDW3,
            _ => unreachable!(),
        }
    }
    #[doc = "18% tolerance"]
    #[inline(always)]
    pub fn is_wdw0(&self) -> bool {
        *self == ABW_A::WDW0
    }
    #[doc = "15% tolerance"]
    #[inline(always)]
    pub fn is_wdw1(&self) -> bool {
        *self == ABW_A::WDW1
    }
    #[doc = "21% tolerance"]
    #[inline(always)]
    pub fn is_wdw2(&self) -> bool {
        *self == ABW_A::WDW2
    }
    #[doc = "25% tolerance"]
    #[inline(always)]
    pub fn is_wdw3(&self) -> bool {
        *self == ABW_A::WDW3
    }
}
#[doc = "Field `ABW` writer - Auto Baud Window"]
pub type ABW_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ABW_A>;
impl<'a, REG> ABW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "18% tolerance"]
    #[inline(always)]
    pub fn wdw0(self) -> &'a mut crate::W<REG> {
        self.variant(ABW_A::WDW0)
    }
    #[doc = "15% tolerance"]
    #[inline(always)]
    pub fn wdw1(self) -> &'a mut crate::W<REG> {
        self.variant(ABW_A::WDW1)
    }
    #[doc = "21% tolerance"]
    #[inline(always)]
    pub fn wdw2(self) -> &'a mut crate::W<REG> {
        self.variant(ABW_A::WDW2)
    }
    #[doc = "25% tolerance"]
    #[inline(always)]
    pub fn wdw3(self) -> &'a mut crate::W<REG> {
        self.variant(ABW_A::WDW3)
    }
}
impl R {
    #[doc = "Bits 6:7 - Auto Baud Window"]
    #[inline(always)]
    pub fn abw(&self) -> ABW_R {
        ABW_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 6:7 - Auto Baud Window"]
    #[inline(always)]
    #[must_use]
    pub fn abw(&mut self) -> ABW_W<CTRLD_SPEC> {
        ABW_W::new(self, 6)
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
#[doc = "Control D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLD_SPEC;
impl crate::RegisterSpec for CTRLD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrld::R`](R) reader structure"]
impl crate::Readable for CTRLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrld::W`](W) writer structure"]
impl crate::Writable for CTRLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::Resettable for CTRLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
