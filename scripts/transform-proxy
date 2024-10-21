#!/usr/bin/env python

import sys
import json
import tomllib
from urllib.parse import urlparse, urlunparse

with open('config-transform-proxy.toml', 'rb') as file:
    config = tomllib.load(file)

def handle_capabilities():
    return {
        'media': {
            'transform': True
        }
    }

def transform(media):
    suffix = ''
    if 'source-pixiv' in media['history']:
        suffix = "pixiv"
    elif 'source-kemono' in media['history']:
        suffix = "kemono"
    else:
        return media

    uri = urlparse(media['uri'])
    if 'http' in uri.scheme:
        if uri.path == '':
            uri = uri._replace(path = '/')

        # its called an AUTHORITY not a "netloc". i hate this language
        # https://www.rfc-editor.org/rfc/rfc2396#section-3.2
        uri = uri._replace(netloc=config['proxy_authority'], path=suffix + uri.path)

    # "urlunparse".. really?
    media['uri'] = urlunparse(uri)

    return media

def handle_transform(params):
    return list(map(transform, params))

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
        case 'media/transform':
            result = handle_transform(params)
        case _:
            result = None
            error = "unknown method"

    print(json.dumps({
        'id': id,
        'result': result,
        'error': error
    }))
