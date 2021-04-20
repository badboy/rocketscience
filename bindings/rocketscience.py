# This file was autogenerated by some hot garbage in the `uniffi` crate.
# Trust me, you don't want to mess with it!

# Common helper code.
#
# Ideally this would live in a separate .py file where it can be unittested etc
# in isolation, and perhaps even published as a re-useable package.
#
# However, it's important that the detils of how this helper code works (e.g. the
# way that different builtin types are passed across the FFI) exactly match what's
# expected by the rust code on the other side of the interface. In practice right
# now that means coming from the exact some version of `uniffi` that was used to
# compile the rust component. The easiest way to ensure this is to bundle the Python
# helpers directly inline like we're doing here.

import os
import sys
import ctypes
import enum
import struct
import contextlib


class RustBuffer(ctypes.Structure):
    _fields_ = [
        ("capacity", ctypes.c_int32),
        ("len", ctypes.c_int32),
        ("data", ctypes.POINTER(ctypes.c_char)),
        # Ref https://github.com/mozilla/uniffi-rs/issues/334 for this weird "padding" field.
        ("padding", ctypes.c_int64),
    ]

    @staticmethod
    def alloc(size):
        return rust_call_with_error(InternalError, _UniFFILib.ffi_rocketscience_3a9e_rustbuffer_alloc, size)

    @staticmethod
    def reserve(rbuf, additional):
        return rust_call_with_error(InternalError, _UniFFILib.ffi_rocketscience_3a9e_rustbuffer_reserve, rbuf, additional)

    def free(self):
        return rust_call_with_error(InternalError, _UniFFILib.ffi_rocketscience_3a9e_rustbuffer_free, self)

    def __str__(self):
        return "RustBuffer(capacity={}, len={}, data={})".format(
            self.capacity,
            self.len,
            self.data[0:self.len]
        )

    @contextlib.contextmanager
    def allocWithBuilder():
        """Context-manger to allocate a buffer using a RustBufferBuilder.

        The allocated buffer will be automatically freed if an error occurs, ensuring that
        we don't accidentally leak it.
        """
        builder = RustBufferBuilder()
        try:
            yield builder
        except:
            builder.discard()
            raise

    @contextlib.contextmanager
    def consumeWithStream(self):
        """Context-manager to consume a buffer using a RustBufferStream.

        The RustBuffer will be freed once the context-manager exits, ensuring that we don't
        leak it even if an error occurs.
        """
        try:
            s = RustBufferStream(self)
            yield s
            if s.remaining() != 0:
                raise RuntimeError("junk data left in buffer after consuming")
        finally:
            self.free()

    # For every type that lowers into a RustBuffer, we provide helper methods for
    # conveniently doing the lifting and lowering. Putting them on this internal
    # helper object (rather than, say, as methods on the public classes) makes it
    # easier for us to hide these implementation details from consumers, in the face
    # of python's free-for-all type system.

    # The Enum type Direction.

    @staticmethod
    def allocFromEnumDirection(v):
        with RustBuffer.allocWithBuilder() as builder:
            builder.writeEnumDirection(v)
            return builder.finalize()

    def consumeIntoEnumDirection(self):
        with self.consumeWithStream() as stream:
            return stream.readEnumDirection()

    # The Record type Part.

    @staticmethod
    def allocFromRecordPart(v):
        with RustBuffer.allocWithBuilder() as builder:
            builder.writeRecordPart(v)
            return builder.finalize()

    def consumeIntoRecordPart(self):
        with self.consumeWithStream() as stream:
            return stream.readRecordPart()

    # The primitive String type.

    @staticmethod
    def allocFromString(value):
        with RustBuffer.allocWithBuilder() as builder:
            builder.write(value.encode("utf-8"))
            return builder.finalize()

    def consumeIntoString(self):
        with self.consumeWithStream() as stream:
            return stream.read(stream.remaining()).decode("utf-8")


