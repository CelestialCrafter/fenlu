#!/usr/bin/env python

import sys
import json

def has_tag(tags, desired):
    return any(tag == desired for tag in tags)

def has_tags(tags, desired):
    result = True

    for tag in desired:
        invert = False
        if tag.startswith('!'):
            invert = True
            tag = tag[1:]

        result = result and has_tag(tags, tag)
        if invert:
            result = not result
        if not result:
            return False

    return True

tags = []

def handle_query(params):
    global tags

    query = params['query']
    tags = query.split(';')
    tags = tags[:len(tags) - 1]

    return { 'query': query }

def handle_filter(params):
    global tags
    return list(map(lambda media: len(tags) == 0 or has_tags(media['tags'], tags), params))

def handle_capabilities():
    return {
        'media': {
            'filter': True
        },
        'query': {
            'set': True
        }
    }

for line in sys.stdin:
    # strip off EOF
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
            result = None
            error = "unknown method"

    print(json.dumps({
        'id': id,
        'result': result,
        'error': error
    }))
