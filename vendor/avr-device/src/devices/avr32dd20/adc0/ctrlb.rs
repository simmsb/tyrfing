#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `SAMPNUM` reader - Accumulation Samples"]
pub type SAMPNUM_R = crate::FieldReader<SAMPNUM_A>;
#[doc = "Accumulation Samples\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAMPNUM_A {
    #[doc = "0: No accumulation"]
    NONE = 0,
    #[doc = "1: 2 results accumulated"]
    ACC2 = 1,
    #[doc = "2: 4 results accumulated"]
    ACC4 = 2,
    #[doc = "3: 8 results accumulated"]
    ACC8 = 3,
    #[doc = "4: 16 results accumulated"]
    ACC16 = 4,
    #[doc = "5: 32 results accumulated"]
    ACC32 = 5,
    #[doc = "6: 64 results accumulated"]
    ACC64 = 6,
    #[doc = "7: 128 results accumulated"]
    ACC128 = 7,
}
impl From<SAMPNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMPNUM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAMPNUM_A {
    type Ux = u8;
}
impl SAMPNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAMPNUM_A {
        match self.bits {
            0 => SAMPNUM_A::NONE,
            1 => SAMPNUM_A::ACC2,
            2 => SAMPNUM_A::ACC4,
            3 => SAMPNUM_A::ACC8,
            4 => SAMPNUM_A::ACC16,
            5 => SAMPNUM_A::ACC32,
            6 => SAMPNUM_A::ACC64,
            7 => SAMPNUM_A::ACC128,
            _ => unreachable!(),
        }
    }
    #[doc = "No accumulation"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SAMPNUM_A::NONE
    }
    #[doc = "2 results accumulated"]
    #[inline(always)]
    pub fn is_acc2(&self) -> bool {
        *self == SAMPNUM_A::ACC2
    }
    #[doc = "4 results accumulated"]
    #[inline(always)]
    pub fn is_acc4(&self) -> bool {
        *self == SAMPNUM_A::ACC4
    }
    #[doc = "8 results accumulated"]
    #[inline(always)]
    pub fn is_acc8(&self) -> bool {
        *self == SAMPNUM_A::ACC8
    }
    #[doc = "16 results accumulated"]
    #[inline(always)]
    pub fn is_acc16(&self) -> bool {
        *self == SAMPNUM_A::ACC16
    }
    #[doc = "32 results accumulated"]
    #[inline(always)]
    pub fn is_acc32(&self) -> bool {
        *self == SAMPNUM_A::ACC32
    }
    #[doc = "64 results accumulated"]
    #[inline(always)]
    pub fn is_acc64(&self) -> bool {
        *self == SAMPNUM_A::ACC64
    }
    #[doc = "128 results accumulated"]
    #[inline(always)]
    pub fn is_acc128(&self) -> bool {
        *self == SAMPNUM_A::ACC128
    }
}
#[doc = "Field `SAMPNUM` writer - Accumulation Samples"]
pub type SAMPNUM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SAMPNUM_A>;
impl<'a, REG> SAMPNUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No accumulation"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPNUM_A::NONE)
    }
    #[doc = "2 results accumulated"]
    #[inline(always)]
    pub fn acc2(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPNUM_A::ACC2)
    }
    #[doc = "4 results accumulated"]
    #[inline(always)]
    pub fn acc4(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPNUM_A::ACC4)
    }
    #[doc = "8 results accumulated"]
    #[inline(always)]
    pub fn acc8(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPNUM_A::ACC8)
    }
    #[doc = "16 results accumulated"]
    #[inline(always)]
    pub fn acc16(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPNUM_A::ACC16)
    }
    #[doc = "32 results accumulated"]
    #[inline(always)]
    pub fn acc32(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPNUM_A::ACC32)
    }
    #[doc = "64 results accumulated"]
    #[inline(always)]
    pub fn acc64(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPNUM_A::ACC64)
    }
    #[doc = "128 results accumulated"]
    #[inline(always)]
    pub fn acc128(self) -> &'a mut crate::W<REG> {
        self.variant(SAMPNUM_A::ACC128)
    }
}
impl R {
    #[doc = "Bits 0:2 - Accumulation Samples"]
    #[inline(always)]
    pub fn sampnum(&self) -> SAMPNUM_R {
        SAMPNUM_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Accumulation Samples"]
    #[inline(always)]
    #[must_use]
    pub fn sampnum(&mut self) -> SAMPNUM_W<CTRLB_SPEC> {
        SAMPNUM_W::new(self, 0)
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