class ForeignBytes(ctypes.Structure):
    _fields_ = [
        ("len", ctypes.c_int32),
        ("data", ctypes.POINTER(ctypes.c_char)),
        # Ref https://github.com/mozilla/uniffi-rs/issues/334 for these weird "padding" fields.
        ("padding", ctypes.c_int64),
        ("padding2", ctypes.c_int32),
    ]

    def __str__(self):
        return "ForeignBytes(len={}, data={})".format(self.len, self.data[0:self.len])

class RustBufferStream(object):
    """Helper for structured reading of values from a RustBuffer."""

    def __init__(self, rbuf):
        self.rbuf = rbuf
        self.offset = 0

    def remaining(self):
        return self.rbuf.len - self.offset

    def _unpack_from(self, size, format):
        if self.offset + size > self.rbuf.len:
            raise InternalError("read past end of rust buffer")
        value = struct.unpack(format, self.rbuf.data[self.offset:self.offset+size])[0]
        self.offset += size
        return value

    def read(self, size):
        if self.offset + size > self.rbuf.len:
            raise InternalError("read past end of rust buffer")
        data = self.rbuf.data[self.offset:self.offset+size]
        self.offset += size
        return data

    # For every type used in the interface, we provide helper methods for conveniently
    # reading that type in a buffer. Putting them on this internal helper object (rather
    # than, say, as methods on the public classes) makes it easier for us to hide these
    # implementation details from consumers, in the face of python's free-for-all type
    # system.

    def readBool(self):
        v = self._unpack_from(1, ">b")
        if v == 0:
            return False
        if v == 1:
            return True
        raise InternalError("Unexpected byte for Boolean type")

    # The Enum type Direction.

    def readEnumDirection(self):
        variant = self._unpack_from(4, ">i")
        return Direction(variant)

    # The Error type LaunchError.
    # Errors cannot currently be serialized, but we can produce a helpful error.

    def readErrorLaunchError(self):
        raise InternalError("RustBufferStream.read not implemented yet for ErrorLaunchError")

    # The Record type Part.

    def readRecordPart(self):
        return Part(
            self.readString(),
            self.readU64(),
            self.readU64()
        )

    def readString(self):
        size = self._unpack_from(4, ">i")
        if size < 0:
            raise InternalError("Unexpected negative string length")
        utf8Bytes = self.read(size)
        return utf8Bytes.decode("utf-8")

    # The Object type Rocket.
    # Objects cannot currently be serialized, but we can produce a helpful error.

    def readObjectRocket(self):
        raise InternalError("RustBufferStream.read not implemented yet for ObjectRocket")

    def readU64(self):
        return self._unpack_from(8, ">Q")

class RustBufferBuilder(object):
    """Helper for structured writing of values into a RustBuffer."""

    def __init__(self):
        self.rbuf = RustBuffer.alloc(16)
        self.rbuf.len = 0

    def finalize(self):
        rbuf = self.rbuf
        self.rbuf = None
        return rbuf

    def discard(self):
        if self.rbuf is not None:
            rbuf = self.finalize()
            rbuf.free()

    @contextlib.contextmanager
    def _reserve(self, numBytes):
        if self.rbuf.len + numBytes > self.rbuf.capacity:
            self.rbuf = RustBuffer.reserve(self.rbuf, numBytes)
        yield None
        self.rbuf.len += numBytes

    def _pack_into(self, size, format, value):
        with self._reserve(size):
            # XXX TODO: I feel like I should be able to use `struct.pack_into` here but can't figure it out.
            for i, byte in enumerate(struct.pack(format, value)):
                self.rbuf.data[self.rbuf.len + i] = byte

    def write(self, value):
        with self._reserve(len(value)):
            for i, byte in enumerate(value):
                self.rbuf.data[self.rbuf.len + i] = byte

    # For every type used in the interface, we provide helper methods for conveniently
    # writing values of that type in a buffer. Putting them on this internal helper object
    # (rather than, say, as methods on the public classes) makes it easier for us to hide
    # these implementation details from consumers, in the face of python's free-for-all
    # type system.

    def writeBool(self, v):
        self._pack_into(1, ">b", 1 if v else 0)

    # The Enum type Direction.

    def writeEnumDirection(self, v):
        self._pack_into(4, ">i", v.value)

    # The Error type LaunchError.
    # Errors cannot currently be serialized, but we can produce a helpful error.

    def writeErrorLaunchError(self):
        raise InternalError("RustBufferStream.write() not implemented yet for ErrorLaunchError")

    # The Record type Part.

    def writeRecordPart(self, v):
        self.writeString(v.name)
        self.writeU64(v.cost)
        self.writeU64(v.weight)

    def writeString(self, v):
        utf8Bytes = v.encode("utf-8")
        self._pack_into(4, ">i", len(utf8Bytes))
        self.write(utf8Bytes)

    # The Object type Rocket.
    # Objects cannot currently be serialized, but we can produce a helpful error.

    def writeObjectRocket(self):
        raise InternalError("RustBufferStream.write() not implemented yet for ObjectRocket")

    def writeU64(self, v):
        self._pack_into(8, ">Q", v)

