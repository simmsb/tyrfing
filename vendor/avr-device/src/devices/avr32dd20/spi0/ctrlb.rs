#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CTRLB_SPEC>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CTRLB_SPEC>;
#[doc = "Field `MODE` reader - SPI Mode"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "SPI Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: SPI Mode 0"]
    _0 = 0,
    #[doc = "1: SPI Mode 1"]
    _1 = 1,
    #[doc = "2: SPI Mode 2"]
    _2 = 2,
    #[doc = "3: SPI Mode 3"]
    _3 = 3,
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
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::_0,
            1 => MODE_A::_1,
            2 => MODE_A::_2,
            3 => MODE_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "SPI Mode 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODE_A::_0
    }
    #[doc = "SPI Mode 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODE_A::_1
    }
    #[doc = "SPI Mode 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == MODE_A::_2
    }
    #[doc = "SPI Mode 3"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == MODE_A::_3
    }
}
#[doc = "Field `MODE` writer - SPI Mode"]
pub type MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI Mode 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_0)
    }
    #[doc = "SPI Mode 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_1)
    }
    #[doc = "SPI Mode 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_2)
    }
    #[doc = "SPI Mode 3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_3)
    }
}
#[doc = "Field `SSD` reader - SPI Select Disable"]
pub type SSD_R = crate::BitReader;
#[doc = "Field `SSD` writer - SPI Select Disable"]
pub type SSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFWR` reader - Buffer Mode Wait for Receive"]
pub type BUFWR_R = crate::BitReader;
#[doc = "Field `BUFWR` writer - Buffer Mode Wait for Receive"]
pub type BUFWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFEN` reader - Buffer Mode Enable"]
pub type BUFEN_R = crate::BitReader;
#[doc = "Field `BUFEN` writer - Buffer Mode Enable"]
pub type BUFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SPI Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - SPI Select Disable"]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Buffer Mode Wait for Receive"]
    #[inline(always)]
    pub fn bufwr(&self) -> BUFWR_R {
        BUFWR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Buffer Mode Enable"]
    #[inline(always)]
    pub fn bufen(&self) -> BUFEN_R {
        BUFEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CTRLB_SPEC> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - SPI Select Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ssd(&mut self) -> SSD_W<CTRLB_SPEC> {
        SSD_W::new(self, 2)
    }
    #[doc = "Bit 6 - Buffer Mode Wait for Receive"]
    #[inline(always)]
    #[must_use]
    pub fn bufwr(&mut self) -> BUFWR_W<CTRLB_SPEC> {
        BUFWR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Buffer Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufen(&mut self) -> BUFEN_W<CTRLB_SPEC> {
        BUFEN_W::new(self, 7)
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
