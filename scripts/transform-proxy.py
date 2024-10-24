#!/usr/bin/env python

import tomllib
from urllib.parse import urlparse, urlunparse

from common import listen

with open('config-transform-proxy.toml', 'rb') as file:
    config = tomllib.load(file)

def transform(media):
    suffix = ''
    if 'source-pixiv.py' in media['history']:
        suffix = "pixiv"
    elif 'exclude-source-kemono.py' in media['history']:
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

def handle_capabilities(_):
    return { 'media': ('transform', None) }

listen({
    'capabilities/capabilities': handle_capabilities,
    'media/transform': handle_transform
})