# Error definitions
class RustError(ctypes.Structure):
    _fields_ = [
        ("code", ctypes.c_int32),
        ("message", ctypes.c_void_p),
    ]

    def free(self):
        rust_call_with_error(InternalError, _UniFFILib.ffi_rocketscience_3a9e_string_free, self.message)

    def __str__(self):
        return "RustError(code={}, message={})".format(
            self.code,
            str(ctypes.cast(self.message, ctypes.c_char_p).value, "utf-8"),
        )

class InternalError(Exception):
    @staticmethod
    def raise_err(code, message):
        raise InternalError(message)


class LaunchError:
    class RocketLaunch(Exception):
        pass

    @staticmethod
    def raise_err(code, message):
        if code == 1:
            raise LaunchError.RocketLaunch(message)
        
        raise Exception("Unknown error code")


def rust_call_with_error(error_class, fn, *args):
    error = RustError()
    error.code = 0

    args_with_error = args + (ctypes.byref(error),)
    result = fn(*args_with_error)
    if error.code != 0:
        message = str(error)
        error.free()

        error_class.raise_err(error.code, message)
    
    return result

# This is how we find and load the dynamic library provided by the component.
# For now we just look it up by name.
#
# XXX TODO: This will probably grow some magic for resolving megazording in future.
# E.g. we might start by looking for the named component in `libuniffi.so` and if
# that fails, fall back to loading it separately from `lib${componentName}.so`.

def loadIndirect():
    if sys.platform == "linux":
        libname = "lib{}.so"
    elif sys.platform == "darwin":
        libname = "lib{}.dylib"
    elif sys.platform.startswith("win"):
        # As of python3.8, ctypes does not seem to search $PATH when loading DLLs.
        # We could use `os.add_dll_directory` to configure the search path, but
        # it doesn't feel right to mess with application-wide settings. Let's
        # assume that the `.dll` is next to the `.py` file and load by full path.
        libname = os.path.join(
            os.path.dirname(__file__),
            "{}.dll",
        )
    return getattr(ctypes.cdll, libname.format("rocketscience"))

# A ctypes library to expose the extern-C FFI definitions.
# This is an implementation detail which will be called internally by the public API.

