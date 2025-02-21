#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceBlockBackup<'a, P0>(pipe: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FhServiceBlockBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::core::HRESULT;
    }
    FhServiceBlockBackup(pipe.into()).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceClosePipe<'a, P0>(pipe: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FhServiceClosePipe(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::core::HRESULT;
    }
    FhServiceClosePipe(pipe.into()).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn FhServiceOpenPipe<'a, P0>(startserviceifstopped: P0) -> ::windows::core::Result<super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FhServiceOpenPipe(startserviceifstopped: super::super::Foundation::BOOL, pipe: *mut super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    FhServiceOpenPipe(startserviceifstopped.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceReloadConfiguration<'a, P0>(pipe: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FhServiceReloadConfiguration(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::core::HRESULT;
    }
    FhServiceReloadConfiguration(pipe.into()).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn FhServiceStartBackup<'a, P0, P1>(pipe: P0, lowpriorityio: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FhServiceStartBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE, lowpriorityio: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    FhServiceStartBackup(pipe.into(), lowpriorityio.into()).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn FhServiceStopBackup<'a, P0, P1>(pipe: P0, stoptracking: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FhServiceStopBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE, stoptracking: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    FhServiceStopBackup(pipe.into(), stoptracking.into()).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`, `\"Win32_System_WindowsProgramming\"`*"]
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceUnblockBackup<'a, P0>(pipe: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FhServiceUnblockBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::core::HRESULT;
    }
    FhServiceUnblockBackup(pipe.into()).ok()
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
#[repr(transparent)]
pub struct IFhConfigMgr(::windows::core::IUnknown);
impl IFhConfigMgr {
    pub unsafe fn LoadConfiguration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).LoadConfiguration)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDefaultConfiguration<'a, P0>(&self, overwriteifexists: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).CreateDefaultConfiguration)(::windows::core::Vtable::as_raw(self), overwriteifexists.into()).ok()
    }
    pub unsafe fn SaveConfiguration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SaveConfiguration)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddRemoveExcludeRule<'a, P0>(&self, add: P0, category: FH_PROTECTED_ITEM_CATEGORY, item: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).AddRemoveExcludeRule)(::windows::core::Vtable::as_raw(self), add.into(), category, ::core::mem::transmute_copy(item)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIncludeExcludeRules<'a, P0>(&self, include: P0, category: FH_PROTECTED_ITEM_CATEGORY) -> ::windows::core::Result<IFhScopeIterator>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIncludeExcludeRules)(::windows::core::Vtable::as_raw(self), include.into(), category, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFhScopeIterator>(result__)
    }
    pub unsafe fn GetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLocalPolicy)(::windows::core::Vtable::as_raw(self), localpolicytype, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn SetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLocalPolicy)(::windows::core::Vtable::as_raw(self), localpolicytype, policyvalue).ok()
    }
    pub unsafe fn GetBackupStatus(&self) -> ::windows::core::Result<FH_BACKUP_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBackupStatus)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FH_BACKUP_STATUS>(result__)
    }
    pub unsafe fn SetBackupStatus(&self, backupstatus: FH_BACKUP_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBackupStatus)(::windows::core::Vtable::as_raw(self), backupstatus).ok()
    }
    pub unsafe fn GetDefaultTarget(&self) -> ::windows::core::Result<IFhTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDefaultTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFhTarget>(result__)
    }
    pub unsafe fn ValidateTarget(&self, targeturl: &::windows::core::BSTR) -> ::windows::core::Result<FH_DEVICE_VALIDATION_RESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ValidateTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(targeturl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FH_DEVICE_VALIDATION_RESULT>(result__)
    }
    pub unsafe fn ProvisionAndSetNewTarget(&self, targeturl: &::windows::core::BSTR, targetname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ProvisionAndSetNewTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(targeturl), ::core::mem::transmute_copy(targetname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeDefaultTargetRecommendation<'a, P0>(&self, recommend: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).ChangeDefaultTargetRecommendation)(::windows::core::Vtable::as_raw(self), recommend.into()).ok()
    }
    pub unsafe fn QueryProtectionStatus(&self, protectionstate: *mut u32, protecteduntiltime: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).QueryProtectionStatus)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(protectionstate), ::core::mem::transmute(protecteduntiltime)).ok()
    }
}
::windows::core::interface_hierarchy!(IFhConfigMgr, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFhConfigMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFhConfigMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFhConfigMgr {}
impl ::core::fmt::Debug for IFhConfigMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFhConfigMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFhConfigMgr {
    type Vtable = IFhConfigMgr_Vtbl;
}
unsafe impl ::windows::core::Interface for IFhConfigMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a5fea5b_bf8f_4ee5_b8c3_44d8a0d7331c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhConfigMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub LoadConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateDefaultConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateDefaultConfiguration: usize,
    pub SaveConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddRemoveExcludeRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, add: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddRemoveExcludeRule: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIncludeExcludeRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, include: super::super::Foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, iterator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIncludeExcludeRules: usize,
    pub GetLocalPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: *mut u64) -> ::windows::core::HRESULT,
    pub SetLocalPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows::core::HRESULT,
    pub GetBackupStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, backupstatus: *mut FH_BACKUP_STATUS) -> ::windows::core::HRESULT,
    pub SetBackupStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, backupstatus: FH_BACKUP_STATUS) -> ::windows::core::HRESULT,
    pub GetDefaultTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, defaulttarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ValidateTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows::core::HRESULT,
    pub ProvisionAndSetNewTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, targetname: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangeDefaultTargetRecommendation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recommend: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangeDefaultTargetRecommendation: usize,
    pub QueryProtectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectionstate: *mut u32, protecteduntiltime: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
