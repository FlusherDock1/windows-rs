#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IFunctionDiscovery(::windows::core::IUnknown);
impl IFunctionDiscovery {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInstanceCollection<'a, P0, P1, P2>(&self, pszcategory: P0, pszsubcategory: P1, fincludeallsubcategories: P2) -> ::windows::core::Result<IFunctionInstanceCollection>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInstanceCollection)(::windows::core::Vtable::as_raw(self), pszcategory.into(), pszsubcategory.into(), fincludeallsubcategories.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstanceCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInstance<'a, P0>(&self, pszfunctioninstanceidentity: P0) -> ::windows::core::Result<IFunctionInstance>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetInstance)(::windows::core::Vtable::as_raw(self), pszfunctioninstanceidentity.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateInstanceCollectionQuery<'a, P0, P1, P2, P3>(&self, pszcategory: P0, pszsubcategory: P1, fincludeallsubcategories: P2, pifunctiondiscoverynotification: P3, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut ::core::option::Option<IFunctionInstanceCollectionQuery>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, IFunctionDiscoveryNotification>>,
    {
        (::windows::core::Vtable::vtable(self).CreateInstanceCollectionQuery)(::windows::core::Vtable::as_raw(self), pszcategory.into(), pszsubcategory.into(), fincludeallsubcategories.into(), pifunctiondiscoverynotification.into().abi(), ::core::mem::transmute(pfdqcquerycontext), ::core::mem::transmute(ppifunctioninstancecollectionquery)).ok()
    }
    pub unsafe fn CreateInstanceQuery<'a, P0, P1>(&self, pszfunctioninstanceidentity: P0, pifunctiondiscoverynotification: P1, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::core::option::Option<IFunctionInstanceQuery>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFunctionDiscoveryNotification>>,
    {
        (::windows::core::Vtable::vtable(self).CreateInstanceQuery)(::windows::core::Vtable::as_raw(self), pszfunctioninstanceidentity.into(), pifunctiondiscoverynotification.into().abi(), ::core::mem::transmute(pfdqcquerycontext), ::core::mem::transmute(ppifunctioninstancequery)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddInstance<'a, P0, P1, P2>(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: P0, pszsubcategory: P1, pszcategoryidentity: P2) -> ::windows::core::Result<IFunctionInstance>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AddInstance)(::windows::core::Vtable::as_raw(self), enumsystemvisibility, pszcategory.into(), pszsubcategory.into(), pszcategoryidentity.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
    pub unsafe fn RemoveInstance<'a, P0, P1, P2>(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: P0, pszsubcategory: P1, pszcategoryidentity: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).RemoveInstance)(::windows::core::Vtable::as_raw(self), enumsystemvisibility, pszcategory.into(), pszsubcategory.into(), pszcategoryidentity.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IFunctionDiscovery, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFunctionDiscovery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscovery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscovery {}
impl ::core::fmt::Debug for IFunctionDiscovery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscovery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFunctionDiscovery {
    type Vtable = IFunctionDiscovery_Vtbl;
}
unsafe impl ::windows::core::Interface for IFunctionDiscovery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4df99b70_e148_4432_b004_4c9eeb535a5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscovery_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInstanceCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcategory: ::windows::core::PCWSTR, pszsubcategory: ::windows::core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, ppifunctioninstancecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInstanceCollection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: ::windows::core::PCWSTR, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInstance: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateInstanceCollectionQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcategory: ::windows::core::PCWSTR, pszsubcategory: ::windows::core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: *mut ::core::ffi::c_void, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateInstanceCollectionQuery: usize,
    pub CreateInstanceQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: ::windows::core::PCWSTR, pifunctiondiscoverynotification: *mut ::core::ffi::c_void, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: ::windows::core::PCWSTR, pszsubcategory: ::windows::core::PCWSTR, pszcategoryidentity: ::windows::core::PCWSTR, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddInstance: usize,
    pub RemoveInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: ::windows::core::PCWSTR, pszsubcategory: ::windows::core::PCWSTR, pszcategoryidentity: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IFunctionDiscoveryNotification(::windows::core::IUnknown);
impl IFunctionDiscoveryNotification {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnUpdate<'a, P0>(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionInstance>>,
    {
        (::windows::core::Vtable::vtable(self).OnUpdate)(::windows::core::Vtable::as_raw(self), enumqueryupdateaction, fdqcquerycontext, pifunctioninstance.into().abi()).ok()
    }
    pub unsafe fn OnError<'a, P0>(&self, hr: ::windows::core::HRESULT, fdqcquerycontext: u64, pszprovider: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).OnError)(::windows::core::Vtable::as_raw(self), hr, fdqcquerycontext, pszprovider.into()).ok()
    }
    pub unsafe fn OnEvent<'a, P0>(&self, dweventid: u32, fdqcquerycontext: u64, pszprovider: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).OnEvent)(::windows::core::Vtable::as_raw(self), dweventid, fdqcquerycontext, pszprovider.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IFunctionDiscoveryNotification, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFunctionDiscoveryNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryNotification {}
impl ::core::fmt::Debug for IFunctionDiscoveryNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFunctionDiscoveryNotification {
    type Vtable = IFunctionDiscoveryNotification_Vtbl;
}
unsafe impl ::windows::core::Interface for IFunctionDiscoveryNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f6c1ba8_5330_422e_a368_572b244d3f87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryNotification_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnUpdate: usize,
    pub OnError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, fdqcquerycontext: u64, pszprovider: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub OnEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweventid: u32, fdqcquerycontext: u64, pszprovider: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IFunctionDiscoveryProvider(::windows::core::IUnknown);
impl IFunctionDiscoveryProvider {
    pub unsafe fn Initialize<'a, P0, P1>(&self, pifunctiondiscoveryproviderfactory: P0, pifunctiondiscoverynotification: P1, lciduserdefault: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionDiscoveryProviderFactory>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFunctionDiscoveryNotification>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), pifunctiondiscoveryproviderfactory.into().abi(), pifunctiondiscoverynotification.into().abi(), lciduserdefault, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Query<'a, P0>(&self, pifunctiondiscoveryproviderquery: P0) -> ::windows::core::Result<IFunctionInstanceCollection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionDiscoveryProviderQuery>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Query)(::windows::core::Vtable::as_raw(self), pifunctiondiscoveryproviderquery.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstanceCollection>(result__)
    }
    pub unsafe fn EndQuery(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EndQuery)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancePropertyStoreValidateAccess<'a, P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionInstance>>,
    {
        (::windows::core::Vtable::vtable(self).InstancePropertyStoreValidateAccess)(::windows::core::Vtable::as_raw(self), pifunctioninstance.into().abi(), iproviderinstancecontext, dwstgaccess).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn InstancePropertyStoreOpen<'a, P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionInstance>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InstancePropertyStoreOpen)(::windows::core::Vtable::as_raw(self), pifunctioninstance.into().abi(), iproviderinstancecontext, dwstgaccess, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancePropertyStoreFlush<'a, P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionInstance>>,
    {
        (::windows::core::Vtable::vtable(self).InstancePropertyStoreFlush)(::windows::core::Vtable::as_raw(self), pifunctioninstance.into().abi(), iproviderinstancecontext).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstanceQueryService<'a, P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionInstance>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InstanceQueryService)(::windows::core::Vtable::as_raw(self), pifunctioninstance.into().abi(), iproviderinstancecontext, ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstanceReleased<'a, P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionInstance>>,
    {
        (::windows::core::Vtable::vtable(self).InstanceReleased)(::windows::core::Vtable::as_raw(self), pifunctioninstance.into().abi(), iproviderinstancecontext).ok()
    }
}
::windows::core::interface_hierarchy!(IFunctionDiscoveryProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFunctionDiscoveryProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryProvider {}
impl ::core::fmt::Debug for IFunctionDiscoveryProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFunctionDiscoveryProvider {
    type Vtable = IFunctionDiscoveryProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IFunctionDiscoveryProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcde394f_1478_4813_a402_f6fb10657222);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderfactory: *mut ::core::ffi::c_void, pifunctiondiscoverynotification: *mut ::core::ffi::c_void, lciduserdefault: u32, pdwstgaccesscapabilities: *mut u32) -> ::windows::core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderquery: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InstancePropertyStoreValidateAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstancePropertyStoreValidateAccess: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub InstancePropertyStoreOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, dwstgaccess: u32, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    InstancePropertyStoreOpen: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstancePropertyStoreFlush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstancePropertyStoreFlush: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstanceQueryService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppiunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstanceQueryService: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstanceReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstanceReleased: usize,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IFunctionDiscoveryProviderFactory(::windows::core::IUnknown);
impl IFunctionDiscoveryProviderFactory {
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CreatePropertyStore(&self) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePropertyStore)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn CreateInstance<'a, P0, P1, P2, P3>(&self, pszsubcategory: P0, pszproviderinstanceidentity: P1, iproviderinstancecontext: isize, pipropertystore: P2, pifunctiondiscoveryprovider: P3) -> ::windows::core::Result<IFunctionInstance>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, IFunctionDiscoveryProvider>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateInstance)(::windows::core::Vtable::as_raw(self), pszsubcategory.into(), pszproviderinstanceidentity.into(), iproviderinstancecontext, pipropertystore.into().abi(), pifunctiondiscoveryprovider.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
    pub unsafe fn CreateFunctionInstanceCollection(&self) -> ::windows::core::Result<IFunctionInstanceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateFunctionInstanceCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstanceCollection>(result__)
    }
}
::windows::core::interface_hierarchy!(IFunctionDiscoveryProviderFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFunctionDiscoveryProviderFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryProviderFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryProviderFactory {}
impl ::core::fmt::Debug for IFunctionDiscoveryProviderFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryProviderFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFunctionDiscoveryProviderFactory {
    type Vtable = IFunctionDiscoveryProviderFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IFunctionDiscoveryProviderFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86443ff0_1ad5_4e68_a45a_40c2c329de3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryProviderFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreatePropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreatePropertyStore: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR, pszproviderinstanceidentity: ::windows::core::PCWSTR, iproviderinstancecontext: isize, pipropertystore: *mut ::core::ffi::c_void, pifunctiondiscoveryprovider: *mut ::core::ffi::c_void, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateInstance: usize,
    pub CreateFunctionInstanceCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IFunctionDiscoveryProviderQuery(::windows::core::IUnknown);
impl IFunctionDiscoveryProviderQuery {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInstanceQuery(&self, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).IsInstanceQuery)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pisinstancequery), ::core::mem::transmute(ppszconstraintvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubcategoryQuery(&self, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).IsSubcategoryQuery)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pissubcategoryquery), ::core::mem::transmute(ppszconstraintvalue)).ok()
    }
    pub unsafe fn GetQueryConstraints(&self) -> ::windows::core::Result<IProviderQueryConstraintCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetQueryConstraints)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IProviderQueryConstraintCollection>(result__)
    }
    pub unsafe fn GetPropertyConstraints(&self) -> ::windows::core::Result<IProviderPropertyConstraintCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPropertyConstraints)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IProviderPropertyConstraintCollection>(result__)
    }
}
::windows::core::interface_hierarchy!(IFunctionDiscoveryProviderQuery, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFunctionDiscoveryProviderQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryProviderQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryProviderQuery {}
impl ::core::fmt::Debug for IFunctionDiscoveryProviderQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryProviderQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFunctionDiscoveryProviderQuery {
    type Vtable = IFunctionDiscoveryProviderQuery_Vtbl;
}
unsafe impl ::windows::core::Interface for IFunctionDiscoveryProviderQuery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6876ea98_baec_46db_bc20_75a76e267a3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryProviderQuery_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInstanceQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInstanceQuery: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSubcategoryQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSubcategoryQuery: usize,
    pub GetQueryConstraints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiproviderqueryconstraints: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropertyConstraints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiproviderpropertyconstraints: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IFunctionDiscoveryServiceProvider(::windows::core::IUnknown);
impl IFunctionDiscoveryServiceProvider {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<'a, P0, T>(&self, pifunctioninstance: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionInstance>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), pifunctioninstance.into().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
::windows::core::interface_hierarchy!(IFunctionDiscoveryServiceProvider, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFunctionDiscoveryServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionDiscoveryServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionDiscoveryServiceProvider {}
impl ::core::fmt::Debug for IFunctionDiscoveryServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionDiscoveryServiceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFunctionDiscoveryServiceProvider {
    type Vtable = IFunctionDiscoveryServiceProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IFunctionDiscoveryServiceProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c81ed02_1b04_43f2_a451_69966cbcd1c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionDiscoveryServiceProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFunctionInstance(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFunctionInstance {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueryService)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
    pub unsafe fn GetID(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u16>(result__)
    }
    pub unsafe fn GetProviderInstanceID(&self) -> ::windows::core::Result<*mut u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProviderInstanceID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn OpenPropertyStore(&self, dwstgaccess: super::super::System::Com::STGM) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OpenPropertyStore)(::windows::core::Vtable::as_raw(self), dwstgaccess, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    pub unsafe fn GetCategory(&self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetCategory)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppszcomemcategory), ::core::mem::transmute(ppszcomemsubcategory)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFunctionInstance, ::windows::core::IUnknown, super::super::System::Com::IServiceProvider);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFunctionInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFunctionInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFunctionInstance {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFunctionInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionInstance").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFunctionInstance {
    type Vtable = IFunctionInstance_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFunctionInstance {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33591c10_0bed_4f02_b0ab_1530d5533ee9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstance_Vtbl {
    pub base__: super::super::System::Com::IServiceProvider_Vtbl,
    pub GetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcomemidentity: *mut *mut u16) -> ::windows::core::HRESULT,
    pub GetProviderInstanceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcomemproviderinstanceidentity: *mut *mut u16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub OpenPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstgaccess: super::super::System::Com::STGM, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    OpenPropertyStore: usize,
    pub GetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IFunctionInstanceCollection(::windows::core::IUnknown);
impl IFunctionInstanceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Get<'a, P0>(&self, pszinstanceidentity: P0, pdwindex: *mut u32, ppifunctioninstance: *mut ::core::option::Option<IFunctionInstance>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Get)(::windows::core::Vtable::as_raw(self), pszinstanceidentity.into(), ::core::mem::transmute(pdwindex), ::core::mem::transmute(ppifunctioninstance)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, dwindex: u32) -> ::windows::core::Result<IFunctionInstance> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Item)(::windows::core::Vtable::as_raw(self), dwindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<'a, P0>(&self, pifunctioninstance: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionInstance>>,
    {
        (::windows::core::Vtable::vtable(self).Add)(::windows::core::Vtable::as_raw(self), pifunctioninstance.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Remove(&self, dwindex: u32) -> ::windows::core::Result<IFunctionInstance> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Remove)(::windows::core::Vtable::as_raw(self), dwindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
    pub unsafe fn Delete(&self, dwindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self), dwindex).ok()
    }
    pub unsafe fn DeleteAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteAll)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IFunctionInstanceCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFunctionInstanceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionInstanceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionInstanceCollection {}
impl ::core::fmt::Debug for IFunctionInstanceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionInstanceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFunctionInstanceCollection {
    type Vtable = IFunctionInstanceCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IFunctionInstanceCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0a3d895_855c_42a2_948d_2f97d450ecb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszinstanceidentity: ::windows::core::PCWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Get: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Remove: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IFunctionInstanceCollectionQuery(::windows::core::IUnknown);
impl IFunctionInstanceCollectionQuery {
    pub unsafe fn AddQueryConstraint<'a, P0, P1>(&self, pszconstraintname: P0, pszconstraintvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).AddQueryConstraint)(::windows::core::Vtable::as_raw(self), pszconstraintname.into(), pszconstraintvalue.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn AddPropertyConstraint(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddPropertyConstraint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(pv), enumpropertyconstraint).ok()
    }
    pub unsafe fn Execute(&self) -> ::windows::core::Result<IFunctionInstanceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Execute)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstanceCollection>(result__)
    }
}
::windows::core::interface_hierarchy!(IFunctionInstanceCollectionQuery, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFunctionInstanceCollectionQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionInstanceCollectionQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionInstanceCollectionQuery {}
impl ::core::fmt::Debug for IFunctionInstanceCollectionQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionInstanceCollectionQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFunctionInstanceCollectionQuery {
    type Vtable = IFunctionInstanceCollectionQuery_Vtbl;
}
unsafe impl ::windows::core::Interface for IFunctionInstanceCollectionQuery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57cc6fd2_c09a_4289_bb72_25f04142058e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceCollectionQuery_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddQueryConstraint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszconstraintname: ::windows::core::PCWSTR, pszconstraintvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub AddPropertyConstraint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, enumpropertyconstraint: PropertyConstraint) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    AddPropertyConstraint: usize,
    pub Execute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IFunctionInstanceQuery(::windows::core::IUnknown);
impl IFunctionInstanceQuery {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Execute(&self) -> ::windows::core::Result<IFunctionInstance> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Execute)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
}
::windows::core::interface_hierarchy!(IFunctionInstanceQuery, ::windows::core::IUnknown);
impl ::core::clone::Clone for IFunctionInstanceQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFunctionInstanceQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFunctionInstanceQuery {}
impl ::core::fmt::Debug for IFunctionInstanceQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFunctionInstanceQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IFunctionInstanceQuery {
    type Vtable = IFunctionInstanceQuery_Vtbl;
}
unsafe impl ::windows::core::Interface for IFunctionInstanceQuery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6242bc6b_90ec_4b37_bb46_e229fd84ed95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFunctionInstanceQuery_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Execute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Execute: usize,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IPNPXAssociation(::windows::core::IUnknown);
impl IPNPXAssociation {
    pub unsafe fn Associate<'a, P0>(&self, pszsubcategory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Associate)(::windows::core::Vtable::as_raw(self), pszsubcategory.into()).ok()
    }
    pub unsafe fn Unassociate<'a, P0>(&self, pszsubcategory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Unassociate)(::windows::core::Vtable::as_raw(self), pszsubcategory.into()).ok()
    }
    pub unsafe fn Delete<'a, P0>(&self, pszsubcategory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self), pszsubcategory.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IPNPXAssociation, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPNPXAssociation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPNPXAssociation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPNPXAssociation {}
impl ::core::fmt::Debug for IPNPXAssociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPNPXAssociation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPNPXAssociation {
    type Vtable = IPNPXAssociation_Vtbl;
}
unsafe impl ::windows::core::Interface for IPNPXAssociation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bd7e521_4da6_42d5_81ba_1981b6b94075);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPNPXAssociation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Associate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Unassociate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IPNPXDeviceAssociation(::windows::core::IUnknown);
impl IPNPXDeviceAssociation {
    pub unsafe fn Associate<'a, P0, P1>(&self, pszsubcategory: P0, pifunctiondiscoverynotification: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFunctionDiscoveryNotification>>,
    {
        (::windows::core::Vtable::vtable(self).Associate)(::windows::core::Vtable::as_raw(self), pszsubcategory.into(), pifunctiondiscoverynotification.into().abi()).ok()
    }
    pub unsafe fn Unassociate<'a, P0, P1>(&self, pszsubcategory: P0, pifunctiondiscoverynotification: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFunctionDiscoveryNotification>>,
    {
        (::windows::core::Vtable::vtable(self).Unassociate)(::windows::core::Vtable::as_raw(self), pszsubcategory.into(), pifunctiondiscoverynotification.into().abi()).ok()
    }
    pub unsafe fn Delete<'a, P0, P1>(&self, pszsubcategory: P0, pifunctiondiscoverynotification: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IFunctionDiscoveryNotification>>,
    {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self), pszsubcategory.into(), pifunctiondiscoverynotification.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IPNPXDeviceAssociation, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPNPXDeviceAssociation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPNPXDeviceAssociation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPNPXDeviceAssociation {}
impl ::core::fmt::Debug for IPNPXDeviceAssociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPNPXDeviceAssociation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPNPXDeviceAssociation {
    type Vtable = IPNPXDeviceAssociation_Vtbl;
}
unsafe impl ::windows::core::Interface for IPNPXDeviceAssociation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeed366d0_35b8_4fc5_8d20_7e5bd31f6ded);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPNPXDeviceAssociation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Associate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR, pifunctiondiscoverynotification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unassociate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR, pifunctiondiscoverynotification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR, pifunctiondiscoverynotification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IPropertyStoreCollection(::windows::core::IUnknown);
impl IPropertyStoreCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Get<'a, P0>(&self, pszinstanceidentity: P0, pdwindex: *mut u32, ppipropertystore: *mut ::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).Get)(::windows::core::Vtable::as_raw(self), pszinstanceidentity.into(), ::core::mem::transmute(pdwindex), ::core::mem::transmute(ppipropertystore)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Item(&self, dwindex: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Item)(::windows::core::Vtable::as_raw(self), dwindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Add<'a, P0>(&self, pipropertystore: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    {
        (::windows::core::Vtable::vtable(self).Add)(::windows::core::Vtable::as_raw(self), pipropertystore.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Remove(&self, dwindex: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Remove)(::windows::core::Vtable::as_raw(self), dwindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    pub unsafe fn Delete(&self, dwindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self), dwindex).ok()
    }
    pub unsafe fn DeleteAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteAll)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPropertyStoreCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPropertyStoreCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyStoreCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyStoreCollection {}
impl ::core::fmt::Debug for IPropertyStoreCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyStoreCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPropertyStoreCollection {
    type Vtable = IPropertyStoreCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IPropertyStoreCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd14d9c30_12d2_42d8_bce4_c60c2bb226fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyStoreCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszinstanceidentity: ::windows::core::PCWSTR, pdwindex: *mut u32, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Get: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Item: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipropertystore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Add: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pipropertystore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Remove: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT,
    pub DeleteAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IProviderProperties(::windows::core::IUnknown);
impl IProviderProperties {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCount<'a, P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionInstance>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), pifunctioninstance.into().abi(), iproviderinstancecontext, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetAt<'a, P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize, dwindex: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionInstance>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), pifunctioninstance.into().abi(), iproviderinstancecontext, dwindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue<'a, P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionInstance>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetValue)(::windows::core::Vtable::as_raw(self), pifunctioninstance.into().abi(), iproviderinstancecontext, ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetValue<'a, P0>(&self, pifunctioninstance: P0, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFunctionInstance>>,
    {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), pifunctioninstance.into().abi(), iproviderinstancecontext, ::core::mem::transmute(key), ::core::mem::transmute(ppropvar)).ok()
    }
}
::windows::core::interface_hierarchy!(IProviderProperties, ::windows::core::IUnknown);
impl ::core::clone::Clone for IProviderProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProviderProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderProperties {}
impl ::core::fmt::Debug for IProviderProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProviderProperties {
    type Vtable = IProviderProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IProviderProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf986ea6_3b5f_4c5f_b88a_2f8b20ceef17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderProperties_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetCount: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifunctioninstance: *mut ::core::ffi::c_void, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    SetValue: usize,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IProviderPropertyConstraintCollection(::windows::core::IUnknown);
impl IProviderPropertyConstraintCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Get(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(ppropvar), ::core::mem::transmute(pdwpropertyconstraint)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Item(&self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Item)(::windows::core::Vtable::as_raw(self), dwindex, ::core::mem::transmute(pkey), ::core::mem::transmute(ppropvar), ::core::mem::transmute(pdwpropertyconstraint)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn Next(&self, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pkey), ::core::mem::transmute(ppropvar), ::core::mem::transmute(pdwpropertyconstraint)).ok()
    }
    pub unsafe fn Skip(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IProviderPropertyConstraintCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IProviderPropertyConstraintCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProviderPropertyConstraintCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderPropertyConstraintCollection {}
impl ::core::fmt::Debug for IProviderPropertyConstraintCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderPropertyConstraintCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProviderPropertyConstraintCollection {
    type Vtable = IProviderPropertyConstraintCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IProviderPropertyConstraintCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4fae42f_5778_4a13_8540_b5fd8c1398dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderPropertyConstraintCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Get: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IProviderPublishing(::windows::core::IUnknown);
impl IProviderPublishing {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateInstance<'a, P0, P1>(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: P0, pszproviderinstanceidentity: P1) -> ::windows::core::Result<IFunctionInstance>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateInstance)(::windows::core::Vtable::as_raw(self), enumvisibilityflags, pszsubcategory.into(), pszproviderinstanceidentity.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFunctionInstance>(result__)
    }
    pub unsafe fn RemoveInstance<'a, P0, P1>(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: P0, pszproviderinstanceidentity: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).RemoveInstance)(::windows::core::Vtable::as_raw(self), enumvisibilityflags, pszsubcategory.into(), pszproviderinstanceidentity.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IProviderPublishing, ::windows::core::IUnknown);
impl ::core::clone::Clone for IProviderPublishing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProviderPublishing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderPublishing {}
impl ::core::fmt::Debug for IProviderPublishing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderPublishing").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProviderPublishing {
    type Vtable = IProviderPublishing_Vtbl;
}
unsafe impl ::windows::core::Interface for IProviderPublishing {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd1b9a04_206c_4a05_a0c8_1635a21a2b7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderPublishing_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: ::windows::core::PCWSTR, pszproviderinstanceidentity: ::windows::core::PCWSTR, ppifunctioninstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateInstance: usize,
    pub RemoveInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: ::windows::core::PCWSTR, pszproviderinstanceidentity: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
pub struct IProviderQueryConstraintCollection(::windows::core::IUnknown);
impl IProviderQueryConstraintCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn Get<'a, P0>(&self, pszconstraintname: P0) -> ::windows::core::Result<*mut u16>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Get)(::windows::core::Vtable::as_raw(self), pszconstraintname.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u16>(result__)
    }
    pub unsafe fn Item(&self, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Item)(::windows::core::Vtable::as_raw(self), dwindex, ::core::mem::transmute(ppszconstraintname), ::core::mem::transmute(ppszconstraintvalue)).ok()
    }
    pub unsafe fn Next(&self, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Next)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppszconstraintname), ::core::mem::transmute(ppszconstraintvalue)).ok()
    }
    pub unsafe fn Skip(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Skip)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IProviderQueryConstraintCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IProviderQueryConstraintCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProviderQueryConstraintCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderQueryConstraintCollection {}
