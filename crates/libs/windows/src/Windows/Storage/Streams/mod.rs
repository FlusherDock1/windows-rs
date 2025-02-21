#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IBuffer(::windows::core::IUnknown);
impl IBuffer {
    pub fn Capacity(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Capacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLength)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
::windows::core::interface_hierarchy!(IBuffer, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBuffer {}
impl ::core::fmt::Debug for IBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{905a0fe0-bc53-11df-8c49-001e4fc686da}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IBuffer {
    type Vtable = IBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for IBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe0_bc53_11df_8c49_001e4fc686da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBuffer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Capacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBufferFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBufferFactory {
    type Vtable = IBufferFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IBufferFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71af914d_c10f_484b_bc50_14bc623b3a27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capacity: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBufferStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBufferStatics {
    type Vtable = IBufferStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IBufferStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe901e65b_d716_475a_a90a_af7229b1e741);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateCopyFromMemoryBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateCopyFromMemoryBuffer: usize,
    #[cfg(feature = "Foundation")]
    pub CreateMemoryBufferOverIBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMemoryBufferOverIBuffer: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IContentTypeProvider(::windows::core::IUnknown);
impl IContentTypeProvider {
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IContentTypeProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IContentTypeProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContentTypeProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContentTypeProvider {}
impl ::core::fmt::Debug for IContentTypeProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContentTypeProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IContentTypeProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{97d098a5-3b99-4de9-88a5-e11d2f50c795}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IContentTypeProvider {
    type Vtable = IContentTypeProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IContentTypeProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97d098a5_3b99_4de9_88a5_e11d2f50c795);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentTypeProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IDataReader(::windows::core::IUnknown);
impl IDataReader {
    pub fn UnconsumedBufferLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnconsumedBufferLength)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnicodeEncoding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<UnicodeEncoding>(result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetUnicodeEncoding)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ByteOrder)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ByteOrder>(result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetByteOrder)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn InputStreamOptions(&self) -> ::windows::core::Result<InputStreamOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputStreamOptions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InputStreamOptions>(result__)
        }
    }
    pub fn SetInputStreamOptions(&self, value: InputStreamOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetInputStreamOptions)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ReadByte(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadByte)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn ReadBytes(&self, value: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ReadBytes)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_mut_ptr()).ok() }
    }
    pub fn ReadBuffer(&self, length: u32) -> ::windows::core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadBuffer)(::windows::core::Vtable::as_raw(this), length, result__.as_mut_ptr()).from_abi::<IBuffer>(result__)
        }
    }
    pub fn ReadBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadBoolean)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ReadGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadGuid)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn ReadInt16(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadInt16)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i16>(result__)
        }
    }
    pub fn ReadInt32(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadInt32)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ReadInt64(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadInt64)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    pub fn ReadUInt16(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadUInt16)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn ReadUInt32(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadUInt32)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ReadUInt64(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadUInt64)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn ReadSingle(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadSingle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn ReadDouble(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadDouble)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ReadString(&self, codeunitcount: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadString)(::windows::core::Vtable::as_raw(this), codeunitcount, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadDateTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadDateTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadTimeSpan(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadTimeSpan)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadAsync(&self, count: u32) -> ::windows::core::Result<DataReaderLoadOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadAsync)(::windows::core::Vtable::as_raw(this), count, result__.as_mut_ptr()).from_abi::<DataReaderLoadOperation>(result__)
        }
    }
    pub fn DetachBuffer(&self) -> ::windows::core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DetachBuffer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IBuffer>(result__)
        }
    }
    pub fn DetachStream(&self) -> ::windows::core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DetachStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IInputStream>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IDataReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IDataReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataReader {}
impl ::core::fmt::Debug for IDataReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDataReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e2b50029-b4c1-4314-a4b8-fb813a2f275e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IDataReader {
    type Vtable = IDataReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IDataReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2b50029_b4c1_4314_a4b8_fb813a2f275e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UnconsumedBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UnicodeEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnicodeEncoding) -> ::windows::core::HRESULT,
    pub SetUnicodeEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UnicodeEncoding) -> ::windows::core::HRESULT,
    pub ByteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ByteOrder) -> ::windows::core::HRESULT,
    pub SetByteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ByteOrder) -> ::windows::core::HRESULT,
    pub InputStreamOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InputStreamOptions) -> ::windows::core::HRESULT,
    pub SetInputStreamOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InputStreamOptions) -> ::windows::core::HRESULT,
    pub ReadByte: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub ReadBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *mut u8) -> ::windows::core::HRESULT,
    pub ReadBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReadBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ReadGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ReadInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT,
    pub ReadInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ReadInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub ReadUInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ReadUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ReadUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub ReadSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub ReadDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub ReadString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codeunitcount: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadDateTime: usize,
    #[cfg(feature = "Foundation")]
    pub ReadTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadTimeSpan: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsync: usize,
    pub DetachBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DetachStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataReaderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDataReaderFactory {
    type Vtable = IDataReaderFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IDataReaderFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7527847_57da_4e15_914c_06806699a098);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReaderFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateDataReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataReaderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDataReaderStatics {
    type Vtable = IDataReaderStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IDataReaderStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11fcbfc8_f93a_471b_b121_f379e349313c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReaderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IDataWriter(::windows::core::IUnknown);
impl IDataWriter {
    pub fn UnstoredBufferLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnstoredBufferLength)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnicodeEncoding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<UnicodeEncoding>(result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetUnicodeEncoding)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ByteOrder)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ByteOrder>(result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetByteOrder)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteByte(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteByte)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteBytes(&self, value: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteBytes)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn WriteBuffer<'a, P0, E0>(&self, buffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteBuffer)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn WriteBufferRange<'a, P0, E0>(&self, buffer: P0, start: u32, count: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteBufferRange)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), start, count).ok() }
    }
    pub fn WriteBoolean(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteBoolean)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteGuid(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteGuid)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteInt16(&self, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteInt16)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteInt32(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteInt32)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteInt64(&self, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteInt64)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteUInt16(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteUInt16)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteUInt32(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteUInt32)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteUInt64(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteUInt64)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteSingle(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteSingle)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteDouble(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteDouble)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteDateTime(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteDateTime)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteTimeSpan(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteTimeSpan)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteString)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MeasureString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MeasureString)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StoreAsync(&self) -> ::windows::core::Result<DataWriterStoreOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StoreAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<DataWriterStoreOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlushAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn DetachBuffer(&self) -> ::windows::core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DetachBuffer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IBuffer>(result__)
        }
    }
    pub fn DetachStream(&self) -> ::windows::core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DetachStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IOutputStream>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IDataWriter, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IDataWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataWriter {}
