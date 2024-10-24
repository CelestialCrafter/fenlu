#!/usr/bin/env python

from common import listen

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

    tags = params['query'].split(';')
    tags = tags[:len(tags) - 1]

    return

def handle_filter(params):
    global tags
    return {
        'included': list(map(lambda media: len(tags) == 0 or has_tags(media['tags'], tags), params))
    }

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
