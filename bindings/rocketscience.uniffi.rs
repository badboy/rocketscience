// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!

// Everybody gets basic buffer support, since it's needed for passing complex types over the FFI.

/// This helper allocates a new byte buffer owned by the Rust code, and returns it
/// to the foreign-language code as a `RustBuffer` struct. Callers must eventually
/// free the resulting buffer, either by explicitly calling the destructor defined below,
/// or by passing ownership of the buffer back into Rust code.
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn ffi_rocketscience_3a9e_rustbuffer_alloc(
    size: i32,
    err: &mut uniffi::deps::ffi_support::ExternError,
) -> uniffi::RustBuffer {
    uniffi::deps::ffi_support::call_with_output(err, || {
        uniffi::RustBuffer::new_with_size(size.max(0) as usize)
    })
}

/// This helper copies bytes owned by the foreign-language code into a new byte buffer owned
/// by the Rust code, and returns it as a `RustBuffer` struct. Callers must eventually
/// free the resulting buffer, either by explicitly calling the destructor defined below,
/// or by passing ownership of the buffer back into Rust code.
///
/// # Safety
/// This function will dereference a provided pointer in order to copy bytes from it, so
/// make sure the `ForeignBytes` struct contains a valid pointer and length.
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_rocketscience_3a9e_rustbuffer_from_bytes(
    bytes: uniffi::ForeignBytes,
    err: &mut uniffi::deps::ffi_support::ExternError,
) -> uniffi::RustBuffer {
    uniffi::deps::ffi_support::call_with_output(err, || {
        let bytes = bytes.as_slice();
        uniffi::RustBuffer::from_vec(bytes.to_vec())
    })
}

/// Free a byte buffer that had previously been passed to the foreign language code.
///
/// # Safety
/// The argument *must* be a uniquely-owned `RustBuffer` previously obtained from a call
/// into the Rust code that returned a buffer, or you'll risk freeing unowned memory or
/// corrupting the allocator state.
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_rocketscience_3a9e_rustbuffer_free(
    buf: uniffi::RustBuffer,
    err: &mut uniffi::deps::ffi_support::ExternError,
) {
    uniffi::deps::ffi_support::call_with_output(err, || uniffi::RustBuffer::destroy(buf))
}

/// Reserve additional capacity in a byte buffer that had previously been passed to the
/// foreign language code.
///
/// The first argument *must* be a uniquely-owned `RustBuffer` previously
/// obtained from a call into the Rust code that returned a buffer. Its underlying data pointer
/// will be reallocated if necessary and returned in a new `RustBuffer` struct.
///
/// The second argument must be the minimum number of *additional* bytes to reserve
/// capacity for in the buffer; it is likely to reserve additional capacity in practice
/// due to amortized growth strategy of Rust vectors.
///
/// # Safety
/// The first argument *must* be a uniquely-owned `RustBuffer` previously obtained from a call
/// into the Rust code that returned a buffer, or you'll risk freeing unowned memory or
/// corrupting the allocator state.
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_rocketscience_3a9e_rustbuffer_reserve(
    buf: uniffi::RustBuffer,
    additional: i32,
    err: &mut uniffi::deps::ffi_support::ExternError,
) -> uniffi::RustBuffer {
    uniffi::deps::ffi_support::call_with_output(err, || {
        use std::convert::TryInto;
        let additional: usize = additional
            .try_into()
            .expect("additional buffer length negative or overflowed");
        let mut v = buf.destroy_into_vec();
        v.reserve(additional);
        uniffi::RustBuffer::from_vec(v)
    })
}

/// Free a String that had previously been passed to the foreign language code.
///
/// # Safety
///
/// In order to free the string, Rust takes ownership of a raw pointer which is an
/// unsafe operation. The argument *must* be a uniquely-owned pointer previously
/// obtained from a call into the rust code that returned a string.
/// (In practice that means you got it from the `message` field of an `ExternError`,
/// because that's currently the only place we use `char*` types in our API).
#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn ffi_rocketscience_3a9e_string_free(
    cstr: *mut std::os::raw::c_char,
    err: &mut uniffi::deps::ffi_support::ExternError,
) {
    uniffi::deps::ffi_support::call_with_output(err, || {
        uniffi::deps::ffi_support::destroy_c_string(cstr)
    })
}

