from cffi import FFI


ffi = FFI()
ffi.cdef("""
    typedef ... StringMap;

    StringMap* stringmap_new(size_t count,
                             const char** keys,
                             const char** vals);

    const char** stringmap_names(const StringMap* m);
""")
lib = ffi.dlopen('target/debug/libqselect.dylib')


class StringMap(object):
    def __init__(self, stuff=[], **values):
        ptrs = ((ffi.new('const char *', k), ffi.new('const char *', v))
                for (k, v) in stuff + values.items())
        ptrs = zip(*ptrs)
        keys, vals = [ffi.new('const char*[]', list(arr)) for arr in ptrs]
        count = len(keys)
        self._underyling = lib.stringmap_new(count, keys, vals)

    def keys(self):
        n = 0
        ptr = lib.stringmap_names(self._underyling)
        while ptr[n] != ffi.NULL:
            yield ffi.string(ptr[n])
            n += 1
