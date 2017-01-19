from collections import OrderedDict

from . import bridge


def parse(queries):
    return OrderedDict((query.name, query)
                       for query in bridge.Queries(queries))
