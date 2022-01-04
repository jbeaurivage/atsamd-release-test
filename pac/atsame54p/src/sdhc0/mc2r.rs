#[doc = "Register `MC2R` writer"]
pub struct W(crate::W<MC2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MC2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MC2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MC2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRESP` writer - e.MMC Abort Wait IRQ"]
pub struct SRESP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRESP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `ABOOT` writer - e.MMC Abort Boot"]
pub struct ABOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> ABOOT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - e.MMC Abort Wait IRQ"]
    #[inline(always)]
    pub fn sresp(&mut self) -> SRESP_W {
        SRESP_W { w: self }
    }
    #[doc = "Bit 1 - e.MMC Abort Boot"]
    #[inline(always)]
    pub fn aboot(&mut self) -> ABOOT_W {
        ABOOT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Control 2\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mc2r](index.html) module"]
pub struct MC2R_SPEC;
impl crate::RegisterSpec for MC2R_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [mc2r::W](W) writer structure"]
impl crate::Writable for MC2R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MC2R to value 0"]
impl crate::Resettable for MC2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