impl ::core::fmt::Debug for IDataWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDataWriter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{64b89265-d341-4922-b38a-dd4af8808c4e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IDataWriter {
    type Vtable = IDataWriter_Vtbl;
}
unsafe impl ::windows::core::Interface for IDataWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64b89265_d341_4922_b38a_dd4af8808c4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataWriter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UnstoredBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UnicodeEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnicodeEncoding) -> ::windows::core::HRESULT,
    pub SetUnicodeEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UnicodeEncoding) -> ::windows::core::HRESULT,
    pub ByteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ByteOrder) -> ::windows::core::HRESULT,
    pub SetByteOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ByteOrder) -> ::windows::core::HRESULT,
    pub WriteByte: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub WriteBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT,
    pub WriteBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteBufferRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, start: u32, count: u32) -> ::windows::core::HRESULT,
    pub WriteBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub WriteGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub WriteInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub WriteInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub WriteInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT,
    pub WriteUInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT,
    pub WriteUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub WriteUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
    pub WriteSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub WriteDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WriteDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteDateTime: usize,
    #[cfg(feature = "Foundation")]
    pub WriteTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteTimeSpan: usize,
    pub WriteString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MeasureString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StoreAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FlushAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlushAsync: usize,
    pub DetachBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DetachStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataWriterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDataWriterFactory {
    type Vtable = IDataWriterFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IDataWriterFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x338c67c2_8b84_4c2b_9c50_7b8767847a1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataWriterFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateDataWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileRandomAccessStreamStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IFileRandomAccessStreamStatics {
    type Vtable = IFileRandomAccessStreamStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IFileRandomAccessStreamStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73550107_3b57_4b5d_8345_554d2fc621f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileRandomAccessStreamStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenTransactedWriteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenTransactedWriteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenTransactedWriteWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenTransactedWriteWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub OpenForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    OpenForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub OpenForUserWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    OpenForUserWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub OpenTransactedWriteForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    OpenTransactedWriteForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub OpenTransactedWriteForUserWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    OpenTransactedWriteForUserWithOptionsAsync: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IInputStream(::windows::core::IUnknown);
impl IInputStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, P0, E0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), count, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(IInputStream, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IInputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IInputStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IInputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IInputStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&IInputStream> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IInputStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IInputStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputStream {}
impl ::core::fmt::Debug for IInputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInputStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{905a0fe2-bc53-11df-8c49-001e4fc686da}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IInputStream {
    type Vtable = IInputStream_Vtbl;
}
unsafe impl ::windows::core::Interface for IInputStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe2_bc53_11df_8c49_001e4fc686da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputStream_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, count: u32, options: InputStreamOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadAsync: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IInputStreamReference(::windows::core::IUnknown);
impl IInputStreamReference {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenSequentialReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IInputStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenSequentialReadAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<IInputStream>>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IInputStreamReference, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IInputStreamReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInputStreamReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputStreamReference {}
impl ::core::fmt::Debug for IInputStreamReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputStreamReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInputStreamReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{43929d18-5ec9-4b5a-919c-4205b0c804b6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IInputStreamReference {
    type Vtable = IInputStreamReference_Vtbl;
}
unsafe impl ::windows::core::Interface for IInputStreamReference {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43929d18_5ec9_4b5a_919c_4205b0c804b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputStreamReference_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub OpenSequentialReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenSequentialReadAsync: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IOutputStream(::windows::core::IUnknown);
impl IOutputStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, P0, E0>(&self, buffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlushAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(IOutputStream, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IOutputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IOutputStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IOutputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IOutputStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&IOutputStream> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IOutputStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IOutputStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOutputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOutputStream {}
impl ::core::fmt::Debug for IOutputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOutputStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IOutputStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{905a0fe6-bc53-11df-8c49-001e4fc686da}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IOutputStream {
    type Vtable = IOutputStream_Vtbl;
}
unsafe impl ::windows::core::Interface for IOutputStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe6_bc53_11df_8c49_001e4fc686da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOutputStream_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub WriteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FlushAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlushAsync: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IPropertySetSerializer(::windows::core::IUnknown);
impl IPropertySetSerializer {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Serialize<'a, P0, E0>(&self, propertyset: P0) -> ::windows::core::Result<IBuffer>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IPropertySet>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Serialize)(::windows::core::Vtable::as_raw(this), propertyset.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Deserialize<'a, P0, E0, P1, E1>(&self, propertyset: P0, buffer: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IPropertySet>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Deserialize)(::windows::core::Vtable::as_raw(this), propertyset.try_into().map_err(|e| e.into())?.abi(), buffer.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
::windows::core::interface_hierarchy!(IPropertySetSerializer, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IPropertySetSerializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertySetSerializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySetSerializer {}
impl ::core::fmt::Debug for IPropertySetSerializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySetSerializer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPropertySetSerializer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6e8ebf1c-ef3d-4376-b20e-5be638aeac77}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPropertySetSerializer {
    type Vtable = IPropertySetSerializer_Vtbl;
}
unsafe impl ::windows::core::Interface for IPropertySetSerializer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e8ebf1c_ef3d_4376_b20e_5be638aeac77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySetSerializer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyset: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Serialize: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Deserialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyset: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Deserialize: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IRandomAccessStream(::windows::core::IUnknown);
impl IRandomAccessStream {
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi::<IInputStream>(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetOutputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi::<IOutputStream>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Seek)(::windows::core::Vtable::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows::core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IRandomAccessStream>(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanRead)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanWrite)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, P0, E0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), count, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, P0, E0>(&self, buffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlushAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IRandomAccessStream, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&IRandomAccessStream> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IRandomAccessStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRandomAccessStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IRandomAccessStream> for ::windows::core::InParam<'a, IInputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IRandomAccessStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRandomAccessStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IRandomAccessStream> for ::windows::core::InParam<'a, IOutputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IRandomAccessStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRandomAccessStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRandomAccessStream {}
impl ::core::fmt::Debug for IRandomAccessStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRandomAccessStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IRandomAccessStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{905a0fe1-bc53-11df-8c49-001e4fc686da}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IRandomAccessStream {
    type Vtable = IRandomAccessStream_Vtbl;
}
unsafe impl ::windows::core::Interface for IRandomAccessStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x905a0fe1_bc53_11df_8c49_001e4fc686da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStream_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
    pub GetInputStreamAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOutputStreamAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: u64) -> ::windows::core::HRESULT,
    pub CloneStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CanRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IRandomAccessStreamReference(::windows::core::IUnknown);
impl IRandomAccessStreamReference {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenReadAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IRandomAccessStreamReference, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IRandomAccessStreamReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRandomAccessStreamReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRandomAccessStreamReference {}
impl ::core::fmt::Debug for IRandomAccessStreamReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRandomAccessStreamReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IRandomAccessStreamReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{33ee3134-1dd6-4e3a-8067-d1c162e8642b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IRandomAccessStreamReference {
    type Vtable = IRandomAccessStreamReference_Vtbl;
}
unsafe impl ::windows::core::Interface for IRandomAccessStreamReference {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ee3134_1dd6_4e3a_8067_d1c162e8642b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamReference_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub OpenReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenReadAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRandomAccessStreamReferenceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRandomAccessStreamReferenceStatics {
    type Vtable = IRandomAccessStreamReferenceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRandomAccessStreamReferenceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x857309dc_3fbf_4e7d_986f_ef3b1a07a964);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamReferenceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateFromUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUri: usize,
    pub CreateFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRandomAccessStreamStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRandomAccessStreamStatics {
    type Vtable = IRandomAccessStreamStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRandomAccessStreamStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x524cedcf_6e29_4ce5_9573_6b753db66c3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CopyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CopySizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, bytestocopy: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopySizeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CopyAndCloseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CopyAndCloseAsync: usize,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct IRandomAccessStreamWithContentType(::windows::core::IUnknown);
impl IRandomAccessStreamWithContentType {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, P0, E0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), count, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, P0, E0>(&self, buffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlushAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<IInputStream> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi::<IInputStream>(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<IOutputStream> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetOutputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi::<IOutputStream>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Seek)(::windows::core::Vtable::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows::core::Result<IRandomAccessStream> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IRandomAccessStream>(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanRead)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanWrite)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IRandomAccessStreamWithContentType, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IRandomAccessStreamWithContentType> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IRandomAccessStreamWithContentType> for IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for ::windows::core::InParam<'a, IContentTypeProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IRandomAccessStreamWithContentType> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for ::windows::core::InParam<'a, IInputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IRandomAccessStreamWithContentType> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for ::windows::core::InParam<'a, IOutputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<IRandomAccessStreamWithContentType> for IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&IRandomAccessStreamWithContentType> for ::windows::core::InParam<'a, IRandomAccessStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRandomAccessStreamWithContentType) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IRandomAccessStreamWithContentType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRandomAccessStreamWithContentType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRandomAccessStreamWithContentType {}
impl ::core::fmt::Debug for IRandomAccessStreamWithContentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRandomAccessStreamWithContentType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IRandomAccessStreamWithContentType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cc254827-4b3d-438f-9232-10c76bc7e038}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IRandomAccessStreamWithContentType {
    type Vtable = IRandomAccessStreamWithContentType_Vtbl;
}
unsafe impl ::windows::core::Interface for IRandomAccessStreamWithContentType {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc254827_4b3d_438f_9232_10c76bc7e038);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamWithContentType_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct Buffer(::windows::core::IUnknown);
impl Buffer {
    pub fn Capacity(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Capacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLength)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Create(capacity: u32) -> ::windows::core::Result<Buffer> {
        Self::IBufferFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), capacity, result__.as_mut_ptr()).from_abi::<Buffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateCopyFromMemoryBuffer<'a, P0, E0>(input: P0) -> ::windows::core::Result<Buffer>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::IMemoryBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCopyFromMemoryBuffer)(::windows::core::Vtable::as_raw(this), input.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<Buffer>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateMemoryBufferOverIBuffer<'a, P0, E0>(input: P0) -> ::windows::core::Result<super::super::Foundation::MemoryBuffer>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IBufferStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMemoryBufferOverIBuffer)(::windows::core::Vtable::as_raw(this), input.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::MemoryBuffer>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBufferFactory<R, F: FnOnce(&IBufferFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Buffer, IBufferFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBufferStatics<R, F: FnOnce(&IBufferStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Buffer, IBufferStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Buffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Buffer {}
impl ::core::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Buffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Buffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.Buffer;{905a0fe0-bc53-11df-8c49-001e4fc686da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Buffer {
    type Vtable = IBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for Buffer {
    const IID: ::windows::core::GUID = <IBuffer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Buffer {
    const NAME: &'static str = "Windows.Storage.Streams.Buffer";
}
::windows::core::interface_hierarchy!(Buffer, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<Buffer> for IBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: Buffer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Buffer> for IBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &Buffer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&Buffer> for ::windows::core::InParam<'a, IBuffer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Buffer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Buffer {}
unsafe impl ::core::marker::Sync for Buffer {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct DataReader(::windows::core::IUnknown);
impl DataReader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn UnconsumedBufferLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnconsumedBufferLength)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnicodeEncoding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<UnicodeEncoding>(result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetUnicodeEncoding)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ByteOrder)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ByteOrder>(result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetByteOrder)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn InputStreamOptions(&self) -> ::windows::core::Result<InputStreamOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputStreamOptions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<InputStreamOptions>(result__)
        }
    }
    pub fn SetInputStreamOptions(&self, value: InputStreamOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetInputStreamOptions)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ReadByte(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadByte)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn ReadBytes(&self, value: &mut [u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ReadBytes)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_mut_ptr()).ok() }
    }
    pub fn ReadBuffer(&self, length: u32) -> ::windows::core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadBuffer)(::windows::core::Vtable::as_raw(this), length, result__.as_mut_ptr()).from_abi::<IBuffer>(result__)
        }
    }
    pub fn ReadBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadBoolean)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ReadGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadGuid)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn ReadInt16(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadInt16)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i16>(result__)
        }
    }
    pub fn ReadInt32(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadInt32)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ReadInt64(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadInt64)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i64>(result__)
        }
    }
    pub fn ReadUInt16(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadUInt16)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn ReadUInt32(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadUInt32)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ReadUInt64(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadUInt64)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn ReadSingle(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadSingle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn ReadDouble(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadDouble)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ReadString(&self, codeunitcount: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadString)(::windows::core::Vtable::as_raw(this), codeunitcount, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadDateTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadDateTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadTimeSpan(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadTimeSpan)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadAsync(&self, count: u32) -> ::windows::core::Result<DataReaderLoadOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadAsync)(::windows::core::Vtable::as_raw(this), count, result__.as_mut_ptr()).from_abi::<DataReaderLoadOperation>(result__)
        }
    }
    pub fn DetachBuffer(&self) -> ::windows::core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DetachBuffer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IBuffer>(result__)
        }
    }
    pub fn DetachStream(&self) -> ::windows::core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DetachStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IInputStream>(result__)
        }
    }
    pub fn CreateDataReader<'a, P0, E0>(inputstream: P0) -> ::windows::core::Result<DataReader>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IDataReaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateDataReader)(::windows::core::Vtable::as_raw(this), inputstream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<DataReader>(result__)
        })
    }
    pub fn FromBuffer<'a, P0, E0>(buffer: P0) -> ::windows::core::Result<DataReader>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IDataReaderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromBuffer)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<DataReader>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDataReaderFactory<R, F: FnOnce(&IDataReaderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DataReader, IDataReaderFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDataReaderStatics<R, F: FnOnce(&IDataReaderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DataReader, IDataReaderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for DataReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DataReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataReader {}