// We generate error mappings into ffi_support::ExternErrors
// so that the errors can propagate through the FFI

#[doc(hidden)]
impl From<LaunchError> for uniffi::deps::ffi_support::ExternError {
    fn from(err: LaunchError) -> uniffi::deps::ffi_support::ExternError {
        // Errno just differentiate between the errors.
        // They are in-order, i.e the first variant of the enum has code 1
        // As we add support for generic errors (e.g panics)
        // we might find that we need to reserve some codes.
        match err {
            LaunchError::RocketLaunch { .. } => uniffi::deps::ffi_support::ExternError::new_error(
                uniffi::deps::ffi_support::ErrorCode::new(1),
                err.to_string(),
            ),
        }
    }
}

// Enum defitions, corresponding to `enum` in UDL.

#[doc(hidden)]
unsafe impl uniffi::ViaFfi for Direction {
    type FfiType = uniffi::RustBuffer;

    fn lower(self) -> Self::FfiType {
        uniffi::lower_into_buffer(self)
    }

    fn try_lift(v: Self::FfiType) -> uniffi::deps::anyhow::Result<Self> {
        uniffi::try_lift_from_buffer(v)
    }

    fn write<B: uniffi::deps::bytes::BufMut>(&self, buf: &mut B) {
        match self {
            Direction::Up {} => {
                buf.put_i32(1);
            }
            Direction::Down {} => {
                buf.put_i32(2);
            }
        };
    }

    fn try_read<B: uniffi::deps::bytes::Buf>(buf: &mut B) -> uniffi::deps::anyhow::Result<Self> {
        uniffi::check_remaining(buf, 4)?;
        Ok(match buf.get_i32() {
            1 => Direction::Up,
            2 => Direction::Down,
            v => uniffi::deps::anyhow::bail!("Invalid Direction enum value: {}", v),
        })
    }
}

// Record definitions, implemented as method-less structs, corresponding to `dictionary` objects.

#[doc(hidden)]
unsafe impl uniffi::ViaFfi for Part {
    type FfiType = uniffi::RustBuffer;

    fn lower(self) -> Self::FfiType {
        uniffi::lower_into_buffer(self)
    }

    fn try_lift(v: Self::FfiType) -> uniffi::deps::anyhow::Result<Self> {
        uniffi::try_lift_from_buffer(v)
    }

    fn write<B: uniffi::deps::bytes::BufMut>(&self, buf: &mut B) {
        // If the provided struct doesn't match the fields declared in the UDL, then
        // the generated code here will fail to compile with somewhat helpful error.
        uniffi::ViaFfi::write(&self.name, buf);
        uniffi::ViaFfi::write(&self.cost, buf);
        uniffi::ViaFfi::write(&self.weight, buf);
    }

    fn try_read<B: uniffi::deps::bytes::Buf>(buf: &mut B) -> uniffi::deps::anyhow::Result<Self> {
        Ok(Self {
            name: <String as uniffi::ViaFfi>::try_read(buf)?,
            cost: <u64 as uniffi::ViaFfi>::try_read(buf)?,
            weight: <u64 as uniffi::ViaFfi>::try_read(buf)?,
        })
    }
}

// Top level functions, corresponding to UDL `namespace` functions.// Object definitions, correspoding to UDL `interface` definitions.

// For each Object definition, we assume the caller has provided an appropriately-shaped `struct`
// with an `impl` for each method on the object. We create a `ConcurrentHandleMap` for safely handing
// out references to these structs to foreign language code, and we provide a `pub extern "C"` function
// corresponding to each method.
//
// If the caller's implementation of the struct does not match with the methods or types specified
// in the UDL, then the rust compiler will complain with a (hopefully at least somewhat helpful!)
// error message when processing this generated code.
uniffi::deps::lazy_static::lazy_static! {
    #[doc(hidden)]
    static ref UNIFFI_HANDLE_MAP_ROCKET: uniffi::ffi::handle_maps::MutexHandleMap<Rocket>
        = Default::default();
}

