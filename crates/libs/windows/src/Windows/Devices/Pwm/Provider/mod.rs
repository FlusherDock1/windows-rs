#[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
#[repr(transparent)]
pub struct IPwmControllerProvider(::windows::core::IUnknown);
impl IPwmControllerProvider {
    pub fn PinCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PinCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ActualFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActualFrequency)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDesiredFrequency(&self, frequency: f64) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetDesiredFrequency)(::windows::core::Vtable::as_raw(this), frequency, result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn MaxFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxFrequency)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn MinFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinFrequency)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn AcquirePin(&self, pin: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AcquirePin)(::windows::core::Vtable::as_raw(this), pin).ok() }
    }
    pub fn ReleasePin(&self, pin: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ReleasePin)(::windows::core::Vtable::as_raw(this), pin).ok() }
    }
    pub fn EnablePin(&self, pin: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).EnablePin)(::windows::core::Vtable::as_raw(this), pin).ok() }
    }
    pub fn DisablePin(&self, pin: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).DisablePin)(::windows::core::Vtable::as_raw(this), pin).ok() }
    }
    pub fn SetPulseParameters(&self, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPulseParameters)(::windows::core::Vtable::as_raw(this), pin, dutycycle, invertpolarity).ok() }
    }
}
::windows::core::interface_hierarchy!(IPwmControllerProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IPwmControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPwmControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPwmControllerProvider {}
impl ::core::fmt::Debug for IPwmControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPwmControllerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPwmControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1300593b-e2e3-40a4-b7d9-48dff0377a52}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPwmControllerProvider {
    type Vtable = IPwmControllerProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IPwmControllerProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1300593b_e2e3_40a4_b7d9_48dff0377a52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmControllerProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PinCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ActualFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDesiredFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frequency: f64, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MaxFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MinFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub AcquirePin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT,
    pub ReleasePin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT,
    pub EnablePin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT,
    pub DisablePin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT,
    pub SetPulseParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
#[repr(transparent)]
pub struct IPwmProvider(::windows::core::IUnknown);
impl IPwmProvider {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetControllers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IPwmProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IPwmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPwmProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPwmProvider {}
impl ::core::fmt::Debug for IPwmProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPwmProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPwmProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a3301228-52f1-47b0-9349-66ba43d25902}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPwmProvider {
    type Vtable = IPwmProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IPwmProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3301228_52f1_47b0_9349_66ba43d25902);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllers: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
