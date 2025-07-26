#[doc = "Register `DPDS1` reader"]
pub type R = crate::R<DPDS1_SPEC>;
#[doc = "Register `DPDS1` writer"]
pub type W = crate::W<DPDS1_SPEC>;
#[doc = "Field `PGDRV` reader - Driver Strength Port G"]
pub type PGDRV_R = crate::FieldReader<PGDRV_A>;
#[doc = "Driver Strength Port G\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGDRV_A {
    #[doc = "0: 2 mA"]
    PAD_IO_2MA = 0,
    #[doc = "1: 4 mA"]
    PAD_IO_4MA = 1,
    #[doc = "2: 6 mA"]
    PAD_IO_6MA = 2,
    #[doc = "3: 8 mA"]
    PAD_IO_8MA = 3,
}
impl From<PGDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PGDRV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PGDRV_A {
    type Ux = u8;
}
impl PGDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PGDRV_A {
        match self.bits {
            0 => PGDRV_A::PAD_IO_2MA,
            1 => PGDRV_A::PAD_IO_4MA,
            2 => PGDRV_A::PAD_IO_6MA,
            3 => PGDRV_A::PAD_IO_8MA,
            _ => unreachable!(),
        }
    }
    #[doc = "2 mA"]
    #[inline(always)]
    pub fn is_pad_io_2ma(&self) -> bool {
        *self == PGDRV_A::PAD_IO_2MA
    }
    #[doc = "4 mA"]
    #[inline(always)]
    pub fn is_pad_io_4ma(&self) -> bool {
        *self == PGDRV_A::PAD_IO_4MA
    }
    #[doc = "6 mA"]
    #[inline(always)]
    pub fn is_pad_io_6ma(&self) -> bool {
        *self == PGDRV_A::PAD_IO_6MA
    }
    #[doc = "8 mA"]
    #[inline(always)]
    pub fn is_pad_io_8ma(&self) -> bool {
        *self == PGDRV_A::PAD_IO_8MA
    }
}
#[doc = "Field `PGDRV` writer - Driver Strength Port G"]
pub type PGDRV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PGDRV_A>;
impl<'a, REG> PGDRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2 mA"]
    #[inline(always)]
    pub fn pad_io_2ma(self) -> &'a mut crate::W<REG> {
        self.variant(PGDRV_A::PAD_IO_2MA)
    }
    #[doc = "4 mA"]
    #[inline(always)]
    pub fn pad_io_4ma(self) -> &'a mut crate::W<REG> {
        self.variant(PGDRV_A::PAD_IO_4MA)
    }
    #[doc = "6 mA"]
    #[inline(always)]
    pub fn pad_io_6ma(self) -> &'a mut crate::W<REG> {
        self.variant(PGDRV_A::PAD_IO_6MA)
    }
    #[doc = "8 mA"]
    #[inline(always)]
    pub fn pad_io_8ma(self) -> &'a mut crate::W<REG> {
        self.variant(PGDRV_A::PAD_IO_8MA)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Driver Strength Port G"]
    #[inline(always)]
    pub fn pgdrv(&self) -> PGDRV_R {
        PGDRV_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits >> 2) & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Driver Strength Port G"]
    #[inline(always)]
    #[must_use]
    pub fn pgdrv(&mut self) -> PGDRV_W<DPDS1_SPEC> {
        PGDRV_W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<DPDS1_SPEC> {
        RES_W::new(self, 2)
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
#[doc = "Port Driver Strength Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpds1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpds1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPDS1_SPEC;
impl crate::RegisterSpec for DPDS1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dpds1::R`](R) reader structure"]
impl crate::Readable for DPDS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpds1::W`](W) writer structure"]
impl crate::Writable for DPDS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPDS1 to value 0"]
impl crate::Resettable for DPDS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
