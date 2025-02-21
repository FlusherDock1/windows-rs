#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseGestureInfoHandle<'a, P0>(hgestureinfo: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HGESTUREINFO>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseGestureInfoHandle(hgestureinfo: HGESTUREINFO) -> super::super::super::Foundation::BOOL;
    }
    CloseGestureInfoHandle(hgestureinfo.into())
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseTouchInputHandle<'a, P0>(htouchinput: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HTOUCHINPUT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseTouchInputHandle(htouchinput: HTOUCHINPUT) -> super::super::super::Foundation::BOOL;
    }
    CloseTouchInputHandle(htouchinput.into())
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGestureConfig<'a, P0>(hwnd: P0, dwreserved: u32, dwflags: u32, pcids: *const u32, pgestureconfig: *mut GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetGestureConfig(hwnd: super::super::super::Foundation::HWND, dwreserved: u32, dwflags: u32, pcids: *const u32, pgestureconfig: *mut GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL;
    }
    GetGestureConfig(hwnd.into(), dwreserved, dwflags, ::core::mem::transmute(pcids), ::core::mem::transmute(pgestureconfig), cbsize)
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGestureExtraArgs<'a, P0>(hgestureinfo: P0, pextraargs: &mut [u8]) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HGESTUREINFO>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetGestureExtraArgs(hgestureinfo: HGESTUREINFO, cbextraargs: u32, pextraargs: *mut u8) -> super::super::super::Foundation::BOOL;
    }
    GetGestureExtraArgs(hgestureinfo.into(), pextraargs.len() as _, ::core::mem::transmute(pextraargs.as_ptr()))
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGestureInfo<'a, P0>(hgestureinfo: P0, pgestureinfo: *mut GESTUREINFO) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HGESTUREINFO>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetGestureInfo(hgestureinfo: HGESTUREINFO, pgestureinfo: *mut GESTUREINFO) -> super::super::super::Foundation::BOOL;
    }
    GetGestureInfo(hgestureinfo.into(), ::core::mem::transmute(pgestureinfo))
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTouchInputInfo<'a, P0>(htouchinput: P0, pinputs: &mut [TOUCHINPUT], cbsize: i32) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<HTOUCHINPUT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetTouchInputInfo(htouchinput: HTOUCHINPUT, cinputs: u32, pinputs: *mut TOUCHINPUT, cbsize: i32) -> super::super::super::Foundation::BOOL;
    }
    GetTouchInputInfo(htouchinput.into(), pinputs.len() as _, ::core::mem::transmute(pinputs.as_ptr()), cbsize)
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsTouchWindow<'a, P0>(hwnd: P0, pulflags: ::core::option::Option<*mut u32>) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsTouchWindow(hwnd: super::super::super::Foundation::HWND, pulflags: *mut u32) -> super::super::super::Foundation::BOOL;
    }
    IsTouchWindow(hwnd.into(), ::core::mem::transmute(pulflags.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterTouchWindow<'a, P0>(hwnd: P0, ulflags: REGISTER_TOUCH_WINDOW_FLAGS) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterTouchWindow(hwnd: super::super::super::Foundation::HWND, ulflags: REGISTER_TOUCH_WINDOW_FLAGS) -> super::super::super::Foundation::BOOL;
    }
    RegisterTouchWindow(hwnd.into(), ulflags)
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetGestureConfig<'a, P0>(hwnd: P0, dwreserved: u32, pgestureconfig: &[GESTURECONFIG], cbsize: u32) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetGestureConfig(hwnd: super::super::super::Foundation::HWND, dwreserved: u32, cids: u32, pgestureconfig: *const GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL;
    }
    SetGestureConfig(hwnd.into(), dwreserved, pgestureconfig.len() as _, ::core::mem::transmute(pgestureconfig.as_ptr()), cbsize)
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterTouchWindow<'a, P0>(hwnd: P0) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UnregisterTouchWindow(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::BOOL;
    }
    UnregisterTouchWindow(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
pub struct IInertiaProcessor(::windows::core::IUnknown);
impl IInertiaProcessor {
    pub unsafe fn InitialOriginX(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InitialOriginX)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialOriginX(&self, x: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialOriginX)(::windows::core::Vtable::as_raw(self), x).ok()
    }
    pub unsafe fn InitialOriginY(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InitialOriginY)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialOriginY(&self, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialOriginY)(::windows::core::Vtable::as_raw(self), y).ok()
    }
    pub unsafe fn InitialVelocityX(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InitialVelocityX)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialVelocityX(&self, x: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialVelocityX)(::windows::core::Vtable::as_raw(self), x).ok()
    }
    pub unsafe fn InitialVelocityY(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InitialVelocityY)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialVelocityY(&self, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialVelocityY)(::windows::core::Vtable::as_raw(self), y).ok()
    }
    pub unsafe fn InitialAngularVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InitialAngularVelocity)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialAngularVelocity(&self, velocity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialAngularVelocity)(::windows::core::Vtable::as_raw(self), velocity).ok()
    }
    pub unsafe fn InitialExpansionVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InitialExpansionVelocity)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialExpansionVelocity(&self, velocity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialExpansionVelocity)(::windows::core::Vtable::as_raw(self), velocity).ok()
    }
    pub unsafe fn InitialRadius(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InitialRadius)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetInitialRadius(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialRadius)(::windows::core::Vtable::as_raw(self), radius).ok()
    }
    pub unsafe fn BoundaryLeft(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BoundaryLeft)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetBoundaryLeft(&self, left: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBoundaryLeft)(::windows::core::Vtable::as_raw(self), left).ok()
    }
    pub unsafe fn BoundaryTop(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BoundaryTop)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetBoundaryTop(&self, top: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBoundaryTop)(::windows::core::Vtable::as_raw(self), top).ok()
    }
    pub unsafe fn BoundaryRight(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BoundaryRight)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetBoundaryRight(&self, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBoundaryRight)(::windows::core::Vtable::as_raw(self), right).ok()
    }
    pub unsafe fn BoundaryBottom(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BoundaryBottom)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetBoundaryBottom(&self, bottom: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBoundaryBottom)(::windows::core::Vtable::as_raw(self), bottom).ok()
    }
    pub unsafe fn ElasticMarginLeft(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ElasticMarginLeft)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetElasticMarginLeft(&self, left: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetElasticMarginLeft)(::windows::core::Vtable::as_raw(self), left).ok()
    }
    pub unsafe fn ElasticMarginTop(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ElasticMarginTop)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetElasticMarginTop(&self, top: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetElasticMarginTop)(::windows::core::Vtable::as_raw(self), top).ok()
    }
    pub unsafe fn ElasticMarginRight(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ElasticMarginRight)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetElasticMarginRight(&self, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetElasticMarginRight)(::windows::core::Vtable::as_raw(self), right).ok()
    }
    pub unsafe fn ElasticMarginBottom(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ElasticMarginBottom)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetElasticMarginBottom(&self, bottom: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetElasticMarginBottom)(::windows::core::Vtable::as_raw(self), bottom).ok()
    }
    pub unsafe fn DesiredDisplacement(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DesiredDisplacement)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetDesiredDisplacement(&self, displacement: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDesiredDisplacement)(::windows::core::Vtable::as_raw(self), displacement).ok()
    }
    pub unsafe fn DesiredRotation(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DesiredRotation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetDesiredRotation(&self, rotation: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDesiredRotation)(::windows::core::Vtable::as_raw(self), rotation).ok()
    }
    pub unsafe fn DesiredExpansion(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DesiredExpansion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetDesiredExpansion(&self, expansion: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDesiredExpansion)(::windows::core::Vtable::as_raw(self), expansion).ok()
    }
    pub unsafe fn DesiredDeceleration(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DesiredDeceleration)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetDesiredDeceleration(&self, deceleration: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDesiredDeceleration)(::windows::core::Vtable::as_raw(self), deceleration).ok()
    }
    pub unsafe fn DesiredAngularDeceleration(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DesiredAngularDeceleration)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetDesiredAngularDeceleration(&self, deceleration: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDesiredAngularDeceleration)(::windows::core::Vtable::as_raw(self), deceleration).ok()
    }
    pub unsafe fn DesiredExpansionDeceleration(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DesiredExpansionDeceleration)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetDesiredExpansionDeceleration(&self, deceleration: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDesiredExpansionDeceleration)(::windows::core::Vtable::as_raw(self), deceleration).ok()
    }
    pub unsafe fn InitialTimestamp(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).InitialTimestamp)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetInitialTimestamp(&self, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetInitialTimestamp)(::windows::core::Vtable::as_raw(self), timestamp).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Process(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Process)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProcessTime(&self, timestamp: u32) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ProcessTime)(::windows::core::Vtable::as_raw(self), timestamp, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Complete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Complete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CompleteTime(&self, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CompleteTime)(::windows::core::Vtable::as_raw(self), timestamp).ok()
    }
}
::windows::core::interface_hierarchy!(IInertiaProcessor, ::windows::core::IUnknown);
impl ::core::clone::Clone for IInertiaProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInertiaProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInertiaProcessor {}
impl ::core::fmt::Debug for IInertiaProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInertiaProcessor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IInertiaProcessor {
    type Vtable = IInertiaProcessor_Vtbl;
}
unsafe impl ::windows::core::Interface for IInertiaProcessor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18b00c6d_c5ee_41b1_90a9_9d4a929095ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaProcessor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InitialOriginX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialOriginX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT,
    pub InitialOriginY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialOriginY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT,
    pub InitialVelocityX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialVelocityX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32) -> ::windows::core::HRESULT,
    pub InitialVelocityY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, y: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialVelocityY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, y: f32) -> ::windows::core::HRESULT,
    pub InitialAngularVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialAngularVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT,
    pub InitialExpansionVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialExpansionVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocity: f32) -> ::windows::core::HRESULT,
    pub InitialRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: *mut f32) -> ::windows::core::HRESULT,
    pub SetInitialRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub BoundaryLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT,
    pub SetBoundaryLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT,
    pub BoundaryTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT,
    pub SetBoundaryTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT,
    pub BoundaryRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT,
    pub SetBoundaryRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT,
    pub BoundaryBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT,
    pub SetBoundaryBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT,
    pub ElasticMarginLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: *mut f32) -> ::windows::core::HRESULT,
    pub SetElasticMarginLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT,
    pub ElasticMarginTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: *mut f32) -> ::windows::core::HRESULT,
    pub SetElasticMarginTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT,
    pub ElasticMarginRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: *mut f32) -> ::windows::core::HRESULT,
    pub SetElasticMarginRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT,
    pub ElasticMarginBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: *mut f32) -> ::windows::core::HRESULT,
    pub SetElasticMarginBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT,
    pub DesiredDisplacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displacement: *mut f32) -> ::windows::core::HRESULT,
    pub SetDesiredDisplacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displacement: f32) -> ::windows::core::HRESULT,
    pub DesiredRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotation: *mut f32) -> ::windows::core::HRESULT,
    pub SetDesiredRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotation: f32) -> ::windows::core::HRESULT,
    pub DesiredExpansion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expansion: *mut f32) -> ::windows::core::HRESULT,
    pub SetDesiredExpansion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expansion: f32) -> ::windows::core::HRESULT,
    pub DesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT,
    pub DesiredAngularDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT,
    pub SetDesiredAngularDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT,
    pub DesiredExpansionDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: *mut f32) -> ::windows::core::HRESULT,
    pub SetDesiredExpansionDeceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deceleration: f32) -> ::windows::core::HRESULT,
    pub InitialTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: *mut u32) -> ::windows::core::HRESULT,
    pub SetInitialTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Process: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProcessTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u32, completed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProcessTime: usize,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CompleteTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
