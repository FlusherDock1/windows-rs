#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
#[repr(transparent)]
pub struct IOplockBreakingHandler(::windows::core::IUnknown);
impl IOplockBreakingHandler {
    pub unsafe fn OplockBreaking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OplockBreaking)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IOplockBreakingHandler, ::windows::core::IUnknown);
impl ::core::clone::Clone for IOplockBreakingHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOplockBreakingHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOplockBreakingHandler {}
impl ::core::fmt::Debug for IOplockBreakingHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOplockBreakingHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IOplockBreakingHandler {
    type Vtable = IOplockBreakingHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for IOplockBreakingHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x826abe3d_3acd_47d3_84f2_88aaedcf6304);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockBreakingHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OplockBreaking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
#[repr(transparent)]
pub struct IRandomAccessStreamFileAccessMode(::windows::core::IUnknown);
impl IRandomAccessStreamFileAccessMode {
    pub unsafe fn GetMode(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
::windows::core::interface_hierarchy!(IRandomAccessStreamFileAccessMode, ::windows::core::IUnknown);
impl ::core::clone::Clone for IRandomAccessStreamFileAccessMode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRandomAccessStreamFileAccessMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRandomAccessStreamFileAccessMode {}
impl ::core::fmt::Debug for IRandomAccessStreamFileAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRandomAccessStreamFileAccessMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IRandomAccessStreamFileAccessMode {
    type Vtable = IRandomAccessStreamFileAccessMode_Vtbl;
}
unsafe impl ::windows::core::Interface for IRandomAccessStreamFileAccessMode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x332e5848_2e15_458e_85c4_c911c0c3d6f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamFileAccessMode_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileaccessmode: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
#[repr(transparent)]
pub struct IStorageFolderHandleAccess(::windows::core::IUnknown);
impl IStorageFolderHandleAccess {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, P0, P1>(&self, filename: P0, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: P1) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IOplockBreakingHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Create)(::windows::core::Vtable::as_raw(self), filename.into(), creationoptions, accessoptions, sharingoptions, options, oplockbreakinghandler.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::HANDLE>(result__)
    }
}
::windows::core::interface_hierarchy!(IStorageFolderHandleAccess, ::windows::core::IUnknown);
impl ::core::clone::Clone for IStorageFolderHandleAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageFolderHandleAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageFolderHandleAccess {}
impl ::core::fmt::Debug for IStorageFolderHandleAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageFolderHandleAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IStorageFolderHandleAccess {
    type Vtable = IStorageFolderHandleAccess_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageFolderHandleAccess {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf19938f_5462_48a0_be65_d2a3271a08d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderHandleAccess_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: *mut ::core::ffi::c_void, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
#[repr(transparent)]
pub struct IStorageItemHandleAccess(::windows::core::IUnknown);
impl IStorageItemHandleAccess {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<'a, P0>(&self, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: P0) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IOplockBreakingHandler>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Create)(::windows::core::Vtable::as_raw(self), accessoptions, sharingoptions, options, oplockbreakinghandler.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::HANDLE>(result__)
    }
}
::windows::core::interface_hierarchy!(IStorageItemHandleAccess, ::windows::core::IUnknown);
impl ::core::clone::Clone for IStorageItemHandleAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageItemHandleAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItemHandleAccess {}
impl ::core::fmt::Debug for IStorageItemHandleAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItemHandleAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IStorageItemHandleAccess {
    type Vtable = IStorageItemHandleAccess_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorageItemHandleAccess {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ca296b2_2c25_4d22_b785_b885c8201e6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemHandleAccess_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: *mut ::core::ffi::c_void, interophandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
#[repr(transparent)]
pub struct IUnbufferedFileHandleOplockCallback(::windows::core::IUnknown);
impl IUnbufferedFileHandleOplockCallback {
    pub unsafe fn OnBrokenCallback(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnBrokenCallback)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IUnbufferedFileHandleOplockCallback, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUnbufferedFileHandleOplockCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUnbufferedFileHandleOplockCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUnbufferedFileHandleOplockCallback {}
impl ::core::fmt::Debug for IUnbufferedFileHandleOplockCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUnbufferedFileHandleOplockCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUnbufferedFileHandleOplockCallback {
    type Vtable = IUnbufferedFileHandleOplockCallback_Vtbl;
}
unsafe impl ::windows::core::Interface for IUnbufferedFileHandleOplockCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1019a0e_6243_4329_8497_2e75894d7710);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnbufferedFileHandleOplockCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnBrokenCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
#[repr(transparent)]
pub struct IUnbufferedFileHandleProvider(::windows::core::IUnknown);
impl IUnbufferedFileHandleProvider {
    pub unsafe fn OpenUnbufferedFileHandle<'a, P0>(&self, oplockbreakcallback: P0) -> ::windows::core::Result<usize>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IUnbufferedFileHandleOplockCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenUnbufferedFileHandle)(::windows::core::Vtable::as_raw(self), oplockbreakcallback.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
    }
    pub unsafe fn CloseUnbufferedFileHandle(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CloseUnbufferedFileHandle)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IUnbufferedFileHandleProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUnbufferedFileHandleProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUnbufferedFileHandleProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUnbufferedFileHandleProvider {}
impl ::core::fmt::Debug for IUnbufferedFileHandleProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUnbufferedFileHandleProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUnbufferedFileHandleProvider {
    type Vtable = IUnbufferedFileHandleProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IUnbufferedFileHandleProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa65c9109_42ab_4b94_a7b1_dd2e4e68515e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnbufferedFileHandleProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OpenUnbufferedFileHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oplockbreakcallback: *mut ::core::ffi::c_void, filehandle: *mut usize) -> ::windows::core::HRESULT,
    pub CloseUnbufferedFileHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HANDLE_ACCESS_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HAO_NONE: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HAO_READ_ATTRIBUTES: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(128u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HAO_READ: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(1179785u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HAO_WRITE: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(1179926u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HAO_DELETE: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(65536u32);
impl ::core::marker::Copy for HANDLE_ACCESS_OPTIONS {}
impl ::core::clone::Clone for HANDLE_ACCESS_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HANDLE_ACCESS_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HANDLE_ACCESS_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HANDLE_ACCESS_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HANDLE_ACCESS_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HANDLE_ACCESS_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HANDLE_ACCESS_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HANDLE_ACCESS_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HANDLE_ACCESS_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HANDLE_ACCESS_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HANDLE_CREATION_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HCO_CREATE_NEW: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HCO_CREATE_ALWAYS: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HCO_OPEN_EXISTING: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HCO_OPEN_ALWAYS: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HCO_TRUNCATE_EXISTING: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(5i32);
impl ::core::marker::Copy for HANDLE_CREATION_OPTIONS {}
impl ::core::clone::Clone for HANDLE_CREATION_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HANDLE_CREATION_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HANDLE_CREATION_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HANDLE_CREATION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HANDLE_CREATION_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HANDLE_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_NONE: HANDLE_OPTIONS = HANDLE_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_OPEN_REQUIRING_OPLOCK: HANDLE_OPTIONS = HANDLE_OPTIONS(262144u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_DELETE_ON_CLOSE: HANDLE_OPTIONS = HANDLE_OPTIONS(67108864u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_SEQUENTIAL_SCAN: HANDLE_OPTIONS = HANDLE_OPTIONS(134217728u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_RANDOM_ACCESS: HANDLE_OPTIONS = HANDLE_OPTIONS(268435456u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_NO_BUFFERING: HANDLE_OPTIONS = HANDLE_OPTIONS(536870912u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_OVERLAPPED: HANDLE_OPTIONS = HANDLE_OPTIONS(1073741824u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HO_WRITE_THROUGH: HANDLE_OPTIONS = HANDLE_OPTIONS(2147483648u32);
impl ::core::marker::Copy for HANDLE_OPTIONS {}
impl ::core::clone::Clone for HANDLE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HANDLE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HANDLE_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HANDLE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HANDLE_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HANDLE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HANDLE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HANDLE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HANDLE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HANDLE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HANDLE_SHARING_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HSO_SHARE_NONE: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HSO_SHARE_READ: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HSO_SHARE_WRITE: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(2u32);
#[doc = "*Required features: `\"Win32_System_WinRT_Storage\"`*"]
pub const HSO_SHARE_DELETE: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(4u32);
impl ::core::marker::Copy for HANDLE_SHARING_OPTIONS {}
impl ::core::clone::Clone for HANDLE_SHARING_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HANDLE_SHARING_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HANDLE_SHARING_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HANDLE_SHARING_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HANDLE_SHARING_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HANDLE_SHARING_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HANDLE_SHARING_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HANDLE_SHARING_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HANDLE_SHARING_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HANDLE_SHARING_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