impl ::core::fmt::Debug for IProviderQueryConstraintCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderQueryConstraintCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProviderQueryConstraintCollection {
    type Vtable = IProviderQueryConstraintCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IProviderQueryConstraintCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c243e11_3261_4bcd_b922_84a873d460ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderQueryConstraintCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszconstraintname: ::windows::core::PCWSTR, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const DEVICEDISPLAY_DISCOVERYMETHOD_AD_PRINTER: ::windows::core::PCWSTR = ::windows::w!("Published Printer");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const DEVICEDISPLAY_DISCOVERYMETHOD_ASP_INFRA: ::windows::core::PCWSTR = ::windows::w!("AspInfra");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const DEVICEDISPLAY_DISCOVERYMETHOD_BLUETOOTH: ::windows::core::PCWSTR = ::windows::w!("Bluetooth");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const DEVICEDISPLAY_DISCOVERYMETHOD_BLUETOOTH_LE: ::windows::core::PCWSTR = ::windows::w!("Bluetooth Low Energy");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const DEVICEDISPLAY_DISCOVERYMETHOD_NETBIOS: ::windows::core::PCWSTR = ::windows::w!("NetBIOS");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const DEVICEDISPLAY_DISCOVERYMETHOD_PNP: ::windows::core::PCWSTR = ::windows::w!("PnP");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const DEVICEDISPLAY_DISCOVERYMETHOD_UPNP: ::windows::core::PCWSTR = ::windows::w!("UPnP");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const DEVICEDISPLAY_DISCOVERYMETHOD_WFD: ::windows::core::PCWSTR = ::windows::w!("WiFiDirect");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const DEVICEDISPLAY_DISCOVERYMETHOD_WSD: ::windows::core::PCWSTR = ::windows::w!("WSD");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const DEVICEDISPLAY_DISCOVERYMETHOD_WUSB: ::windows::core::PCWSTR = ::windows::w!("WUSB");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const E_FDPAIRING_AUTHFAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1882193917i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const E_FDPAIRING_AUTHNOTALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1882193914i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const E_FDPAIRING_CONNECTTIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-1882193916i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const E_FDPAIRING_HWFAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-1882193918i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const E_FDPAIRING_IPBUSDISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-1882193913i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const E_FDPAIRING_NOCONNECTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-1882193919i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const E_FDPAIRING_NOPROFILES: ::windows::core::HRESULT = ::windows::core::HRESULT(-1882193912i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const E_FDPAIRING_TOOMANYCONNECTIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-1882193915i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_BT: ::windows::core::PCWSTR = ::windows::w!("Provider\\Microsoft.Devices.Bluetooth");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_DEVICEDISPLAYOBJECTS: ::windows::core::PCWSTR = ::windows::w!("Provider\\Microsoft.Base.DeviceDisplayObjects");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_DEVICEFUNCTIONENUMERATORS: ::windows::core::PCWSTR = ::windows::w!("Layered\\Microsoft.Devices.FunctionEnumerators");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_DEVICEPAIRING: ::windows::core::PCWSTR = ::windows::w!("Layered\\Microsoft.Base.DevicePairing");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_DEVICES: ::windows::core::PCWSTR = ::windows::w!("Layered\\Microsoft.Base.Devices");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_DEVQUERYOBJECTS: ::windows::core::PCWSTR = ::windows::w!("Provider\\Microsoft.Base.DevQueryObjects");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_NETBIOS: ::windows::core::PCWSTR = ::windows::w!("Provider\\Microsoft.Networking.Netbios");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_NETWORKDEVICES: ::windows::core::PCWSTR = ::windows::w!("Layered\\Microsoft.Networking.Devices");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_PNP: ::windows::core::PCWSTR = ::windows::w!("Provider\\Microsoft.Base.PnP");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_PNPXASSOCIATION: ::windows::core::PCWSTR = ::windows::w!("Provider\\Microsoft.PnPX.Association");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_PUBLICATION: ::windows::core::PCWSTR = ::windows::w!("Provider\\Microsoft.Base.Publication");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_REGISTRY: ::windows::core::PCWSTR = ::windows::w!("Provider\\Microsoft.Base.Registry");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_SSDP: ::windows::core::PCWSTR = ::windows::w!("Provider\\Microsoft.Networking.SSDP");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_WCN: ::windows::core::PCWSTR = ::windows::w!("Provider\\Microsoft.Networking.WCN");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_WSDISCOVERY: ::windows::core::PCWSTR = ::windows::w!("Provider\\Microsoft.Networking.WSD");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_CATEGORY_WUSB: ::windows::core::PCWSTR = ::windows::w!("Provider\\Microsoft.Devices.WirelessUSB");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_SUBCAT_DEVICES_WSDPRINTERS: ::windows::core::PCWSTR = ::windows::w!("WSDPrinters");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_SUBCAT_NETWORKDEVICES_SSDP: ::windows::core::PCWSTR = ::windows::w!("SSDP");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_SUBCAT_NETWORKDEVICES_WSD: ::windows::core::PCWSTR = ::windows::w!("WSD");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_SUBCAT_REG_DIRECTED: ::windows::core::PCWSTR = ::windows::w!("Directed");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FCTN_SUBCAT_REG_PUBLICATION: ::windows::core::PCWSTR = ::windows::w!("Publication");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_CONSTRAINTVALUE_ALL: ::windows::core::PCWSTR = ::windows::w!("All");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_CONSTRAINTVALUE_COMCLSCONTEXT_INPROC_SERVER: ::windows::core::PCWSTR = ::windows::w!("1");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_CONSTRAINTVALUE_COMCLSCONTEXT_LOCAL_SERVER: ::windows::core::PCWSTR = ::windows::w!("4");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_CONSTRAINTVALUE_FALSE: ::windows::core::PCWSTR = ::windows::w!("FALSE");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_CONSTRAINTVALUE_PAIRED: ::windows::core::PCWSTR = ::windows::w!("Paired");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_CONSTRAINTVALUE_RECURSESUBCATEGORY_TRUE: ::windows::core::PCWSTR = ::windows::w!("TRUE");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_CONSTRAINTVALUE_ROUTINGSCOPE_ALL: ::windows::core::PCWSTR = ::windows::w!("All");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_CONSTRAINTVALUE_ROUTINGSCOPE_DIRECT: ::windows::core::PCWSTR = ::windows::w!("Direct");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_CONSTRAINTVALUE_TRUE: ::windows::core::PCWSTR = ::windows::w!("TRUE");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_CONSTRAINTVALUE_UNPAIRED: ::windows::core::PCWSTR = ::windows::w!("UnPaired");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_CONSTRAINTVALUE_VISIBILITY_ALL: ::windows::core::PCWSTR = ::windows::w!("1");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_CONSTRAINTVALUE_VISIBILITY_DEFAULT: ::windows::core::PCWSTR = ::windows::w!("0");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_EVENTID: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_EVENTID_ASYNCTHREADEXIT: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_EVENTID_IPADDRESSCHANGE: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_EVENTID_PRIVATE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_EVENTID_QUERYREFRESH: u32 = 1004u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_EVENTID_SEARCHCOMPLETE: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_EVENTID_SEARCHSTART: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_LONGHORN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_QUERYCONSTRAINT_COMCLSCONTEXT: ::windows::core::PCWSTR = ::windows::w!("COMClsContext");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_QUERYCONSTRAINT_INQUIRY_TIMEOUT: ::windows::core::PCWSTR = ::windows::w!("InquiryModeTimeout");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_QUERYCONSTRAINT_PAIRING_STATE: ::windows::core::PCWSTR = ::windows::w!("PairingState");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_QUERYCONSTRAINT_PROVIDERINSTANCEID: ::windows::core::PCWSTR = ::windows::w!("ProviderInstanceID");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_QUERYCONSTRAINT_RECURSESUBCATEGORY: ::windows::core::PCWSTR = ::windows::w!("RecurseSubcategory");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_QUERYCONSTRAINT_ROUTINGSCOPE: ::windows::core::PCWSTR = ::windows::w!("RoutingScope");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_QUERYCONSTRAINT_SUBCATEGORY: ::windows::core::PCWSTR = ::windows::w!("Subcategory");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_QUERYCONSTRAINT_VISIBILITY: ::windows::core::PCWSTR = ::windows::w!("Visibility");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_SUBKEY: ::windows::core::PCWSTR = ::windows::w!("SOFTWARE\\Microsoft\\Function Discovery\\");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_Visibility_Default: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const FD_Visibility_Hidden: u32 = 1u32;
pub const FMTID_Device: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57);
pub const FMTID_DeviceInterface: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53808008_07bb_4661_bc3c_b5953e708560);
pub const FMTID_FD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x904b03a2_471d_423c_a584_f3483238a146);
pub const FMTID_PNPX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd);
pub const FMTID_PNPXDynamicProperty: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd);
pub const FMTID_Pairing: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc);
pub const FMTID_WSD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92506491_ff95_4724_a05a_5b81885a7c92);
pub const FunctionDiscovery: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc72be2ec_8e90_452c_b29a_ab8ff1c071fc);
pub const FunctionInstanceCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba818ce5_b55f_443f_ad39_2fe89be6191f);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const MAX_FDCONSTRAINTNAME_LENGTH: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const MAX_FDCONSTRAINTVALUE_LENGTH: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const ONLINE_PROVIDER_DEVICES_QUERYCONSTRAINT_OWNERNAME: ::windows::core::PCWSTR = ::windows::w!("OwnerName");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Characteristics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 29u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_ClassCoInstallers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x713d1703_a2e2_49f5_9214_56472ef3da5c), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_ClassInstaller: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_ClassName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_DefaultService: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_DevType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 27u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Exclusive: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 28u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_IconPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_LowerFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 20u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Name: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_NoDisplayClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_NoInstallClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_NoUseClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_PropPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_Security: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 25u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_SecuritySDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 26u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_SilentInstall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x259abffc_50a7_47ce_af08_68c9a7d73366), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceClass_UpperFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4321918b_f69e_470d_a5de_4d88c75ad24b), pid: 19u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Address: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 51u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_AlwaysShowDeviceAsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 101u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_AssociationArray: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 80u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_BaselineExperienceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 78u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 90u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_CategoryGroup_Desc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 94u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_CategoryGroup_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 95u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category_Desc_Plural: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 92u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category_Desc_Singular: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 91u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Category_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 93u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DeviceDescription1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 81u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DeviceDescription2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 82u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DeviceFunctionSubRank: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 100u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_DiscoveryMethod: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 52u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_ExperienceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 89u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12288u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 57u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_InstallInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x83da6326_97a6_4088_9453_a1923f573b29), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsAuthenticated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 54u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 55u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsDefaultDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 86u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsDeviceUniquelyIdentifiable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 79u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsEncrypted: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 53u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsLocalMachine: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 70u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsMetadataSearchInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 72u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsNetworkDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 85u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsNotInterestingForDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 74u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsNotWorkingProperly: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 83u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsPaired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 56u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsSharedDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 84u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_IsShowInDisconnectedState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 68u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Last_Connected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 67u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Last_Seen: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 66u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_LaunchDeviceStageFromExplorer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 77u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_LaunchDeviceStageOnDeviceConnect: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 76u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8192u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_MetadataCabinet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 87u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_MetadataChecksum: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 73u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_MetadataPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 71u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_ModelName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8194u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_ModelNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8195u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_PrimaryCategory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 97u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_RequiresPairingElevation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 88u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_RequiresUninstallElevation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 99u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_UnpairUninstall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 98u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceDisplay_Version: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 65u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterfaceClass_DefaultInterface: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x14c83a99_0b3f_44b7_be4c_a178d3990564), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_ClassGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_Enabled: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DeviceInterface_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_AdditionalSoftwareRequested: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 19u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Address: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 30u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BIOSVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xeaee7f1d_6a33_44d1_9441_5f46def23198), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BaseContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 38u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 23u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusReportedDeviceDesc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_BusTypeGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 21u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Capabilities: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 17u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Characteristics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 29u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Children: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Class: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ClassGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_CompatibleIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ConfigFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8c7ed206_3f8a_4827_b3ab_ae9e1faefc6c), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DHP_Rebalance_Policy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DevNodeStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DevType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 27u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DeviceDesc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Driver: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverCoInstallers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverDesc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverInfPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverInfSection: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverInfSectionExt: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverLogoLevel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverPropPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverRank: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_DriverVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_EjectionRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_EnumeratorName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 24u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Exclusive: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 28u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_FriendlyNameAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_GenericDriverInstalled: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 18u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_HardwareIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_InstallInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x83da6326_97a6_4088_9453_a1923f573b29), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_InstallState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 36u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_InstanceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x78c34fc8_104a_4aca_9ea4_524d52996e57), pid: 256u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_IsAssociateableByUserAction: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Legacy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x80497100_8c73_48b9_aad9_ce387e19c56e), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LegacyBusType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 22u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LocationInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LocationPaths: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 37u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_LowerFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 20u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ManufacturerAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_MatchingDeviceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ModelId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_NoConnectSound: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 17u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Numa_Node: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PDOName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 16u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Parent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PowerData: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 32u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PowerRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_PresenceNotForDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ProblemCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalPolicy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 33u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalPolicyDefault: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 34u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalPolicyOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 35u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_RemovalRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Reported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x80497100_8c73_48b9_aad9_ce387e19c56e), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ResourcePickerExceptions: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_ResourcePickerTags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa8b865dd_2e3d_4094_ad97_e593a70c75d6), pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SafeRemovalRequired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafd97640_86a3_4210_b67c_289c41aabe55), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SafeRemovalRequiredOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xafd97640_86a3_4210_b67c_289c41aabe55), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Security: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 25u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SecuritySDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 26u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Service: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_Siblings: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_SignalStrength: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x80d81ea6_7473_4b0c_8216_efc11a2c4c8b), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_TransportRelations: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4340a6c5_93fa_4706_972c_7b648008a5a7), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_UINumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 18u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_UINumberDescFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 31u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Device_UpperFilters: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0), pid: 19u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_BrandingIcon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_DetailedDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_DocumentationLink: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_Model: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_DrvPkg_VendorWebSite: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xcf73bb51_3abf_44a2_85e0_9a3dc7a12132), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_FunctionInstance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x08c0c253_a154_4746_9005_82de5317148b), pid: 1u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Devinst: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4097u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DisplayAttribute: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DriverDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DriverProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_DriverVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Function: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4099u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Image: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4098u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Model: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Name: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_SerialNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_ShellAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4100u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Hardware_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x5eaf3ef2_e0ca_4598_bf06_71ed1d9dd953), pid: 4096u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xb725f130_47ef_101a_a5f1_02608c9eebac), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Numa_Proximity_Domain: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x540b947e_8b40_45bc_a8a2_6a0b894cbda2), pid: 1u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Associated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Category_Desc_NonPlural: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12304u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_CompactSignature: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28674u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_CompatibleTypes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DeviceCategory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12292u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DeviceCategory_Desc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12293u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DeviceCertHash: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28675u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_DomainName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 20480u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_FirmwareVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12289u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_GlobalIdentity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4096u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4101u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_IPBusEnumerated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28688u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_InstallState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Installable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x4fc5077e_b686_44be_93e3_86cafe368ccd), pid: 1u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_IpAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12297u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ManufacturerUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8193u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_MetadataVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4100u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ModelUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8196u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_NetworkInterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12296u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_NetworkInterfaceLuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12295u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_PhysicalAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12294u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_PresentationUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8198u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_RemoteAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4102u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Removable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28672u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_RootProxy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4103u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Scopes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4098u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_SecureChannel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 28673u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_SerialNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 12290u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16384u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceControlUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16388u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceDescUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16389u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceEventSubUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16390u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16385u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ServiceTypes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 16386u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_ShareName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 20482u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Types: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4097u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_Upc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 8197u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PNPX_XAddrs: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 4099u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_IsWifiOnlyDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 16u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemDefault: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemIcon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_Pairing_ListItemText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8807cae6_7db6_4f10_8ee4_435eaa1392bc), pid: 1u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SSDP_AltLocationInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24576u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SSDP_DevLifeTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24577u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_SSDP_NetworkInterface: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x656a3bb3_ecc0_43fd_8477_4ae0404a96cd), pid: 24578u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_AssocState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b88_4684_11da_a26a_0002b3988e81), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_AuthType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b82_4684_11da_a26a_0002b3988e81), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConfigError: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConfigMethods: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b85_4684_11da_a26a_0002b3988e81), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConfigState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 11u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_ConnType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b84_4684_11da_a26a_0002b3988e81), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_DevicePasswordId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 12u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_EncryptType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b83_4684_11da_a26a_0002b3988e81), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_OSVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b89_4684_11da_a26a_0002b3988e81), pid: 13u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_RegistrarType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b8b_4684_11da_a26a_0002b3988e81), pid: 15u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_RequestType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b81_4684_11da_a26a_0002b3988e81), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_RfBand: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b87_4684_11da_a26a_0002b3988e81), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_VendorExtension: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b8a_4684_11da_a26a_0002b3988e81), pid: 14u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WCN_Version: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x88190b80_4684_11da_a26a_0002b3988e81), pid: 1u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Comment: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_DisplayType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_LocalName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Provider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_RemoteName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Scope: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 1u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Type: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_WNET_Usage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xdebda43a_37b3_4383_91e7_4498da2995ab), pid: 4u32 };
pub const PNPXAssociation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcee8ccc9_4f6b_4469_a235_5a22869eef03);
pub const PNPXPairingHandler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8a27942_ade7_4085_aa6e_4fadc7ada1ef);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_CAMERA: ::windows::core::PCWSTR = ::windows::w!("Cameras");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_COMPUTER: ::windows::core::PCWSTR = ::windows::w!("Computers");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_DISPLAYS: ::windows::core::PCWSTR = ::windows::w!("Displays");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_FAX: ::windows::core::PCWSTR = ::windows::w!("FAX");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_GAMING_DEVICE: ::windows::core::PCWSTR = ::windows::w!("Gaming");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_HOME_AUTOMATION_SYSTEM: ::windows::core::PCWSTR = ::windows::w!("HomeAutomation");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_HOME_SECURITY_SYSTEM: ::windows::core::PCWSTR = ::windows::w!("HomeSecurity");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_INPUTDEVICE: ::windows::core::PCWSTR = ::windows::w!("Input");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_MFP: ::windows::core::PCWSTR = ::windows::w!("MFP");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_MULTIMEDIA_DEVICE: ::windows::core::PCWSTR = ::windows::w!("MediaDevices");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_NETWORK_INFRASTRUCTURE: ::windows::core::PCWSTR = ::windows::w!("NetworkInfrastructure");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_OTHER: ::windows::core::PCWSTR = ::windows::w!("Other");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_PRINTER: ::windows::core::PCWSTR = ::windows::w!("Printers");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_SCANNER: ::windows::core::PCWSTR = ::windows::w!("Scanners");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_STORAGE: ::windows::core::PCWSTR = ::windows::w!("Storage");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_DEVICECATEGORY_TELEPHONE: ::windows::core::PCWSTR = ::windows::w!("Phones");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_INSTALLSTATE_FAILED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_INSTALLSTATE_INSTALLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_INSTALLSTATE_INSTALLING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNPX_INSTALLSTATE_NOTINSTALLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNP_CONSTRAINTVALUE_NOTIFICATIONSONLY: ::windows::core::PCWSTR = ::windows::w!("TRUE");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PNP_CONSTRAINTVALUE_NOTPRESENT: ::windows::core::PCWSTR = ::windows::w!("TRUE");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERDDO_QUERYCONSTRAINT_DEVICEFUNCTIONDISPLAYOBJECTS: ::windows::core::PCWSTR = ::windows::w!("DeviceFunctionDisplayObjects");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERDDO_QUERYCONSTRAINT_DEVICEINTERFACES: ::windows::core::PCWSTR = ::windows::w!("DeviceInterfaces");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERDDO_QUERYCONSTRAINT_ONLYCONNECTEDDEVICES: ::windows::core::PCWSTR = ::windows::w!("OnlyConnectedDevices");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERPNP_QUERYCONSTRAINT_INTERFACECLASS: ::windows::core::PCWSTR = ::windows::w!("InterfaceClass");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERPNP_QUERYCONSTRAINT_NOTIFICATIONSONLY: ::windows::core::PCWSTR = ::windows::w!("NotifyOnly");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERPNP_QUERYCONSTRAINT_NOTPRESENT: ::windows::core::PCWSTR = ::windows::w!("NotPresent");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERSSDP_QUERYCONSTRAINT_CUSTOMXMLPROPERTY: ::windows::core::PCWSTR = ::windows::w!("CustomXmlProperty");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERSSDP_QUERYCONSTRAINT_TYPE: ::windows::core::PCWSTR = ::windows::w!("Type");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERWNET_QUERYCONSTRAINT_PROPERTIES: ::windows::core::PCWSTR = ::windows::w!("Properties");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERWNET_QUERYCONSTRAINT_RESOURCETYPE: ::windows::core::PCWSTR = ::windows::w!("ResourceType");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERWNET_QUERYCONSTRAINT_TYPE: ::windows::core::PCWSTR = ::windows::w!("Type");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERWSD_QUERYCONSTRAINT_DIRECTEDADDRESS: ::windows::core::PCWSTR = ::windows::w!("RemoteAddress");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERWSD_QUERYCONSTRAINT_SCOPE: ::windows::core::PCWSTR = ::windows::w!("Scope");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERWSD_QUERYCONSTRAINT_SECURITY_REQUIREMENTS: ::windows::core::PCWSTR = ::windows::w!("SecurityRequirements");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERWSD_QUERYCONSTRAINT_SSL_CERTHASH_FOR_SERVER_AUTH: ::windows::core::PCWSTR = ::windows::w!("SSLServerAuthCertHash");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERWSD_QUERYCONSTRAINT_SSL_CERT_FOR_CLIENT_AUTH: ::windows::core::PCWSTR = ::windows::w!("SSLClientAuthCert");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const PROVIDERWSD_QUERYCONSTRAINT_TYPE: ::windows::core::PCWSTR = ::windows::w!("Type");
pub const PropertyStore: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4796550_df61_448b_9193_13fc1341b163);
pub const PropertyStoreCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedd36029_d753_4862_aa5b_5bccad2a4d29);
pub const SID_DeviceDisplayStatusManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf59aa553_8309_46ca_9736_1ac3c62d6031);
pub const SID_EnumDeviceFunction: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13e0e9e2_c3fa_4e3c_906e_64502fa4dc95);
pub const SID_EnumInterface: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40eab0b9_4d7f_4b53_a334_1581dd9041f4);
pub const SID_FDPairingHandler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x383b69fa_5486_49da_91f5_d63c24c8e9d0);
pub const SID_FunctionDiscoveryProviderRefresh: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b4cbdc9_31c4_40d4_a62d_772aa174ed52);
pub const SID_PNPXAssociation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcee8ccc9_4f6b_4469_a235_5a22869eef03);
pub const SID_PNPXPropertyStore: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa86530b1_542f_439f_b71c_b0756b13677a);
pub const SID_PNPXServiceCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x439e80ee_a217_4712_9fa6_deabd9c2a727);
pub const SID_PnpProvider: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8101368e_cabb_4426_acff_96c410812000);
pub const SID_UPnPActivator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d0d66eb_cf74_4164_b52f_08344672dd46);
pub const SID_UninstallDeviceFunction: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc920566e_5671_4496_8025_bf0b89bd44cd);
pub const SID_UnpairProvider: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89a502fc_857b_4698_a0b7_027192002f9e);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const SSDP_CONSTRAINTVALUE_TYPE_ALL: ::windows::core::PCWSTR = ::windows::w!("ssdp:all");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const SSDP_CONSTRAINTVALUE_TYPE_DEVICE_PREFIX: ::windows::core::PCWSTR = ::windows::w!("urn:schemas-upnp-org:device:");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const SSDP_CONSTRAINTVALUE_TYPE_ROOT: ::windows::core::PCWSTR = ::windows::w!("upnp:rootdevice");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const SSDP_CONSTRAINTVALUE_TYPE_SVC_PREFIX: ::windows::core::PCWSTR = ::windows::w!("urn:schemas-upnp-org:service:");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const WNET_CONSTRAINTVALUE_PROPERTIES_ALL: ::windows::core::PCWSTR = ::windows::w!("All");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const WNET_CONSTRAINTVALUE_PROPERTIES_LIMITED: ::windows::core::PCWSTR = ::windows::w!("Limited");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const WNET_CONSTRAINTVALUE_RESOURCETYPE_DISK: ::windows::core::PCWSTR = ::windows::w!("Disk");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const WNET_CONSTRAINTVALUE_RESOURCETYPE_DISKORPRINTER: ::windows::core::PCWSTR = ::windows::w!("DiskOrPrinter");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const WNET_CONSTRAINTVALUE_RESOURCETYPE_PRINTER: ::windows::core::PCWSTR = ::windows::w!("Printer");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const WNET_CONSTRAINTVALUE_TYPE_ALL: ::windows::core::PCWSTR = ::windows::w!("All");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const WNET_CONSTRAINTVALUE_TYPE_DOMAIN: ::windows::core::PCWSTR = ::windows::w!("Domain");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const WNET_CONSTRAINTVALUE_TYPE_SERVER: ::windows::core::PCWSTR = ::windows::w!("Server");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const WSD_CONSTRAINTVALUE_NO_TRUST_VERIFICATION: ::windows::core::PCWSTR = ::windows::w!("3");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const WSD_CONSTRAINTVALUE_REQUIRE_SECURECHANNEL: ::windows::core::PCWSTR = ::windows::w!("1");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const WSD_CONSTRAINTVALUE_REQUIRE_SECURECHANNEL_AND_COMPACTSIGNATURE: ::windows::core::PCWSTR = ::windows::w!("2");
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PropertyConstraint(pub i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QC_EQUALS: PropertyConstraint = PropertyConstraint(0i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QC_NOTEQUAL: PropertyConstraint = PropertyConstraint(1i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QC_LESSTHAN: PropertyConstraint = PropertyConstraint(2i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QC_LESSTHANOREQUAL: PropertyConstraint = PropertyConstraint(3i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QC_GREATERTHAN: PropertyConstraint = PropertyConstraint(4i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QC_GREATERTHANOREQUAL: PropertyConstraint = PropertyConstraint(5i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QC_STARTSWITH: PropertyConstraint = PropertyConstraint(6i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QC_EXISTS: PropertyConstraint = PropertyConstraint(7i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QC_DOESNOTEXIST: PropertyConstraint = PropertyConstraint(8i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QC_CONTAINS: PropertyConstraint = PropertyConstraint(9i32);
impl ::core::marker::Copy for PropertyConstraint {}
impl ::core::clone::Clone for PropertyConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PropertyConstraint {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PropertyConstraint {
    type Abi = Self;
}
impl ::core::fmt::Debug for PropertyConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyConstraint").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QueryCategoryType(pub i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QCT_PROVIDER: QueryCategoryType = QueryCategoryType(0i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QCT_LAYERED: QueryCategoryType = QueryCategoryType(1i32);
impl ::core::marker::Copy for QueryCategoryType {}
impl ::core::clone::Clone for QueryCategoryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QueryCategoryType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QueryCategoryType {
    type Abi = Self;
}
impl ::core::fmt::Debug for QueryCategoryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QueryCategoryType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QueryUpdateAction(pub i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QUA_ADD: QueryUpdateAction = QueryUpdateAction(0i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QUA_REMOVE: QueryUpdateAction = QueryUpdateAction(1i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const QUA_CHANGE: QueryUpdateAction = QueryUpdateAction(2i32);
impl ::core::marker::Copy for QueryUpdateAction {}
impl ::core::clone::Clone for QueryUpdateAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QueryUpdateAction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QueryUpdateAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for QueryUpdateAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QueryUpdateAction").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SystemVisibilityFlags(pub i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const SVF_SYSTEM: SystemVisibilityFlags = SystemVisibilityFlags(0i32);
#[doc = "*Required features: `\"Win32_Devices_FunctionDiscovery\"`*"]
pub const SVF_USER: SystemVisibilityFlags = SystemVisibilityFlags(1i32);
impl ::core::marker::Copy for SystemVisibilityFlags {}
impl ::core::clone::Clone for SystemVisibilityFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemVisibilityFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SystemVisibilityFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemVisibilityFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemVisibilityFlags").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