pub struct IManipulationProcessor(::windows::core::IUnknown);
impl IManipulationProcessor {
    pub unsafe fn SupportedManipulations(&self) -> ::windows::core::Result<MANIPULATION_PROCESSOR_MANIPULATIONS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SupportedManipulations)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<MANIPULATION_PROCESSOR_MANIPULATIONS>(result__)
    }
    pub unsafe fn SetSupportedManipulations(&self, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSupportedManipulations)(::windows::core::Vtable::as_raw(self), manipulations).ok()
    }
    pub unsafe fn PivotPointX(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PivotPointX)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetPivotPointX(&self, pivotpointx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPivotPointX)(::windows::core::Vtable::as_raw(self), pivotpointx).ok()
    }
    pub unsafe fn PivotPointY(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PivotPointY)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetPivotPointY(&self, pivotpointy: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPivotPointY)(::windows::core::Vtable::as_raw(self), pivotpointy).ok()
    }
    pub unsafe fn PivotRadius(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PivotRadius)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetPivotRadius(&self, pivotradius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPivotRadius)(::windows::core::Vtable::as_raw(self), pivotradius).ok()
    }
    pub unsafe fn CompleteManipulation(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CompleteManipulation)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ProcessDown(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ProcessDown)(::windows::core::Vtable::as_raw(self), manipulatorid, x, y).ok()
    }
    pub unsafe fn ProcessMove(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ProcessMove)(::windows::core::Vtable::as_raw(self), manipulatorid, x, y).ok()
    }
    pub unsafe fn ProcessUp(&self, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ProcessUp)(::windows::core::Vtable::as_raw(self), manipulatorid, x, y).ok()
    }
    pub unsafe fn ProcessDownWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ProcessDownWithTime)(::windows::core::Vtable::as_raw(self), manipulatorid, x, y, timestamp).ok()
    }
    pub unsafe fn ProcessMoveWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ProcessMoveWithTime)(::windows::core::Vtable::as_raw(self), manipulatorid, x, y, timestamp).ok()
    }
    pub unsafe fn ProcessUpWithTime(&self, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ProcessUpWithTime)(::windows::core::Vtable::as_raw(self), manipulatorid, x, y, timestamp).ok()
    }
    pub unsafe fn GetVelocityX(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVelocityX)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn GetVelocityY(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVelocityY)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn GetExpansionVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetExpansionVelocity)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn GetAngularVelocity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAngularVelocity)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn MinimumScaleRotateRadius(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MinimumScaleRotateRadius)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f32>(result__)
    }
    pub unsafe fn SetMinimumScaleRotateRadius(&self, minradius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMinimumScaleRotateRadius)(::windows::core::Vtable::as_raw(self), minradius).ok()
    }
}
::windows::core::interface_hierarchy!(IManipulationProcessor, ::windows::core::IUnknown);
impl ::core::clone::Clone for IManipulationProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IManipulationProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IManipulationProcessor {}
impl ::core::fmt::Debug for IManipulationProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IManipulationProcessor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IManipulationProcessor {
    type Vtable = IManipulationProcessor_Vtbl;
}
unsafe impl ::windows::core::Interface for IManipulationProcessor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa22ac519_8300_48a0_bef4_f1be8737dba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationProcessor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SupportedManipulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulations: *mut MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT,
    pub SetSupportedManipulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulations: MANIPULATION_PROCESSOR_MANIPULATIONS) -> ::windows::core::HRESULT,
    pub PivotPointX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotpointx: *mut f32) -> ::windows::core::HRESULT,
    pub SetPivotPointX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotpointx: f32) -> ::windows::core::HRESULT,
    pub PivotPointY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotpointy: *mut f32) -> ::windows::core::HRESULT,
    pub SetPivotPointY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotpointy: f32) -> ::windows::core::HRESULT,
    pub PivotRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotradius: *mut f32) -> ::windows::core::HRESULT,
    pub SetPivotRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pivotradius: f32) -> ::windows::core::HRESULT,
    pub CompleteManipulation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub ProcessMove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub ProcessUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub ProcessDownWithTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT,
    pub ProcessMoveWithTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT,
    pub ProcessUpWithTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manipulatorid: u32, x: f32, y: f32, timestamp: u32) -> ::windows::core::HRESULT,
    pub GetVelocityX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityx: *mut f32) -> ::windows::core::HRESULT,
    pub GetVelocityY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, velocityy: *mut f32) -> ::windows::core::HRESULT,
    pub GetExpansionVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expansionvelocity: *mut f32) -> ::windows::core::HRESULT,
    pub GetAngularVelocity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angularvelocity: *mut f32) -> ::windows::core::HRESULT,
    pub MinimumScaleRotateRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minradius: *mut f32) -> ::windows::core::HRESULT,
    pub SetMinimumScaleRotateRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minradius: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
