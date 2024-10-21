#!/usr/bin/env python

import re

from common import listen

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

def handle_capabilities(_):
    return {
        'media': ('filter', None),
        'query': {
            'set': True
        }
    }

listen({
    'capabilities/capabilities': handle_capabilities,
    'query/set': handle_query,
    'media/filter': handle_filter
})