_UniFFILib = loadIndirect()
_UniFFILib.ffi_rocketscience_3a9e_Rocket_object_free.argtypes = (
    ctypes.c_uint64,
    ctypes.POINTER(RustError),
)
_UniFFILib.ffi_rocketscience_3a9e_Rocket_object_free.restype = None
_UniFFILib.rocketscience_3a9e_Rocket_new.argtypes = (
    RustBuffer,
    ctypes.POINTER(RustError),
)
_UniFFILib.rocketscience_3a9e_Rocket_new.restype = ctypes.c_uint64
_UniFFILib.rocketscience_3a9e_Rocket_show.argtypes = (
    ctypes.c_uint64,
    ctypes.POINTER(RustError),
)
_UniFFILib.rocketscience_3a9e_Rocket_show.restype = RustBuffer
_UniFFILib.rocketscience_3a9e_Rocket_launch.argtypes = (
    ctypes.c_uint64,
    ctypes.POINTER(RustError),
)
_UniFFILib.rocketscience_3a9e_Rocket_launch.restype = ctypes.c_int8
_UniFFILib.rocketscience_3a9e_Rocket_add.argtypes = (
    ctypes.c_uint64,
    RustBuffer,
    ctypes.POINTER(RustError),
)
_UniFFILib.rocketscience_3a9e_Rocket_add.restype = None
_UniFFILib.rocketscience_3a9e_Rocket_lock_steering.argtypes = (
    ctypes.c_uint64,
    RustBuffer,
    ctypes.POINTER(RustError),
)
_UniFFILib.rocketscience_3a9e_Rocket_lock_steering.restype = None
_UniFFILib.ffi_rocketscience_3a9e_rustbuffer_alloc.argtypes = (
    ctypes.c_int32,
    ctypes.POINTER(RustError),
)
_UniFFILib.ffi_rocketscience_3a9e_rustbuffer_alloc.restype = RustBuffer
_UniFFILib.ffi_rocketscience_3a9e_rustbuffer_from_bytes.argtypes = (
    ForeignBytes,
    ctypes.POINTER(RustError),
)
_UniFFILib.ffi_rocketscience_3a9e_rustbuffer_from_bytes.restype = RustBuffer
_UniFFILib.ffi_rocketscience_3a9e_rustbuffer_free.argtypes = (
    RustBuffer,
    ctypes.POINTER(RustError),
)
_UniFFILib.ffi_rocketscience_3a9e_rustbuffer_free.restype = None
_UniFFILib.ffi_rocketscience_3a9e_rustbuffer_reserve.argtypes = (
    RustBuffer,
    ctypes.c_int32,
    ctypes.POINTER(RustError),
)
_UniFFILib.ffi_rocketscience_3a9e_rustbuffer_reserve.restype = RustBuffer
_UniFFILib.ffi_rocketscience_3a9e_string_free.argtypes = (
    ctypes.c_voidp,
    ctypes.POINTER(RustError),
)
_UniFFILib.ffi_rocketscience_3a9e_string_free.restype = None

# Public interface members begin here.





class Direction(enum.Enum):
    UP = 1
    DOWN = 2
    


class Part(object):
    def __init__(self,name, cost, weight ):
        self.name = name
        self.cost = cost
        self.weight = weight

    def __str__(self):
        return "Part(name={}, cost={}, weight={} )".format(self.name, self.cost, self.weight )

    def __eq__(self, other):
        if self.name != other.name:
            return False
        if self.cost != other.cost:
            return False
        if self.weight != other.weight:
            return False
        return True





class Rocket(object):
    def __init__(self, name):
        name = name
        self._handle = rust_call_with_error(InternalError,_UniFFILib.rocketscience_3a9e_Rocket_new,RustBuffer.allocFromString(name))\

    def __del__(self):
        rust_call_with_error(
            InternalError,
            _UniFFILib.ffi_rocketscience_3a9e_Rocket_object_free,
            self._handle
        )

    

    def show(self, ):
        _retval = rust_call_with_error(InternalError,_UniFFILib.rocketscience_3a9e_Rocket_show,self._handle,)
        return _retval.consumeIntoString()
    
    def launch(self, ):
        _retval = rust_call_with_error(LaunchError,_UniFFILib.rocketscience_3a9e_Rocket_launch,self._handle,)
        return (True if _retval else False)
    
    def add(self, part):
        part = part
        rust_call_with_error(InternalError,_UniFFILib.rocketscience_3a9e_Rocket_add,self._handle,RustBuffer.allocFromRecordPart(part))
    
    def lock_steering(self, direction):
        direction = direction
        rust_call_with_error(InternalError,_UniFFILib.rocketscience_3a9e_Rocket_lock_steering,self._handle,RustBuffer.allocFromEnumDirection(direction))
    
    


__all__ = [
    "InternalError",
    "Direction",
    "Part",
    "Rocket",
    "LaunchError",
]