impl ::core::fmt::Debug for DataReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DataReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataReader;{e2b50029-b4c1-4314-a4b8-fb813a2f275e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DataReader {
    type Vtable = IDataReader_Vtbl;
}
unsafe impl ::windows::core::Interface for DataReader {
    const IID: ::windows::core::GUID = <IDataReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DataReader {
    const NAME: &'static str = "Windows.Storage.Streams.DataReader";
}
::windows::core::interface_hierarchy!(DataReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DataReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DataReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DataReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&DataReader> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataReader) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DataReader> for IDataReader {
    type Error = ::windows::core::Error;
    fn try_from(value: DataReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DataReader> for IDataReader {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DataReader> for ::windows::core::InParam<'a, IDataReader> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataReader) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DataReader {}
unsafe impl ::core::marker::Sync for DataReader {}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
#[repr(transparent)]
pub struct DataReaderLoadOperation(::windows::core::IUnknown);
#[cfg(feature = "Foundation")]
impl DataReaderLoadOperation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Cancel)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted(&self, handler: &super::super::Foundation::AsyncOperationCompletedHandler<u32>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::Foundation::AsyncOperationCompletedHandler<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Completed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::AsyncOperationCompletedHandler<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetResults)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for DataReaderLoadOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for DataReaderLoadOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for DataReaderLoadOperation {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for DataReaderLoadOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataReaderLoadOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for DataReaderLoadOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataReaderLoadOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};u4))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Vtable for DataReaderLoadOperation {
    type Vtable = super::super::Foundation::IAsyncOperation_Vtbl<u32>;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for DataReaderLoadOperation {
    const IID: ::windows::core::GUID = <super::super::Foundation::IAsyncOperation<u32> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for DataReaderLoadOperation {
    const NAME: &'static str = "Windows.Storage.Streams.DataReaderLoadOperation";
}
#[cfg(feature = "Foundation")]
impl DataReaderLoadOperation {
    pub fn get(&self) -> ::windows::core::Result<u32> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::core::Waiter::new()?;
            self.SetCompleted(&super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
impl ::std::future::Future for DataReaderLoadOperation {
    type Output = ::windows::core::Result<u32>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
::windows::core::interface_hierarchy!(DataReaderLoadOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DataReaderLoadOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: DataReaderLoadOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DataReaderLoadOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataReaderLoadOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&DataReaderLoadOperation> for ::windows::core::InParam<'a, super::super::Foundation::IAsyncInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataReaderLoadOperation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DataReaderLoadOperation> for super::super::Foundation::IAsyncOperation<u32> {
    type Error = ::windows::core::Error;
    fn try_from(value: DataReaderLoadOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DataReaderLoadOperation> for super::super::Foundation::IAsyncOperation<u32> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataReaderLoadOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&DataReaderLoadOperation> for ::windows::core::InParam<'a, super::super::Foundation::IAsyncOperation<u32>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataReaderLoadOperation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for DataReaderLoadOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for DataReaderLoadOperation {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct DataWriter(::windows::core::IUnknown);
impl DataWriter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DataWriter, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn UnstoredBufferLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnstoredBufferLength)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnicodeEncoding)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<UnicodeEncoding>(result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetUnicodeEncoding)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ByteOrder)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<ByteOrder>(result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetByteOrder)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteByte(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteByte)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteBytes(&self, value: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteBytes)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn WriteBuffer<'a, P0, E0>(&self, buffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteBuffer)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn WriteBufferRange<'a, P0, E0>(&self, buffer: P0, start: u32, count: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteBufferRange)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), start, count).ok() }
    }
    pub fn WriteBoolean(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteBoolean)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteGuid(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteGuid)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteInt16(&self, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteInt16)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteInt32(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteInt32)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteInt64(&self, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteInt64)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteUInt16(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteUInt16)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteUInt32(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteUInt32)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteUInt64(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteUInt64)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteSingle(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteSingle)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteDouble(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteDouble)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteDateTime(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteDateTime)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteTimeSpan(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).WriteTimeSpan)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn WriteString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteString)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn MeasureString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MeasureString)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StoreAsync(&self) -> ::windows::core::Result<DataWriterStoreOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StoreAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<DataWriterStoreOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlushAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn DetachBuffer(&self) -> ::windows::core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DetachBuffer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IBuffer>(result__)
        }
    }
    pub fn DetachStream(&self) -> ::windows::core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DetachStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IOutputStream>(result__)
        }
    }
    pub fn CreateDataWriter<'a, P0, E0>(outputstream: P0) -> ::windows::core::Result<DataWriter>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IOutputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IDataWriterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateDataWriter)(::windows::core::Vtable::as_raw(this), outputstream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<DataWriter>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDataWriterFactory<R, F: FnOnce(&IDataWriterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DataWriter, IDataWriterFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for DataWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataWriter {}
