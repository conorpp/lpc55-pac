#[doc = "Reader of register ACTIVATION_CODE[%s]"]
pub type R = crate::R<u32, super::ACTIVATION_CODE>;
#[doc = "Writer for register ACTIVATION_CODE[%s]"]
pub type W = crate::W<u32, super::ACTIVATION_CODE>;
#[doc = "Register ACTIVATION_CODE[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::ACTIVATION_CODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIELD`"]
pub type FIELD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FIELD`"]
pub struct FIELD_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FIELD_W {
        FIELD_W { w: self }
    }
}
