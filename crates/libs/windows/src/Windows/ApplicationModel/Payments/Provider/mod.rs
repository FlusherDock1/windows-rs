#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentAppCanMakePaymentTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPaymentAppCanMakePaymentTriggerDetails {
    type Vtable = IPaymentAppCanMakePaymentTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IPaymentAppCanMakePaymentTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ce201f0_8b93_4eb6_8c46_2e4a6c6a26f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppCanMakePaymentTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportCanMakePaymentResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentAppManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPaymentAppManager {
    type Vtable = IPaymentAppManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IPaymentAppManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e47aa53_8521_4969_a957_df2538a3a98f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RegisterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedpaymentmethodids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RegisterAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnregisterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnregisterAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentAppManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPaymentAppManagerStatics {
    type Vtable = IPaymentAppManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPaymentAppManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa341ac28_fc89_4406_b4d9_34e7fe79dfb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentTransaction(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPaymentTransaction {
    type Vtable = IPaymentTransaction_Vtbl;
}
unsafe impl ::windows::core::Interface for IPaymentTransaction {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62581da0_26a5_4e9b_a6eb_66606cf001d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransaction_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PaymentRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPayerEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPayerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPayerPhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UpdateShippingAddressAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shippingaddress: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateShippingAddressAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateSelectedShippingOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectedshippingoption: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateSelectedShippingOptionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AcceptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paymenttoken: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcceptAsync: usize,
    pub Reject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentTransactionAcceptResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPaymentTransactionAcceptResult {
    type Vtable = IPaymentTransactionAcceptResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IPaymentTransactionAcceptResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x060e3276_d30c_4817_95a2_df7ae9273b56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransactionAcceptResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::PaymentRequestCompletionStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaymentTransactionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPaymentTransactionStatics {
    type Vtable = IPaymentTransactionStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPaymentTransactionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d639750_ee0a_4df5_9b1e_1c0f9ec59881);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransactionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
#[repr(transparent)]
pub struct PaymentAppCanMakePaymentTriggerDetails(::windows::core::IUnknown);
impl PaymentAppCanMakePaymentTriggerDetails {
    pub fn Request(&self) -> ::windows::core::Result<super::PaymentRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::PaymentRequest>(result__)
        }
    }
    pub fn ReportCanMakePaymentResult(&self, value: &super::PaymentCanMakePaymentResult) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ReportCanMakePaymentResult)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for PaymentAppCanMakePaymentTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentAppCanMakePaymentTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentAppCanMakePaymentTriggerDetails {}
impl ::core::fmt::Debug for PaymentAppCanMakePaymentTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentAppCanMakePaymentTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentAppCanMakePaymentTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails;{0ce201f0-8b93-4eb6-8c46-2e4a6c6a26f6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PaymentAppCanMakePaymentTriggerDetails {
    type Vtable = IPaymentAppCanMakePaymentTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for PaymentAppCanMakePaymentTriggerDetails {
    const IID: ::windows::core::GUID = <IPaymentAppCanMakePaymentTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentAppCanMakePaymentTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails";
}
::windows::core::interface_hierarchy!(PaymentAppCanMakePaymentTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PaymentAppCanMakePaymentTriggerDetails {}
unsafe impl ::core::marker::Sync for PaymentAppCanMakePaymentTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
#[repr(transparent)]
pub struct PaymentAppManager(::windows::core::IUnknown);
impl PaymentAppManager {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RegisterAsync<'a, P0, E0>(&self, supportedpaymentmethodids: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterAsync)(::windows::core::Vtable::as_raw(this), supportedpaymentmethodids.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnregisterAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnregisterAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Current() -> ::windows::core::Result<PaymentAppManager> {
        Self::IPaymentAppManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Current)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<PaymentAppManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentAppManagerStatics<R, F: FnOnce(&IPaymentAppManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentAppManager, IPaymentAppManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PaymentAppManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentAppManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentAppManager {}
impl ::core::fmt::Debug for PaymentAppManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentAppManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentAppManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentAppManager;{0e47aa53-8521-4969-a957-df2538a3a98f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PaymentAppManager {
    type Vtable = IPaymentAppManager_Vtbl;
}
unsafe impl ::windows::core::Interface for PaymentAppManager {
    const IID: ::windows::core::GUID = <IPaymentAppManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentAppManager";
}
::windows::core::interface_hierarchy!(PaymentAppManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PaymentAppManager {}
unsafe impl ::core::marker::Sync for PaymentAppManager {}
#[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
#[repr(transparent)]
pub struct PaymentTransaction(::windows::core::IUnknown);
impl PaymentTransaction {
    pub fn PaymentRequest(&self) -> ::windows::core::Result<super::PaymentRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PaymentRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::PaymentRequest>(result__)
        }
    }
    pub fn PayerEmail(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PayerEmail)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetPayerEmail(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPayerEmail)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PayerName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PayerName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetPayerName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPayerName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PayerPhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PayerPhoneNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetPayerPhoneNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPayerPhoneNumber)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateShippingAddressAsync(&self, shippingaddress: &super::PaymentAddress) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateShippingAddressAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(shippingaddress), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateSelectedShippingOptionAsync(&self, selectedshippingoption: &super::PaymentShippingOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateSelectedShippingOptionAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(selectedshippingoption), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AcceptAsync(&self, paymenttoken: &super::PaymentToken) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransactionAcceptResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AcceptAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(paymenttoken), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<PaymentTransactionAcceptResult>>(result__)
        }
    }
    pub fn Reject(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Reject)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransaction>> {
        Self::IPaymentTransactionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<PaymentTransaction>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPaymentTransactionStatics<R, F: FnOnce(&IPaymentTransactionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PaymentTransaction, IPaymentTransactionStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PaymentTransaction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentTransaction {}
impl ::core::fmt::Debug for PaymentTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentTransaction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentTransaction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentTransaction;{62581da0-26a5-4e9b-a6eb-66606cf001d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PaymentTransaction {
    type Vtable = IPaymentTransaction_Vtbl;
}
unsafe impl ::windows::core::Interface for PaymentTransaction {
    const IID: ::windows::core::GUID = <IPaymentTransaction as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentTransaction {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentTransaction";
}
::windows::core::interface_hierarchy!(PaymentTransaction, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PaymentTransaction {}
unsafe impl ::core::marker::Sync for PaymentTransaction {}
#[doc = "*Required features: `\"ApplicationModel_Payments_Provider\"`*"]
#[repr(transparent)]
pub struct PaymentTransactionAcceptResult(::windows::core::IUnknown);
impl PaymentTransactionAcceptResult {
    pub fn Status(&self) -> ::windows::core::Result<super::PaymentRequestCompletionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::PaymentRequestCompletionStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for PaymentTransactionAcceptResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaymentTransactionAcceptResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentTransactionAcceptResult {}
impl ::core::fmt::Debug for PaymentTransactionAcceptResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentTransactionAcceptResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaymentTransactionAcceptResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult;{060e3276-d30c-4817-95a2-df7ae9273b56})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PaymentTransactionAcceptResult {
    type Vtable = IPaymentTransactionAcceptResult_Vtbl;
}
unsafe impl ::windows::core::Interface for PaymentTransactionAcceptResult {
    const IID: ::windows::core::GUID = <IPaymentTransactionAcceptResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaymentTransactionAcceptResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult";
}
::windows::core::interface_hierarchy!(PaymentTransactionAcceptResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PaymentTransactionAcceptResult {}
unsafe impl ::core::marker::Sync for PaymentTransactionAcceptResult {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