impl ::core::fmt::Debug for DataWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DataWriter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataWriter;{64b89265-d341-4922-b38a-dd4af8808c4e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DataWriter {
    type Vtable = IDataWriter_Vtbl;
}
unsafe impl ::windows::core::Interface for DataWriter {
    const IID: ::windows::core::GUID = <IDataWriter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DataWriter {
    const NAME: &'static str = "Windows.Storage.Streams.DataWriter";
}
::windows::core::interface_hierarchy!(DataWriter, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DataWriter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DataWriter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DataWriter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataWriter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&DataWriter> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataWriter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<DataWriter> for IDataWriter {
    type Error = ::windows::core::Error;
    fn try_from(value: DataWriter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DataWriter> for IDataWriter {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataWriter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&DataWriter> for ::windows::core::InParam<'a, IDataWriter> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataWriter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DataWriter {}
unsafe impl ::core::marker::Sync for DataWriter {}
#[doc = "*Required features: `\"Storage_Streams\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
#[repr(transparent)]
pub struct DataWriterStoreOperation(::windows::core::IUnknown);
#[cfg(feature = "Foundation")]
impl DataWriterStoreOperation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Cancel)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted(&self, handler: &super::super::Foundation::AsyncOperationCompletedHandler<u32>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::Foundation::AsyncOperationCompletedHandler<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Completed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::AsyncOperationCompletedHandler<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetResults)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for DataWriterStoreOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for DataWriterStoreOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for DataWriterStoreOperation {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for DataWriterStoreOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataWriterStoreOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for DataWriterStoreOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.DataWriterStoreOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};u4))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Vtable for DataWriterStoreOperation {
    type Vtable = super::super::Foundation::IAsyncOperation_Vtbl<u32>;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for DataWriterStoreOperation {
    const IID: ::windows::core::GUID = <super::super::Foundation::IAsyncOperation<u32> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for DataWriterStoreOperation {
    const NAME: &'static str = "Windows.Storage.Streams.DataWriterStoreOperation";
}
#[cfg(feature = "Foundation")]
impl DataWriterStoreOperation {
    pub fn get(&self) -> ::windows::core::Result<u32> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::core::Waiter::new()?;
            self.SetCompleted(&super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
impl ::std::future::Future for DataWriterStoreOperation {
    type Output = ::windows::core::Result<u32>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
::windows::core::interface_hierarchy!(DataWriterStoreOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DataWriterStoreOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: DataWriterStoreOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DataWriterStoreOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataWriterStoreOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&DataWriterStoreOperation> for ::windows::core::InParam<'a, super::super::Foundation::IAsyncInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataWriterStoreOperation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DataWriterStoreOperation> for super::super::Foundation::IAsyncOperation<u32> {
    type Error = ::windows::core::Error;
    fn try_from(value: DataWriterStoreOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DataWriterStoreOperation> for super::super::Foundation::IAsyncOperation<u32> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataWriterStoreOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&DataWriterStoreOperation> for ::windows::core::InParam<'a, super::super::Foundation::IAsyncOperation<u32>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DataWriterStoreOperation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for DataWriterStoreOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for DataWriterStoreOperation {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct FileInputStream(::windows::core::IUnknown);
impl FileInputStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, P0, E0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), count, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for FileInputStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileInputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileInputStream {}
impl ::core::fmt::Debug for FileInputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileInputStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileInputStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.FileInputStream;{905a0fe2-bc53-11df-8c49-001e4fc686da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for FileInputStream {
    type Vtable = IInputStream_Vtbl;
}
unsafe impl ::windows::core::Interface for FileInputStream {
    const IID: ::windows::core::GUID = <IInputStream as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileInputStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileInputStream";
}
::windows::core::interface_hierarchy!(FileInputStream, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<FileInputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInputStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&FileInputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInputStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&FileInputStream> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInputStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileInputStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInputStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInputStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInputStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileInputStream> for ::windows::core::InParam<'a, IInputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInputStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for FileInputStream {}
unsafe impl ::core::marker::Sync for FileInputStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct FileOutputStream(::windows::core::IUnknown);
impl FileOutputStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, P0, E0>(&self, buffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlushAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for FileOutputStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileOutputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileOutputStream {}
impl ::core::fmt::Debug for FileOutputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOutputStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileOutputStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.FileOutputStream;{905a0fe6-bc53-11df-8c49-001e4fc686da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for FileOutputStream {
    type Vtable = IOutputStream_Vtbl;
}
unsafe impl ::windows::core::Interface for FileOutputStream {
    const IID: ::windows::core::GUID = <IOutputStream as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileOutputStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileOutputStream";
}
::windows::core::interface_hierarchy!(FileOutputStream, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<FileOutputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOutputStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&FileOutputStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOutputStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&FileOutputStream> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOutputStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileOutputStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOutputStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOutputStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOutputStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileOutputStream> for ::windows::core::InParam<'a, IOutputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOutputStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for FileOutputStream {}
unsafe impl ::core::marker::Sync for FileOutputStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct FileRandomAccessStream(::windows::core::IUnknown);
impl FileRandomAccessStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(filepath), accessmode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenWithOptionsAsync(filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenWithOptionsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(filepath), accessmode, sharingoptions, opendisposition, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteAsync(filepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenTransactedWriteAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(filepath), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteWithOptionsAsync(filepath: &::windows::core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenTransactedWriteWithOptionsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(filepath), openoptions, opendisposition, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn OpenForUserAsync(user: &super::super::System::User, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(filepath), accessmode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn OpenForUserWithOptionsAsync(user: &super::super::System::User, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenForUserWithOptionsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(filepath), accessmode, sharingoptions, opendisposition, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn OpenTransactedWriteForUserAsync(user: &super::super::System::User, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenTransactedWriteForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(filepath), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn OpenTransactedWriteForUserWithOptionsAsync(user: &super::super::System::User, filepath: &::windows::core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenTransactedWriteForUserWithOptionsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(filepath), openoptions, opendisposition, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, P0, E0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), count, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, P0, E0>(&self, buffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlushAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi::<IInputStream>(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetOutputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi::<IOutputStream>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Seek)(::windows::core::Vtable::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows::core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IRandomAccessStream>(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanRead)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanWrite)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IFileRandomAccessStreamStatics<R, F: FnOnce(&IFileRandomAccessStreamStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FileRandomAccessStream, IFileRandomAccessStreamStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for FileRandomAccessStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileRandomAccessStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileRandomAccessStream {}
impl ::core::fmt::Debug for FileRandomAccessStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileRandomAccessStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileRandomAccessStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.FileRandomAccessStream;{905a0fe1-bc53-11df-8c49-001e4fc686da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for FileRandomAccessStream {
    type Vtable = IRandomAccessStream_Vtbl;
}
unsafe impl ::windows::core::Interface for FileRandomAccessStream {
    const IID: ::windows::core::GUID = <IRandomAccessStream as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileRandomAccessStream";
}
::windows::core::interface_hierarchy!(FileRandomAccessStream, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<FileRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&FileRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&FileRandomAccessStream> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileRandomAccessStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileRandomAccessStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileRandomAccessStream> for ::windows::core::InParam<'a, IInputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileRandomAccessStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileRandomAccessStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileRandomAccessStream> for ::windows::core::InParam<'a, IOutputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileRandomAccessStream> for IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileRandomAccessStream> for IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileRandomAccessStream> for ::windows::core::InParam<'a, IRandomAccessStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileRandomAccessStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for FileRandomAccessStream {}
unsafe impl ::core::marker::Sync for FileRandomAccessStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct InMemoryRandomAccessStream(::windows::core::IUnknown);
impl InMemoryRandomAccessStream {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InMemoryRandomAccessStream, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, P0, E0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), count, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, P0, E0>(&self, buffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlushAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi::<IInputStream>(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetOutputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi::<IOutputStream>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Seek)(::windows::core::Vtable::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows::core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IRandomAccessStream>(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanRead)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanWrite)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for InMemoryRandomAccessStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InMemoryRandomAccessStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InMemoryRandomAccessStream {}
impl ::core::fmt::Debug for InMemoryRandomAccessStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InMemoryRandomAccessStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InMemoryRandomAccessStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.InMemoryRandomAccessStream;{905a0fe1-bc53-11df-8c49-001e4fc686da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InMemoryRandomAccessStream {
    type Vtable = IRandomAccessStream_Vtbl;
}
unsafe impl ::windows::core::Interface for InMemoryRandomAccessStream {
    const IID: ::windows::core::GUID = <IRandomAccessStream as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InMemoryRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.InMemoryRandomAccessStream";
}
::windows::core::interface_hierarchy!(InMemoryRandomAccessStream, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InMemoryRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InMemoryRandomAccessStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&InMemoryRandomAccessStream> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<InMemoryRandomAccessStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InMemoryRandomAccessStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&InMemoryRandomAccessStream> for ::windows::core::InParam<'a, IInputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<InMemoryRandomAccessStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InMemoryRandomAccessStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&InMemoryRandomAccessStream> for ::windows::core::InParam<'a, IOutputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<InMemoryRandomAccessStream> for IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InMemoryRandomAccessStream> for IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&InMemoryRandomAccessStream> for ::windows::core::InParam<'a, IRandomAccessStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InMemoryRandomAccessStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for InMemoryRandomAccessStream {}
unsafe impl ::core::marker::Sync for InMemoryRandomAccessStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct InputStreamOverStream(::windows::core::IUnknown);
impl InputStreamOverStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, P0, E0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), count, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
}
impl ::core::clone::Clone for InputStreamOverStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputStreamOverStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputStreamOverStream {}
impl ::core::fmt::Debug for InputStreamOverStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputStreamOverStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputStreamOverStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.InputStreamOverStream;{905a0fe2-bc53-11df-8c49-001e4fc686da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InputStreamOverStream {
    type Vtable = IInputStream_Vtbl;
}
unsafe impl ::windows::core::Interface for InputStreamOverStream {
    const IID: ::windows::core::GUID = <IInputStream as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.InputStreamOverStream";
}
::windows::core::interface_hierarchy!(InputStreamOverStream, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<InputStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: InputStreamOverStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&InputStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &InputStreamOverStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&InputStreamOverStream> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InputStreamOverStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<InputStreamOverStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: InputStreamOverStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InputStreamOverStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &InputStreamOverStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&InputStreamOverStream> for ::windows::core::InParam<'a, IInputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &InputStreamOverStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for InputStreamOverStream {}
unsafe impl ::core::marker::Sync for InputStreamOverStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct OutputStreamOverStream(::windows::core::IUnknown);
impl OutputStreamOverStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, P0, E0>(&self, buffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlushAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for OutputStreamOverStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OutputStreamOverStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OutputStreamOverStream {}
impl ::core::fmt::Debug for OutputStreamOverStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OutputStreamOverStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OutputStreamOverStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.OutputStreamOverStream;{905a0fe6-bc53-11df-8c49-001e4fc686da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for OutputStreamOverStream {
    type Vtable = IOutputStream_Vtbl;
}
unsafe impl ::windows::core::Interface for OutputStreamOverStream {
    const IID: ::windows::core::GUID = <IOutputStream as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for OutputStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.OutputStreamOverStream";
}
::windows::core::interface_hierarchy!(OutputStreamOverStream, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<OutputStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: OutputStreamOverStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&OutputStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &OutputStreamOverStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&OutputStreamOverStream> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &OutputStreamOverStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<OutputStreamOverStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: OutputStreamOverStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&OutputStreamOverStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &OutputStreamOverStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&OutputStreamOverStream> for ::windows::core::InParam<'a, IOutputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &OutputStreamOverStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for OutputStreamOverStream {}
unsafe impl ::core::marker::Sync for OutputStreamOverStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
pub struct RandomAccessStream;
impl RandomAccessStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyAsync<'a, P0, E0, P1, E1>(source: P0, destination: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IOutputStream>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CopyAsync)(::windows::core::Vtable::as_raw(this), source.try_into().map_err(|e| e.into())?.abi(), destination.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopySizeAsync<'a, P0, E0, P1, E1>(source: P0, destination: P1, bytestocopy: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IOutputStream>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CopySizeAsync)(::windows::core::Vtable::as_raw(this), source.try_into().map_err(|e| e.into())?.abi(), destination.try_into().map_err(|e| e.into())?.abi(), bytestocopy, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyAndCloseAsync<'a, P0, E0, P1, E1>(source: P0, destination: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IOutputStream>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CopyAndCloseAsync)(::windows::core::Vtable::as_raw(this), source.try_into().map_err(|e| e.into())?.abi(), destination.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRandomAccessStreamStatics<R, F: FnOnce(&IRandomAccessStreamStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RandomAccessStream, IRandomAccessStreamStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for RandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStream";
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct RandomAccessStreamOverStream(::windows::core::IUnknown);
impl RandomAccessStreamOverStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadAsync<'a, P0, E0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), count, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WriteAsync<'a, P0, E0>(&self, buffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlushAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi::<IInputStream>(result__)
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetOutputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi::<IOutputStream>(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Seek)(::windows::core::Vtable::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> ::windows::core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<IRandomAccessStream>(result__)
        }
    }
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanRead)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanWrite)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for RandomAccessStreamOverStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RandomAccessStreamOverStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RandomAccessStreamOverStream {}
impl ::core::fmt::Debug for RandomAccessStreamOverStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RandomAccessStreamOverStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RandomAccessStreamOverStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.RandomAccessStreamOverStream;{905a0fe1-bc53-11df-8c49-001e4fc686da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RandomAccessStreamOverStream {
    type Vtable = IRandomAccessStream_Vtbl;
}
unsafe impl ::windows::core::Interface for RandomAccessStreamOverStream {
    const IID: ::windows::core::GUID = <IRandomAccessStream as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RandomAccessStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamOverStream";
}
::windows::core::interface_hierarchy!(RandomAccessStreamOverStream, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<RandomAccessStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&RandomAccessStreamOverStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&RandomAccessStreamOverStream> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<RandomAccessStreamOverStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RandomAccessStreamOverStream> for IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&RandomAccessStreamOverStream> for ::windows::core::InParam<'a, IInputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<RandomAccessStreamOverStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RandomAccessStreamOverStream> for IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&RandomAccessStreamOverStream> for ::windows::core::InParam<'a, IOutputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<RandomAccessStreamOverStream> for IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RandomAccessStreamOverStream> for IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&RandomAccessStreamOverStream> for ::windows::core::InParam<'a, IRandomAccessStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RandomAccessStreamOverStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for RandomAccessStreamOverStream {}
unsafe impl ::core::marker::Sync for RandomAccessStreamOverStream {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
pub struct RandomAccessStreamReference(::windows::core::IUnknown);
impl RandomAccessStreamReference {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenReadAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>>(result__)
        }
    }
    pub fn CreateFromFile<'a, P0, E0>(file: P0) -> ::windows::core::Result<RandomAccessStreamReference>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromFile)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<RandomAccessStreamReference>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUri(uri: &super::super::Foundation::Uri) -> ::windows::core::Result<RandomAccessStreamReference> {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi::<RandomAccessStreamReference>(result__)
        })
    }
    pub fn CreateFromStream<'a, P0, E0>(stream: P0) -> ::windows::core::Result<RandomAccessStreamReference>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromStream)(::windows::core::Vtable::as_raw(this), stream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<RandomAccessStreamReference>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRandomAccessStreamReferenceStatics<R, F: FnOnce(&IRandomAccessStreamReferenceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RandomAccessStreamReference, IRandomAccessStreamReferenceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RandomAccessStreamReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RandomAccessStreamReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RandomAccessStreamReference {}
impl ::core::fmt::Debug for RandomAccessStreamReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RandomAccessStreamReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RandomAccessStreamReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Streams.RandomAccessStreamReference;{33ee3134-1dd6-4e3a-8067-d1c162e8642b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RandomAccessStreamReference {
    type Vtable = IRandomAccessStreamReference_Vtbl;
}
unsafe impl ::windows::core::Interface for RandomAccessStreamReference {
    const IID: ::windows::core::GUID = <IRandomAccessStreamReference as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RandomAccessStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamReference";
}
::windows::core::interface_hierarchy!(RandomAccessStreamReference, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<RandomAccessStreamReference> for IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: RandomAccessStreamReference) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RandomAccessStreamReference> for IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: &RandomAccessStreamReference) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&RandomAccessStreamReference> for ::windows::core::InParam<'a, IRandomAccessStreamReference> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RandomAccessStreamReference) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for RandomAccessStreamReference {}
unsafe impl ::core::marker::Sync for RandomAccessStreamReference {}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ByteOrder(pub i32);
impl ByteOrder {
    pub const LittleEndian: Self = Self(0i32);
    pub const BigEndian: Self = Self(1i32);
}
impl ::core::marker::Copy for ByteOrder {}
impl ::core::clone::Clone for ByteOrder {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ByteOrder {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ByteOrder {
    type Abi = Self;
}
impl ::core::fmt::Debug for ByteOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ByteOrder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ByteOrder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.ByteOrder;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FileOpenDisposition(pub i32);
impl FileOpenDisposition {
    pub const OpenExisting: Self = Self(0i32);
    pub const OpenAlways: Self = Self(1i32);
    pub const CreateNew: Self = Self(2i32);
    pub const CreateAlways: Self = Self(3i32);
    pub const TruncateExisting: Self = Self(4i32);
}
impl ::core::marker::Copy for FileOpenDisposition {}
impl ::core::clone::Clone for FileOpenDisposition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FileOpenDisposition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FileOpenDisposition {
    type Abi = Self;
}
impl ::core::fmt::Debug for FileOpenDisposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOpenDisposition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileOpenDisposition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.FileOpenDisposition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InputStreamOptions(pub u32);
impl InputStreamOptions {
    pub const None: Self = Self(0u32);
    pub const Partial: Self = Self(1u32);
    pub const ReadAhead: Self = Self(2u32);
}
impl ::core::marker::Copy for InputStreamOptions {}
impl ::core::clone::Clone for InputStreamOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InputStreamOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for InputStreamOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for InputStreamOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputStreamOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InputStreamOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InputStreamOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InputStreamOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InputStreamOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InputStreamOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for InputStreamOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.InputStreamOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Streams\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UnicodeEncoding(pub i32);
impl UnicodeEncoding {
    pub const Utf8: Self = Self(0i32);
    pub const Utf16LE: Self = Self(1i32);
    pub const Utf16BE: Self = Self(2i32);
}
impl ::core::marker::Copy for UnicodeEncoding {}
impl ::core::clone::Clone for UnicodeEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnicodeEncoding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnicodeEncoding {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnicodeEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnicodeEncoding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnicodeEncoding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.UnicodeEncoding;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
