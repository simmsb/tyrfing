#[doc = "Register `LOCKBIT` reader"]
pub type R = crate::R<LOCKBIT_SPEC>;
#[doc = "Register `LOCKBIT` writer"]
pub type W = crate::W<LOCKBIT_SPEC>;
#[doc = "Field `LB` reader - Memory Lock"]
pub type LB_R = crate::FieldReader<LB_A>;
#[doc = "Memory Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LB_A {
    #[doc = "0: Further programming and verification disabled"]
    PROG_VER_DISABLED = 0,
    #[doc = "2: Further programming disabled"]
    PROG_DISABLED = 2,
    #[doc = "3: No memory lock features enabled"]
    NO_LOCK = 3,
}
impl From<LB_A> for u8 {
    #[inline(always)]
    fn from(variant: LB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LB_A {
    type Ux = u8;
}
impl LB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LB_A> {
        match self.bits {
            0 => Some(LB_A::PROG_VER_DISABLED),
            2 => Some(LB_A::PROG_DISABLED),
            3 => Some(LB_A::NO_LOCK),
            _ => None,
        }
    }
    #[doc = "Further programming and verification disabled"]
    #[inline(always)]
    pub fn is_prog_ver_disabled(&self) -> bool {
        *self == LB_A::PROG_VER_DISABLED
    }
    #[doc = "Further programming disabled"]
    #[inline(always)]
    pub fn is_prog_disabled(&self) -> bool {
        *self == LB_A::PROG_DISABLED
    }
    #[doc = "No memory lock features enabled"]
    #[inline(always)]
    pub fn is_no_lock(&self) -> bool {
        *self == LB_A::NO_LOCK
    }
}
#[doc = "Field `LB` writer - Memory Lock"]
pub type LB_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LB_A>;
impl<'a, REG> LB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Further programming and verification disabled"]
    #[inline(always)]
    pub fn prog_ver_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LB_A::PROG_VER_DISABLED)
    }
    #[doc = "Further programming disabled"]
    #[inline(always)]
    pub fn prog_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LB_A::PROG_DISABLED)
    }
    #[doc = "No memory lock features enabled"]
    #[inline(always)]
    pub fn no_lock(self) -> &'a mut crate::W<REG> {
        self.variant(LB_A::NO_LOCK)
    }
}
#[doc = "Field `BLB0` reader - Boot Loader Protection Mode"]
pub type BLB0_R = crate::FieldReader<BLB0_A>;
#[doc = "Boot Loader Protection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLB0_A {
    #[doc = "0: LPM and SPM prohibited in Application Section"]
    LPM_SPM_DISABLE = 0,
    #[doc = "1: LPM prohibited in Application Section"]
    LPM_DISABLE = 1,
    #[doc = "2: SPM prohibited in Application Section"]
    SPM_DISABLE = 2,
    #[doc = "3: No lock on SPM and LPM in Application Section"]
    NO_LOCK = 3,
}
impl From<BLB0_A> for u8 {
    #[inline(always)]
    fn from(variant: BLB0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BLB0_A {
    type Ux = u8;
}
impl BLB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BLB0_A {
        match self.bits {
            0 => BLB0_A::LPM_SPM_DISABLE,
            1 => BLB0_A::LPM_DISABLE,
            2 => BLB0_A::SPM_DISABLE,
            3 => BLB0_A::NO_LOCK,
            _ => unreachable!(),
        }
    }
    #[doc = "LPM and SPM prohibited in Application Section"]
    #[inline(always)]
    pub fn is_lpm_spm_disable(&self) -> bool {
        *self == BLB0_A::LPM_SPM_DISABLE
    }
    #[doc = "LPM prohibited in Application Section"]
    #[inline(always)]
    pub fn is_lpm_disable(&self) -> bool {
        *self == BLB0_A::LPM_DISABLE
    }
    #[doc = "SPM prohibited in Application Section"]
    #[inline(always)]
    pub fn is_spm_disable(&self) -> bool {
        *self == BLB0_A::SPM_DISABLE
    }
    #[doc = "No lock on SPM and LPM in Application Section"]
    #[inline(always)]
    pub fn is_no_lock(&self) -> bool {
        *self == BLB0_A::NO_LOCK
    }
}
#[doc = "Field `BLB0` writer - Boot Loader Protection Mode"]
pub type BLB0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BLB0_A>;
impl<'a, REG> BLB0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPM and SPM prohibited in Application Section"]
    #[inline(always)]
    pub fn lpm_spm_disable(self) -> &'a mut crate::W<REG> {
        self.variant(BLB0_A::LPM_SPM_DISABLE)
    }
    #[doc = "LPM prohibited in Application Section"]
    #[inline(always)]
    pub fn lpm_disable(self) -> &'a mut crate::W<REG> {
        self.variant(BLB0_A::LPM_DISABLE)
    }
    #[doc = "SPM prohibited in Application Section"]
    #[inline(always)]
    pub fn spm_disable(self) -> &'a mut crate::W<REG> {
        self.variant(BLB0_A::SPM_DISABLE)
    }
    #[doc = "No lock on SPM and LPM in Application Section"]
    #[inline(always)]
    pub fn no_lock(self) -> &'a mut crate::W<REG> {
        self.variant(BLB0_A::NO_LOCK)
    }
}
#[doc = "Field `BLB1` reader - Boot Loader Protection Mode"]
pub type BLB1_R = crate::FieldReader<BLB1_A>;
#[doc = "Boot Loader Protection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLB1_A {
    #[doc = "0: LPM and SPM prohibited in Boot Section"]
    LPM_SPM_DISABLE = 0,
    #[doc = "1: LPM prohibited in Boot Section"]
    LPM_DISABLE = 1,
    #[doc = "2: SPM prohibited in Boot Section"]
    SPM_DISABLE = 2,
    #[doc = "3: No lock on SPM and LPM in Boot Section"]
    NO_LOCK = 3,
}
impl From<BLB1_A> for u8 {
    #[inline(always)]
    fn from(variant: BLB1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BLB1_A {
    type Ux = u8;
}
impl BLB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BLB1_A {
        match self.bits {
            0 => BLB1_A::LPM_SPM_DISABLE,
            1 => BLB1_A::LPM_DISABLE,
            2 => BLB1_A::SPM_DISABLE,
            3 => BLB1_A::NO_LOCK,
            _ => unreachable!(),
        }
    }
    #[doc = "LPM and SPM prohibited in Boot Section"]
    #[inline(always)]
    pub fn is_lpm_spm_disable(&self) -> bool {
        *self == BLB1_A::LPM_SPM_DISABLE
    }
    #[doc = "LPM prohibited in Boot Section"]
    #[inline(always)]
    pub fn is_lpm_disable(&self) -> bool {
        *self == BLB1_A::LPM_DISABLE
    }
    #[doc = "SPM prohibited in Boot Section"]
    #[inline(always)]
    pub fn is_spm_disable(&self) -> bool {
        *self == BLB1_A::SPM_DISABLE
    }
    #[doc = "No lock on SPM and LPM in Boot Section"]
    #[inline(always)]
    pub fn is_no_lock(&self) -> bool {
        *self == BLB1_A::NO_LOCK
    }
}
#[doc = "Field `BLB1` writer - Boot Loader Protection Mode"]
pub type BLB1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, BLB1_A>;
impl<'a, REG> BLB1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPM and SPM prohibited in Boot Section"]
    #[inline(always)]
    pub fn lpm_spm_disable(self) -> &'a mut crate::W<REG> {
        self.variant(BLB1_A::LPM_SPM_DISABLE)
    }
    #[doc = "LPM prohibited in Boot Section"]
    #[inline(always)]
    pub fn lpm_disable(self) -> &'a mut crate::W<REG> {
        self.variant(BLB1_A::LPM_DISABLE)
    }
    #[doc = "SPM prohibited in Boot Section"]
    #[inline(always)]
    pub fn spm_disable(self) -> &'a mut crate::W<REG> {
        self.variant(BLB1_A::SPM_DISABLE)
    }
    #[doc = "No lock on SPM and LPM in Boot Section"]
    #[inline(always)]
    pub fn no_lock(self) -> &'a mut crate::W<REG> {
        self.variant(BLB1_A::NO_LOCK)
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory Lock"]
    #[inline(always)]
    pub fn lb(&self) -> LB_R {
        LB_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Boot Loader Protection Mode"]
    #[inline(always)]
    pub fn blb0(&self) -> BLB0_R {
        BLB0_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Boot Loader Protection Mode"]
    #[inline(always)]
    pub fn blb1(&self) -> BLB1_R {
        BLB1_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lb(&mut self) -> LB_W<LOCKBIT_SPEC> {
        LB_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Boot Loader Protection Mode"]
    #[inline(always)]
    #[must_use]
    pub fn blb0(&mut self) -> BLB0_W<LOCKBIT_SPEC> {
        BLB0_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Boot Loader Protection Mode"]
    #[inline(always)]
    #[must_use]
    pub fn blb1(&mut self) -> BLB1_W<LOCKBIT_SPEC> {
        BLB1_W::new(self, 4)
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
#[doc = "No Description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lockbit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lockbit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCKBIT_SPEC;
impl crate::RegisterSpec for LOCKBIT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lockbit::R`](R) reader structure"]
impl crate::Readable for LOCKBIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lockbit::W`](W) writer structure"]
impl crate::Writable for LOCKBIT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCKBIT to value 0"]
impl crate::Resettable for LOCKBIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
