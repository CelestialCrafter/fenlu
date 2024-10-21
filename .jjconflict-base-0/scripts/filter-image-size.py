#!/usr/bin/env python

import sys
import json
import re

def apply_op(lhs, rhs, op):
    default = True

    if not (lhs and rhs):
        return default
    
    ops = {
        '>=': lambda x, y: x >= y,
        '<=': lambda x, y: x <= y,
        '>': lambda x, y: x > y,
        '<': lambda x, y: x < y,
        '!': lambda x, y: x != y,
        '=': lambda x, y: x == y
    }

    return ops.get(op, lambda x, y: default)(lhs, rhs)

conditions = []

def handle_query(params):
    global conditions
    conditions = re.findall(r'([wh]) ([<>!=]+) (\d+);', params['query'])
    return

def handle_filter(params):
    global conditions
    return [
        all(
            apply_op(
                media['width'] if lhs == 'w' else media['height'],
                int(rhs),
                op
            )
            for lhs, op, rhs in conditions
        ) if media['type'] == 'Image' else True
        for media in params
    ]

def handle_capabilities():
    return {
        'media': ('filter', None),
        'query': {
            'set': True
        }
    }

for line in sys.stdin:
    line = line.rstrip()

    request = json.loads(line)
    params = request['params']
    id = request['id']

    result = {}
    error = None

    match request['method']:
        case 'capabilities/capabilities':
            result = handle_capabilities()
        case 'query/set':
            result = handle_query(params)
        case 'media/filter':
            result = handle_filter(params)
        case _:
            raise Exception("unknown method")

    print(json.dumps({
        'id': id,
        'result': result,
        'error': error
    }))
