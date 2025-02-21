#[cfg(feature = "Devices_Pwm_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IPwmController(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPwmController {
    type Vtable = IPwmController_Vtbl;
}
unsafe impl ::windows::core::Interface for IPwmController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc45f5c85_d2e8_42cf_9bd6_cf5ed029e6a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PinCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ActualFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDesiredFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredfrequency: f64, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MinFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MaxFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub OpenPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinnumber: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPwmControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPwmControllerStatics {
    type Vtable = IPwmControllerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPwmControllerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4263bda1_8946_4404_bd48_81dd124af4d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmControllerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Pwm_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Pwm_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPwmControllerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPwmControllerStatics2 {
    type Vtable = IPwmControllerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPwmControllerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44fc5b1f_f119_4bdd_97ad_f76ef986736d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmControllerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPwmControllerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPwmControllerStatics3 {
    type Vtable = IPwmControllerStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for IPwmControllerStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2581871_0229_4344_ae3f_9b7cd0e66b94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmControllerStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPwmPin(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPwmPin {
    type Vtable = IPwmPin_Vtbl;
}
unsafe impl ::windows::core::Interface for IPwmPin {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22972dc8_c6cf_4821_b7f9_c6454fb6af79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmPin_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Controller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetActiveDutyCyclePercentage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetActiveDutyCyclePercentage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dutycyclepercentage: f64) -> ::windows::core::HRESULT,
    pub Polarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PwmPulsePolarity) -> ::windows::core::HRESULT,
    pub SetPolarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PwmPulsePolarity) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Pwm\"`*"]
#[repr(transparent)]
pub struct PwmController(::windows::core::IUnknown);
impl PwmController {
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
    pub fn SetDesiredFrequency(&self, desiredfrequency: f64) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetDesiredFrequency)(::windows::core::Vtable::as_raw(this), desiredfrequency, result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn MinFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinFrequency)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn MaxFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxFrequency)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn OpenPin(&self, pinnumber: i32) -> ::windows::core::Result<PwmPin> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenPin)(::windows::core::Vtable::as_raw(this), pinnumber, result__.as_mut_ptr()).from_abi::<PwmPin>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Pwm_Provider", feature = "Foundation_Collections"))]
    pub fn GetControllersAsync<'a, P0, E0>(provider: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PwmController>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, Provider::IPwmProvider>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPwmControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetControllersAsync)(::windows::core::Vtable::as_raw(this), provider.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PwmController>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PwmController>> {
        Self::IPwmControllerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PwmController>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPwmControllerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorFromFriendlyName(friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPwmControllerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorFromFriendlyName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(friendlyname), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PwmController>> {
        Self::IPwmControllerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PwmController>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPwmControllerStatics<R, F: FnOnce(&IPwmControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PwmController, IPwmControllerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPwmControllerStatics2<R, F: FnOnce(&IPwmControllerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PwmController, IPwmControllerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPwmControllerStatics3<R, F: FnOnce(&IPwmControllerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PwmController, IPwmControllerStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PwmController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PwmController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PwmController {}
impl ::core::fmt::Debug for PwmController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PwmController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PwmController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Pwm.PwmController;{c45f5c85-d2e8-42cf-9bd6-cf5ed029e6a7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PwmController {
    type Vtable = IPwmController_Vtbl;
}
unsafe impl ::windows::core::Interface for PwmController {
    const IID: ::windows::core::GUID = <IPwmController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PwmController {
    const NAME: &'static str = "Windows.Devices.Pwm.PwmController";
}
::windows::core::interface_hierarchy!(PwmController, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PwmController {}
unsafe impl ::core::marker::Sync for PwmController {}
#[doc = "*Required features: `\"Devices_Pwm\"`*"]
#[repr(transparent)]
pub struct PwmPin(::windows::core::IUnknown);
impl PwmPin {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Controller(&self) -> ::windows::core::Result<PwmController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Controller)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PwmController>(result__)
        }
    }
    pub fn GetActiveDutyCyclePercentage(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetActiveDutyCyclePercentage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetActiveDutyCyclePercentage(&self, dutycyclepercentage: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetActiveDutyCyclePercentage)(::windows::core::Vtable::as_raw(this), dutycyclepercentage).ok() }
    }
    pub fn Polarity(&self) -> ::windows::core::Result<PwmPulsePolarity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Polarity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PwmPulsePolarity>(result__)
        }
    }
    pub fn SetPolarity(&self, value: PwmPulsePolarity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPolarity)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn IsStarted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStarted)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PwmPin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PwmPin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PwmPin {}
impl ::core::fmt::Debug for PwmPin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PwmPin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PwmPin {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Pwm.PwmPin;{22972dc8-c6cf-4821-b7f9-c6454fb6af79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PwmPin {
    type Vtable = IPwmPin_Vtbl;
}
unsafe impl ::windows::core::Interface for PwmPin {
    const IID: ::windows::core::GUID = <IPwmPin as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PwmPin {
    const NAME: &'static str = "Windows.Devices.Pwm.PwmPin";
}
::windows::core::interface_hierarchy!(PwmPin, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PwmPin> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PwmPin) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PwmPin> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PwmPin) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&PwmPin> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PwmPin) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for PwmPin {}
unsafe impl ::core::marker::Sync for PwmPin {}
#[doc = "*Required features: `\"Devices_Pwm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PwmPulsePolarity(pub i32);
impl PwmPulsePolarity {
    pub const ActiveHigh: Self = Self(0i32);
    pub const ActiveLow: Self = Self(1i32);
}
impl ::core::marker::Copy for PwmPulsePolarity {}
impl ::core::clone::Clone for PwmPulsePolarity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PwmPulsePolarity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PwmPulsePolarity {
    type Abi = Self;
}
impl ::core::fmt::Debug for PwmPulsePolarity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PwmPulsePolarity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PwmPulsePolarity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Pwm.PwmPulsePolarity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
