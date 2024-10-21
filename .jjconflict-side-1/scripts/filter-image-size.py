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

def parse_conditions(query):
    return re.findall(r'([wh]) ([<>!=]+) (\w+);', query)

query = sys.argv[1]
for line in sys.stdin:
    line = line.rstrip()
    if query == '':
        print(line)
        continue

    media = json.loads(line)
    if media['type'] != 'Image':
        print(line)
        continue
    
    ops = [
        apply_op(
            media['width'] if lhs == 'w' else media['height'],
            int(rhs),
            op
        )
        for lhs, op, rhs in parse_conditions(query)
    ]

    if all(ops):
        print(line)