#[doc(hidden)]
#[no_mangle]
pub extern "C" fn ffi_rocketscience_3a9e_Rocket_object_free(handle: u64) {
    let _ = UNIFFI_HANDLE_MAP_ROCKET.delete_u64(handle);
}
#[allow(clippy::all)]
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn rocketscience_3a9e_Rocket_new(
    name: uniffi::RustBuffer,
    err: &mut uniffi::deps::ffi_support::ExternError,
) -> u64 {
    uniffi::deps::log::debug!("rocketscience_3a9e_Rocket_new");
    // If the constructor does not have the same signature as declared in the UDL, then
    // this attempt to call it will fail with a (somewhat) helpful compiler error.

    UNIFFI_HANDLE_MAP_ROCKET.insert_with_output(err, || {
        Rocket::new(<String as uniffi::ViaFfi>::try_lift(name).unwrap())
    })
}
#[allow(clippy::all)]
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn rocketscience_3a9e_Rocket_show(
    handle: u64,
    err: &mut uniffi::deps::ffi_support::ExternError,
) -> uniffi::RustBuffer {
    uniffi::deps::log::debug!("rocketscience_3a9e_Rocket_show");
    // If the method does not have the same signature as declared in the UDL, then
    // this attempt to call it will fail with a (somewhat) helpful compiler error.
    use uniffi::UniffiMethodCall;
    UNIFFI_HANDLE_MAP_ROCKET.method_call_with_output(err, handle, |obj| {
        let _retval = Rocket::show(obj);
        <String as uniffi::ViaFfi>::lower(_retval)
    })
}

#[allow(clippy::all)]
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn rocketscience_3a9e_Rocket_launch(
    handle: u64,
    err: &mut uniffi::deps::ffi_support::ExternError,
) -> i8 {
    uniffi::deps::log::debug!("rocketscience_3a9e_Rocket_launch");
    // If the method does not have the same signature as declared in the UDL, then
    // this attempt to call it will fail with a (somewhat) helpful compiler error.
    use uniffi::UniffiMethodCall;
    UNIFFI_HANDLE_MAP_ROCKET.method_call_with_result(
        err,
        handle,
        |obj| -> Result<i8, LaunchError> {
            let _retval = Rocket::launch(obj)?;
            Ok(<bool as uniffi::ViaFfi>::lower(_retval))
        },
    )
}

#[allow(clippy::all)]
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn rocketscience_3a9e_Rocket_add(
    handle: u64,
    part: uniffi::RustBuffer,
    err: &mut uniffi::deps::ffi_support::ExternError,
) -> () {
    uniffi::deps::log::debug!("rocketscience_3a9e_Rocket_add");
    // If the method does not have the same signature as declared in the UDL, then
    // this attempt to call it will fail with a (somewhat) helpful compiler error.
    use uniffi::UniffiMethodCall;
    UNIFFI_HANDLE_MAP_ROCKET.method_call_with_output(err, handle, |obj| {
        let _retval = Rocket::add(obj, <Part as uniffi::ViaFfi>::try_lift(part).unwrap());
        _retval
    })
}

#[allow(clippy::all)]
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn rocketscience_3a9e_Rocket_lock_steering(
    handle: u64,
    direction: uniffi::RustBuffer,
    err: &mut uniffi::deps::ffi_support::ExternError,
) -> () {
    uniffi::deps::log::debug!("rocketscience_3a9e_Rocket_lock_steering");
    // If the method does not have the same signature as declared in the UDL, then
    // this attempt to call it will fail with a (somewhat) helpful compiler error.
    use uniffi::UniffiMethodCall;
    UNIFFI_HANDLE_MAP_ROCKET.method_call_with_output(err, handle, |obj| {
        let _retval = Rocket::lock_steering(
            obj,
            <Direction as uniffi::ViaFfi>::try_lift(direction).unwrap(),
        );
        _retval
    })
}

// Callback Interface defitions, corresponding to UDL `callback interface` definitions.