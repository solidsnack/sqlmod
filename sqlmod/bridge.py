from cffi import FFI


header_path = 'target/include/sqlmod.h'
lib_path = 'target/debug/libsqlmod.dylib'


def get_header():
    lines = []
    in_decls = False
    with open(header_path) as f:
        for line in f.readlines():
            if in_decls:
                if line.startswith('#include'):
                    continue
                if line.startswith('#'):
                    break
                lines += line
            else:
                if line.startswith('#include'):
                    in_decls = True
    return ''.join(lines).strip()


header = get_header()
ffi = FFI()
ffi.cdef(header)
lib = ffi.dlopen(lib_path)


class Queries(object):
    def __init__(self, queries):
        ptr = lib.sqlmod_queries_parse(Str(queries)._underlying)
        if ptr == ffi.NULL:
            raise ValueError('Bad query parse.')
        self._underyling = ffi.gc(ptr, lib.sqlmod_queries_free)

    def __len__(self):
        return lib.sqlmod_queries_num_queries(self._underyling)

    def __iter__(self):
        n = 0
        while n < len(self):
            yield self[n]
            n += 1

    def __getitem__(self, i):
        q = lib.sqlmod_queries_get_query_by_index(self._underyling, i)
        if q == ffi.NULL:
            raise IndexError('No query with index %s' % i)
        q = Query(q, self)
        q.queries = self                  # Keep obj alive while query is alive
        return q


class Query(object):
    def __init__(self, underlying, queries):
        self._underlying = underlying
        self._parent = queries

    @property
    def name(self):
        if not hasattr(self, '_name'):
            s = Str(lib.sqlmod_query_get_name(self._underlying))
            setattr(self, '_name', str(s))
        return getattr(self, '_name')

    @property
    def text(self):
        if not hasattr(self, '_text'):
            s = Str(lib.sqlmod_query_get_text(self._underlying))
            setattr(self, '_text', str(s))
        return getattr(self, '_text')

    @property
    def attributes(self):
        if not hasattr(self, '_attributes'):
            num = lib.sqlmod_query_num_attributes(self._underlying)
            ss = [lib.sqlmod_query_get_attribute_by_index(self._underlying, i)
                  for i in range(0, num)]
            setattr(self, '_attributes', [str(Str(s)) for s in ss])
        return getattr(self, '_attributes')

    def __str__(self):
        return '--@ %s\n%s' % (' '.join(self.name, self.attributes), self.text)


class Str(object):
    def __init__(self, s):
        if isinstance(s, ffi.CData):
            self._c_str = None
            underlying = s
        else:
            self._c_str = ffi.new('const char[]', s)    # Hold reference for GC
            underlying = lib.sqlmod_str(self._c_str, len(self._c_str) - 1)
        self._underlying = underlying

    @property
    def ptr(self):
        return self._underlying.ptr

    @property
    def is_null(self):
        return self.ptr == ffi.NULL

    def __len__(self):
        return self._underlying.len

    def __str__(self):
        if self.is_null:
            return ''
        return ffi.unpack(self.ptr, len(self))
