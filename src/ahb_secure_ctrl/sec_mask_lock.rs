#[doc = "Reader of register SEC_MASK_LOCK"]
pub type R = crate::R<u32, super::SEC_MASK_LOCK>;
#[doc = "Writer for register SEC_MASK_LOCK"]
pub type W = crate::W<u32, super::SEC_MASK_LOCK>;
#[doc = "Register SEC_MASK_LOCK `reset()`'s with value 0x0aaa"]
impl crate::ResetValue for super::SEC_MASK_LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0aaa
    }
}
#[doc = "SEC_GPIO_MASK0 register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEC_GPIO_MASK0_LOCK_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<SEC_GPIO_MASK0_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_GPIO_MASK0_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEC_GPIO_MASK0_LOCK`"]
pub type SEC_GPIO_MASK0_LOCK_R = crate::R<u8, SEC_GPIO_MASK0_LOCK_A>;
impl SEC_GPIO_MASK0_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEC_GPIO_MASK0_LOCK_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SEC_GPIO_MASK0_LOCK_A::BLOCKED),
            2 => Val(SEC_GPIO_MASK0_LOCK_A::WRITABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SEC_GPIO_MASK0_LOCK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SEC_GPIO_MASK0_LOCK_A::WRITABLE
    }
}
#[doc = "Write proxy for field `SEC_GPIO_MASK0_LOCK`"]
pub struct SEC_GPIO_MASK0_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_GPIO_MASK0_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_GPIO_MASK0_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK0_LOCK_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK0_LOCK_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "SEC_GPIO_MASK1 register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEC_GPIO_MASK1_LOCK_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<SEC_GPIO_MASK1_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_GPIO_MASK1_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEC_GPIO_MASK1_LOCK`"]
pub type SEC_GPIO_MASK1_LOCK_R = crate::R<u8, SEC_GPIO_MASK1_LOCK_A>;
impl SEC_GPIO_MASK1_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEC_GPIO_MASK1_LOCK_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SEC_GPIO_MASK1_LOCK_A::BLOCKED),
            2 => Val(SEC_GPIO_MASK1_LOCK_A::WRITABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SEC_GPIO_MASK1_LOCK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SEC_GPIO_MASK1_LOCK_A::WRITABLE
    }
}
#[doc = "Write proxy for field `SEC_GPIO_MASK1_LOCK`"]
pub struct SEC_GPIO_MASK1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_GPIO_MASK1_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_GPIO_MASK1_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK1_LOCK_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(SEC_GPIO_MASK1_LOCK_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "SEC_CPU_INT_MASK0 register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEC_CPU1_INT_MASK0_LOCK_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<SEC_CPU1_INT_MASK0_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_CPU1_INT_MASK0_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEC_CPU1_INT_MASK0_LOCK`"]
pub type SEC_CPU1_INT_MASK0_LOCK_R = crate::R<u8, SEC_CPU1_INT_MASK0_LOCK_A>;
impl SEC_CPU1_INT_MASK0_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEC_CPU1_INT_MASK0_LOCK_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SEC_CPU1_INT_MASK0_LOCK_A::BLOCKED),
            2 => Val(SEC_CPU1_INT_MASK0_LOCK_A::WRITABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SEC_CPU1_INT_MASK0_LOCK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SEC_CPU1_INT_MASK0_LOCK_A::WRITABLE
    }
}
#[doc = "Write proxy for field `SEC_CPU1_INT_MASK0_LOCK`"]
pub struct SEC_CPU1_INT_MASK0_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_CPU1_INT_MASK0_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_CPU1_INT_MASK0_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SEC_CPU1_INT_MASK0_LOCK_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(SEC_CPU1_INT_MASK0_LOCK_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "SEC_CPU_INT_MASK1 register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEC_CPU1_INT_MASK1_LOCK_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<SEC_CPU1_INT_MASK1_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_CPU1_INT_MASK1_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEC_CPU1_INT_MASK1_LOCK`"]
pub type SEC_CPU1_INT_MASK1_LOCK_R = crate::R<u8, SEC_CPU1_INT_MASK1_LOCK_A>;
impl SEC_CPU1_INT_MASK1_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEC_CPU1_INT_MASK1_LOCK_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SEC_CPU1_INT_MASK1_LOCK_A::BLOCKED),
            2 => Val(SEC_CPU1_INT_MASK1_LOCK_A::WRITABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SEC_CPU1_INT_MASK1_LOCK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SEC_CPU1_INT_MASK1_LOCK_A::WRITABLE
    }
}
#[doc = "Write proxy for field `SEC_CPU1_INT_MASK1_LOCK`"]
pub struct SEC_CPU1_INT_MASK1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_CPU1_INT_MASK1_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_CPU1_INT_MASK1_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SEC_CPU1_INT_MASK1_LOCK_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(SEC_CPU1_INT_MASK1_LOCK_A::WRITABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SEC_GPIO_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask0_lock(&self) -> SEC_GPIO_MASK0_LOCK_R {
        SEC_GPIO_MASK0_LOCK_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - SEC_GPIO_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask1_lock(&self) -> SEC_GPIO_MASK1_LOCK_R {
        SEC_GPIO_MASK1_LOCK_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - SEC_CPU_INT_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_cpu1_int_mask0_lock(&self) -> SEC_CPU1_INT_MASK0_LOCK_R {
        SEC_CPU1_INT_MASK0_LOCK_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - SEC_CPU_INT_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_cpu1_int_mask1_lock(&self) -> SEC_CPU1_INT_MASK1_LOCK_R {
        SEC_CPU1_INT_MASK1_LOCK_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SEC_GPIO_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask0_lock(&mut self) -> SEC_GPIO_MASK0_LOCK_W {
        SEC_GPIO_MASK0_LOCK_W { w: self }
    }
    #[doc = "Bits 2:3 - SEC_GPIO_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask1_lock(&mut self) -> SEC_GPIO_MASK1_LOCK_W {
        SEC_GPIO_MASK1_LOCK_W { w: self }
    }
    #[doc = "Bits 8:9 - SEC_CPU_INT_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_cpu1_int_mask0_lock(&mut self) -> SEC_CPU1_INT_MASK0_LOCK_W {
        SEC_CPU1_INT_MASK0_LOCK_W { w: self }
    }
    #[doc = "Bits 10:11 - SEC_CPU_INT_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_cpu1_int_mask1_lock(&mut self) -> SEC_CPU1_INT_MASK1_LOCK_W {
        SEC_CPU1_INT_MASK1_LOCK_W { w: self }
    }
}