#[repr(transparent)]
pub struct IFhReassociation(::windows::core::IUnknown);
impl IFhReassociation {
    pub unsafe fn ValidateTarget(&self, targeturl: &::windows::core::BSTR) -> ::windows::core::Result<FH_DEVICE_VALIDATION_RESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ValidateTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(targeturl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FH_DEVICE_VALIDATION_RESULT>(result__)
    }
    pub unsafe fn ScanTargetForConfigurations(&self, targeturl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ScanTargetForConfigurations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(targeturl)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConfigurationDetails(&self, index: u32, username: *mut ::windows::core::BSTR, pcname: *mut ::windows::core::BSTR, backuptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetConfigurationDetails)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute(username), ::core::mem::transmute(pcname), ::core::mem::transmute(backuptime)).ok()
    }
    pub unsafe fn SelectConfiguration(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SelectConfiguration)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PerformReassociation<'a, P0>(&self, overwriteifexists: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).PerformReassociation)(::windows::core::Vtable::as_raw(self), overwriteifexists.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IFhReassociation, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFhReassociation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFhReassociation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFhReassociation {}
impl ::core::fmt::Debug for IFhReassociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFhReassociation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFhReassociation {
    type Vtable = IFhReassociation_Vtbl;
}
unsafe impl ::windows::core::Interface for IFhReassociation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6544a28a_f68d_47ac_91ef_16b2b36aa3ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhReassociation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ValidateTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<::windows::core::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows::core::HRESULT,
    pub ScanTargetForConfigurations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetConfigurationDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, username: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>, pcname: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>, backuptime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetConfigurationDetails: usize,
    pub SelectConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PerformReassociation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwriteifexists: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PerformReassociation: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
