from time import time
from functools import wraps

class Debugger:
    attribute_accesses = []
    method_calls = []

def wrapped_method(cls, method):
    @wraps(method)
    def wrapper(*args, **kwargs):
        start_time = time()
        result = method(*args, **kwargs)
        end_time = time()
        Debugger.method_calls.append({
            'class': cls,
            'method': method.__name__,
            'args': args,
            'kwargs': kwargs,
            'time': end_time - start_time
        })
        return result
    return wrapper

def wrapped_setattr(cls):
    def setattr_wrapper(self, name, value):
        object.__setattr__(self, name, value)
        Debugger.attribute_accesses.append({
            'action': 'set',
            'class': cls,
            'attribute': name,
            'value': value
        })
    return setattr_wrapper

def wrapped_getattribute(cls):
    def getattr_wrapper(self, name):
        value = object.__getattribute__(self, name)
        Debugger.attribute_accesses.append({
            'action': 'get',
            'class': cls,
            'attribute': name,
            'value': value
        })
        return value
    return getattr_wrapper

class Meta(type):
    def __new__(cls, name, bases, dct):
        for attr_name, attr_value in dct.items():
            if callable(attr_value):
                dct[attr_name] = wrapped_method(name, attr_value)
        dct['__setattr__'] = wrapped_setattr(name)
        dct['__getattribute__'] = wrapped_getattribute(name)
        return super().__new__(cls, name, bases, dct)

class DebuggedClass(metaclass=Meta):
    pass