pub struct _IManipulationEvents(::windows::core::IUnknown);
impl _IManipulationEvents {
    pub unsafe fn ManipulationStarted(&self, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ManipulationStarted)(::windows::core::Vtable::as_raw(self), x, y).ok()
    }
    pub unsafe fn ManipulationDelta(&self, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ManipulationDelta)(::windows::core::Vtable::as_raw(self), x, y, translationdeltax, translationdeltay, scaledelta, expansiondelta, rotationdelta, cumulativetranslationx, cumulativetranslationy, cumulativescale, cumulativeexpansion, cumulativerotation).ok()
    }
    pub unsafe fn ManipulationCompleted(&self, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ManipulationCompleted)(::windows::core::Vtable::as_raw(self), x, y, cumulativetranslationx, cumulativetranslationy, cumulativescale, cumulativeexpansion, cumulativerotation).ok()
    }
}
::windows::core::interface_hierarchy!(_IManipulationEvents, ::windows::core::IUnknown);
impl ::core::clone::Clone for _IManipulationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for _IManipulationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for _IManipulationEvents {}
impl ::core::fmt::Debug for _IManipulationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IManipulationEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for _IManipulationEvents {
    type Vtable = _IManipulationEvents_Vtbl;
}
unsafe impl ::windows::core::Interface for _IManipulationEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f62c8da_9c53_4b22_93df_927a862bbb03);
}
#[repr(C)]
#[doc(hidden)]
pub struct _IManipulationEvents_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ManipulationStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub ManipulationDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, translationdeltax: f32, translationdeltay: f32, scaledelta: f32, expansiondelta: f32, rotationdelta: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT,
    pub ManipulationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, cumulativetranslationx: f32, cumulativetranslationy: f32, cumulativescale: f32, cumulativeexpansion: f32, cumulativerotation: f32) -> ::windows::core::HRESULT,
}
pub const InertiaProcessor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabb27087_4ce0_4e58_a0cb_e24df96814be);
pub const ManipulationProcessor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x597d4fb0_47fd_4aff_89b9_c6cfae8cf08e);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GESTURECONFIG_ID(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_BEGIN: GESTURECONFIG_ID = GESTURECONFIG_ID(1u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_END: GESTURECONFIG_ID = GESTURECONFIG_ID(2u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_ZOOM: GESTURECONFIG_ID = GESTURECONFIG_ID(3u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_PAN: GESTURECONFIG_ID = GESTURECONFIG_ID(4u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_ROTATE: GESTURECONFIG_ID = GESTURECONFIG_ID(5u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_TWOFINGERTAP: GESTURECONFIG_ID = GESTURECONFIG_ID(6u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_PRESSANDTAP: GESTURECONFIG_ID = GESTURECONFIG_ID(7u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const GID_ROLLOVER: GESTURECONFIG_ID = GESTURECONFIG_ID(7u32);
impl ::core::marker::Copy for GESTURECONFIG_ID {}
impl ::core::clone::Clone for GESTURECONFIG_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GESTURECONFIG_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GESTURECONFIG_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for GESTURECONFIG_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GESTURECONFIG_ID").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GESTURECONFIG_ID {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GESTURECONFIG_ID {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GESTURECONFIG_ID {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GESTURECONFIG_ID {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GESTURECONFIG_ID {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MANIPULATION_PROCESSOR_MANIPULATIONS(pub i32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_NONE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(0i32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_TRANSLATE_X: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(1i32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_TRANSLATE_Y: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(2i32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_SCALE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(4i32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_ROTATE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(8i32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const MANIPULATION_ALL: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(15i32);
impl ::core::marker::Copy for MANIPULATION_PROCESSOR_MANIPULATIONS {}
impl ::core::clone::Clone for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MANIPULATION_PROCESSOR_MANIPULATIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MANIPULATION_PROCESSOR_MANIPULATIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REGISTER_TOUCH_WINDOW_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TWF_FINETOUCH: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TWF_WANTPALM: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(2u32);
impl ::core::marker::Copy for REGISTER_TOUCH_WINDOW_FLAGS {}
impl ::core::clone::Clone for REGISTER_TOUCH_WINDOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REGISTER_TOUCH_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REGISTER_TOUCH_WINDOW_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REGISTER_TOUCH_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGISTER_TOUCH_WINDOW_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOUCHEVENTF_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_MOVE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_DOWN: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_UP: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_INRANGE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_PRIMARY: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_NOCOALESCE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_PEN: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHEVENTF_PALM: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(128u32);
impl ::core::marker::Copy for TOUCHEVENTF_FLAGS {}
impl ::core::clone::Clone for TOUCHEVENTF_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOUCHEVENTF_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TOUCHEVENTF_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TOUCHEVENTF_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOUCHEVENTF_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TOUCHEVENTF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TOUCHEVENTF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TOUCHEVENTF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TOUCHINPUTMASKF_MASK(pub u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHINPUTMASKF_EXTRAINFO: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub const TOUCHINPUTMASKF_CONTACTAREA: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(4u32);
impl ::core::marker::Copy for TOUCHINPUTMASKF_MASK {}
impl ::core::clone::Clone for TOUCHINPUTMASKF_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TOUCHINPUTMASKF_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TOUCHINPUTMASKF_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for TOUCHINPUTMASKF_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOUCHINPUTMASKF_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TOUCHINPUTMASKF_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TOUCHINPUTMASKF_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TOUCHINPUTMASKF_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`*"]
pub struct GESTURECONFIG {
    pub dwID: GESTURECONFIG_ID,
    pub dwWant: u32,
    pub dwBlock: u32,
}
impl ::core::marker::Copy for GESTURECONFIG {}
impl ::core::clone::Clone for GESTURECONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GESTURECONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GESTURECONFIG").field("dwID", &self.dwID).field("dwWant", &self.dwWant).field("dwBlock", &self.dwBlock).finish()
    }
}
unsafe impl ::windows::core::Abi for GESTURECONFIG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GESTURECONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GESTURECONFIG>()) == 0 }
    }
}
impl ::core::cmp::Eq for GESTURECONFIG {}
impl ::core::default::Default for GESTURECONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GESTUREINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwID: u32,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptsLocation: super::super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
    pub dwSequenceID: u32,
    pub ullArguments: u64,
    pub cbExtraArgs: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GESTUREINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GESTUREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GESTUREINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GESTUREINFO").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("dwID", &self.dwID).field("hwndTarget", &self.hwndTarget).field("ptsLocation", &self.ptsLocation).field("dwInstanceID", &self.dwInstanceID).field("dwSequenceID", &self.dwSequenceID).field("ullArguments", &self.ullArguments).field("cbExtraArgs", &self.cbExtraArgs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GESTUREINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GESTUREINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GESTUREINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GESTUREINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GESTUREINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GESTURENOTIFYSTRUCT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptsLocation: super::super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GESTURENOTIFYSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GESTURENOTIFYSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GESTURENOTIFYSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GESTURENOTIFYSTRUCT").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("hwndTarget", &self.hwndTarget).field("ptsLocation", &self.ptsLocation).field("dwInstanceID", &self.dwInstanceID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GESTURENOTIFYSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GESTURENOTIFYSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GESTURENOTIFYSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GESTURENOTIFYSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GESTURENOTIFYSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HGESTUREINFO(pub isize);
impl HGESTUREINFO {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HGESTUREINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HGESTUREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HGESTUREINFO {}
impl ::core::fmt::Debug for HGESTUREINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HGESTUREINFO").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HGESTUREINFO>> for HGESTUREINFO {
    fn from(optional: ::core::option::Option<HGESTUREINFO>) -> HGESTUREINFO {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HGESTUREINFO {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HTOUCHINPUT(pub isize);
impl HTOUCHINPUT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HTOUCHINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HTOUCHINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HTOUCHINPUT {}
impl ::core::fmt::Debug for HTOUCHINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTOUCHINPUT").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HTOUCHINPUT>> for HTOUCHINPUT {
    fn from(optional: ::core::option::Option<HTOUCHINPUT>) -> HTOUCHINPUT {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HTOUCHINPUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Input_Touch\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOUCHINPUT {
    pub x: i32,
    pub y: i32,
    pub hSource: super::super::super::Foundation::HANDLE,
    pub dwID: u32,
    pub dwFlags: TOUCHEVENTF_FLAGS,
    pub dwMask: TOUCHINPUTMASKF_MASK,
    pub dwTime: u32,
    pub dwExtraInfo: usize,
    pub cxContact: u32,
    pub cyContact: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOUCHINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOUCHINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOUCHINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOUCHINPUT").field("x", &self.x).field("y", &self.y).field("hSource", &self.hSource).field("dwID", &self.dwID).field("dwFlags", &self.dwFlags).field("dwMask", &self.dwMask).field("dwTime", &self.dwTime).field("dwExtraInfo", &self.dwExtraInfo).field("cxContact", &self.cxContact).field("cyContact", &self.cyContact).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TOUCHINPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOUCHINPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TOUCHINPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOUCHINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOUCHINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
