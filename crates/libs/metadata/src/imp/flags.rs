#[repr(transparent)]
#[derive(Default)]
pub struct AssemblyFlags(pub u32);

#[repr(transparent)]
#[derive(Default)]
pub struct FieldAttributes(pub u16);

#[repr(transparent)]
#[derive(Default)]
pub struct MethodAttributes(pub u16);

#[repr(transparent)]
#[derive(Default)]
pub struct MethodImplAttributes(pub u16);

#[repr(transparent)]
#[derive(Default)]
pub struct ParamAttributes(pub u16);

#[repr(transparent)]
#[derive(Default)]
pub struct PInvokeAttributes(pub u16);

#[repr(transparent)]
#[derive(Default)]
pub struct TypeAttributes(pub u32);

#[repr(transparent)]
#[derive(Default)]
pub struct GenericParamAttributes(pub u16);

#[repr(transparent)]
#[derive(Default)]
pub struct PropertyAttributes(pub u16);

impl FieldAttributes {
    pub fn public(&self) -> bool {
        self.0 & 0x6 != 0
    }
    pub fn set_public(&mut self) {
        self.0 |= 0x6;
    }
    pub fn literal(&self) -> bool {
        self.0 & 0x40 != 0
    }
    pub fn set_literal(&mut self) {
        self.0 |= 0x40;
    }
    pub fn r#static(&self) -> bool {
        self.0 & 0x10 != 0
    }
    pub fn set_static(&mut self) {
        self.0 |= 0x10;
    }
}

impl MethodAttributes {
    pub fn special(&self) -> bool {
        self.0 & 0x800 != 0
    }
}

impl MethodImplAttributes {
    pub fn preserve_sig(&self) -> bool {
        self.0 & 0x80 != 0
    }
}

impl ParamAttributes {
    pub fn input(&self) -> bool {
        self.0 & 0x1 != 0
    }

    pub fn output(&self) -> bool {
        self.0 & 0x2 != 0
    }

    pub fn optional(&self) -> bool {
        self.0 & 0x10 != 0
    }
}

impl PInvokeAttributes {
    pub fn last_error(&self) -> bool {
        self.0 & 0x40 != 0
    }
    pub fn conv_platform(&self) -> bool {
        self.0 & 0x100 != 0
    }
    pub fn conv_cdecl(&self) -> bool {
        self.0 & 0x200 != 0
    }
    pub fn conv_stdcall(&self) -> bool {
        self.0 & 0x300 != 0
    }
    pub fn conv_thiscall(&self) -> bool {
        self.0 & 0x400 != 0
    }
    pub fn conv_fastcall(&self) -> bool {
        self.0 & 0x500 != 0
    }
}

impl TypeAttributes {
    pub fn public(&self) -> bool {
        self.0 & 0x1 != 0
    }
    pub fn set_public(&mut self) {
        self.0 |= 0x1;
    }

    pub fn explicit_layout(&self) -> bool {
        self.0 & 0x10 != 0
    }
    pub fn set_explicit_layout(&mut self) {
        self.0 |= 0x10;
    }

    pub fn get_abstract(&self) -> bool {
        self.0 & 0x80 != 0
    }
    pub fn set_abstract(&mut self) {
        self.0 |= 0x80;
    }

    pub fn winrt(&self) -> bool {
        self.0 & 0x4000 != 0
    }
    pub fn set_winrt(&mut self) {
        self.0 |= 0x4000;
    }

    pub fn interface(&self) -> bool {
        self.0 & 0x20 != 0
    }
    pub fn set_interface(&mut self) {
        self.0 |= 0x20
    }
}