#[repr(transparent)]
pub struct IFhScopeIterator(::windows::core::IUnknown);
impl IFhScopeIterator {
    pub unsafe fn MoveToNextItem(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).MoveToNextItem)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetItem(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetItem)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
}
::windows::core::interface_hierarchy!(IFhScopeIterator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFhScopeIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFhScopeIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFhScopeIterator {}
impl ::core::fmt::Debug for IFhScopeIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFhScopeIterator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFhScopeIterator {
    type Vtable = IFhScopeIterator_Vtbl;
}
unsafe impl ::windows::core::Interface for IFhScopeIterator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3197abce_532a_44c6_8615_f3666566a720);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhScopeIterator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub MoveToNextItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
#[repr(transparent)]
pub struct IFhTarget(::windows::core::IUnknown);
impl IFhTarget {
    pub unsafe fn GetStringProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStringProperty)(::windows::core::Vtable::as_raw(self), propertytype, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::BSTR>(result__)
    }
    pub unsafe fn GetNumericalProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNumericalProperty)(::windows::core::Vtable::as_raw(self), propertytype, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
}
::windows::core::interface_hierarchy!(IFhTarget, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFhTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFhTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFhTarget {}
impl ::core::fmt::Debug for IFhTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFhTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFhTarget {
    type Vtable = IFhTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for IFhTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd87965fd_2bad_4657_bd3b_9567eb300ced);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhTarget_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStringProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetNumericalProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_CONFIGURATION_PREVIOUSLY_LOADED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220731i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_CONFIG_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220734i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_CONFIG_FILE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220735i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_CORRUPT_CONFIG_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220736i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_INVALID_REHYDRATION_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220726i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_LEGACY_BACKUP_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220715i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_LEGACY_BACKUP_USER_EXCLUDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220716i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_LEGACY_TARGET_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220718i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_LEGACY_TARGET_VALIDATION_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220717i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_NO_VALID_CONFIGURATION_LOADED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220733i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_RECOMMENDATION_CHANGE_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220720i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_TARGET_CANNOT_BE_USED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220727i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_TARGET_NOT_CONFIGURED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220729i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_TARGET_NOT_CONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220732i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_TARGET_NOT_ENOUGH_FREE_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220728i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_TARGET_REHYDRATED_ELSEWHERE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220719i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHCFG_E_TARGET_VERIFICATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220730i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHSVC_E_BACKUP_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219968i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHSVC_E_CONFIG_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219966i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHSVC_E_CONFIG_DISABLED_GP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219965i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHSVC_E_CONFIG_REHYDRATING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219963i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHSVC_E_FATAL_CONFIG_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219964i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FHSVC_E_NOT_CONFIGURED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219967i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_BACKUP_NOT_SUPPORTED: u32 = 2064u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_DISABLED_BY_GP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_FATAL_CONFIG_ERROR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_MIGRATING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_NOT_TRACKED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_NO_ERROR: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_OFF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_REHYDRATING: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_RUNNING: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_STAGING_FULL: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_TARGET_ABSENT: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_TARGET_ACCESS_DENIED: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_TARGET_FS_LIMITATION: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_TARGET_FULL: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_TARGET_FULL_RETENTION_MAX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_TARGET_LOW_SPACE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_TARGET_LOW_SPACE_RETENTION_MAX: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_TARGET_VOLUME_DIRTY: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATE_TOO_MUCH_BEHIND: u32 = 240u32;
pub const FhConfigMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed43bb3c_09e9_498a_9df6_2177244c6db4);
pub const FhReassociation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d728e35_16fa_4320_9e8b_bfd7100a8846);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FH_BACKUP_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATUS_DISABLED: FH_BACKUP_STATUS = FH_BACKUP_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATUS_DISABLED_BY_GP: FH_BACKUP_STATUS = FH_BACKUP_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATUS_ENABLED: FH_BACKUP_STATUS = FH_BACKUP_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_STATUS_REHYDRATING: FH_BACKUP_STATUS = FH_BACKUP_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const MAX_BACKUP_STATUS: FH_BACKUP_STATUS = FH_BACKUP_STATUS(4i32);
impl ::core::marker::Copy for FH_BACKUP_STATUS {}
impl ::core::clone::Clone for FH_BACKUP_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_BACKUP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FH_BACKUP_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_BACKUP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_BACKUP_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FH_DEVICE_VALIDATION_RESULT(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_ACCESS_DENIED: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_INVALID_DRIVE_TYPE: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_READ_ONLY_PERMISSION: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_CURRENT_DEFAULT: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_NAMESPACE_EXISTS: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_TARGET_PART_OF_LIBRARY: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_VALID_TARGET: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const MAX_VALIDATION_RESULT: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(7i32);
impl ::core::marker::Copy for FH_DEVICE_VALIDATION_RESULT {}
impl ::core::clone::Clone for FH_DEVICE_VALIDATION_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_DEVICE_VALIDATION_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FH_DEVICE_VALIDATION_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_DEVICE_VALIDATION_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_DEVICE_VALIDATION_RESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FH_LOCAL_POLICY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_FREQUENCY: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_RETENTION_TYPE: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_RETENTION_AGE: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const MAX_LOCAL_POLICY: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(3i32);
impl ::core::marker::Copy for FH_LOCAL_POLICY_TYPE {}
impl ::core::clone::Clone for FH_LOCAL_POLICY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_LOCAL_POLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FH_LOCAL_POLICY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_LOCAL_POLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_LOCAL_POLICY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FH_PROTECTED_ITEM_CATEGORY(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_FOLDER: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_LIBRARY: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const MAX_PROTECTED_ITEM_CATEGORY: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(2i32);
impl ::core::marker::Copy for FH_PROTECTED_ITEM_CATEGORY {}
impl ::core::clone::Clone for FH_PROTECTED_ITEM_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_PROTECTED_ITEM_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FH_PROTECTED_ITEM_CATEGORY {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_PROTECTED_ITEM_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_PROTECTED_ITEM_CATEGORY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FH_RETENTION_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_RETENTION_DISABLED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_RETENTION_UNLIMITED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_RETENTION_AGE_BASED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const MAX_RETENTION_TYPE: FH_RETENTION_TYPES = FH_RETENTION_TYPES(3i32);
impl ::core::marker::Copy for FH_RETENTION_TYPES {}
impl ::core::clone::Clone for FH_RETENTION_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_RETENTION_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FH_RETENTION_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_RETENTION_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_RETENTION_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FH_TARGET_DRIVE_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_DRIVE_UNKNOWN: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_DRIVE_REMOVABLE: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_DRIVE_FIXED: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_DRIVE_REMOTE: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(4i32);
impl ::core::marker::Copy for FH_TARGET_DRIVE_TYPES {}
impl ::core::clone::Clone for FH_TARGET_DRIVE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_TARGET_DRIVE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FH_TARGET_DRIVE_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_TARGET_DRIVE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_TARGET_DRIVE_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FH_TARGET_PROPERTY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_TARGET_NAME: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_TARGET_URL: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const FH_TARGET_DRIVE_TYPE: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const MAX_TARGET_PROPERTY: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(3i32);
impl ::core::marker::Copy for FH_TARGET_PROPERTY_TYPE {}
impl ::core::clone::Clone for FH_TARGET_PROPERTY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_TARGET_PROPERTY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FH_TARGET_PROPERTY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_TARGET_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_TARGET_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FhBackupStopReason(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const BackupInvalidStopReason: FhBackupStopReason = FhBackupStopReason(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const BackupLimitUserBusyMachineOnAC: FhBackupStopReason = FhBackupStopReason(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const BackupLimitUserIdleMachineOnDC: FhBackupStopReason = FhBackupStopReason(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const BackupLimitUserBusyMachineOnDC: FhBackupStopReason = FhBackupStopReason(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileHistory\"`*"]
pub const BackupCancelled: FhBackupStopReason = FhBackupStopReason(4i32);
impl ::core::marker::Copy for FhBackupStopReason {}
impl ::core::clone::Clone for FhBackupStopReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FhBackupStopReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FhBackupStopReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for FhBackupStopReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FhBackupStopReason").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
