#!/usr/bin/env python

import os
import sys
import tomllib
import subprocess
from urllib.parse import urlparse, urlunparse

from common import listen, open_uri

with open('config-transform-proxy.toml', 'rb') as file:
    config = tomllib.load(file)

originals = {}

def transform(media):
    original = media['uri']

    suffix = ''
    if 'source-pixiv.py' in media['history']:
        suffix = "pixiv"
    elif 'exclude-source-kemono.py' in media['history']:
        suffix = "kemono"
    else:
        return media, original

    uri = urlparse(media['uri'])
    if 'http' in uri.scheme:
        if uri.path == '':
            uri = uri._replace(path = '/')

        # its called an AUTHORITY not a "netloc". i hate this language
        # https://www.rfc-editor.org/rfc/rfc2396#section-3.2
        uri = uri._replace(netloc=config['proxy_authority'], path=suffix + uri.path)

        # "urlunparse".. really?
        media['uri'] = urlunparse(uri)

    return media, original

def handle_transform(params):
    media, extra = list(zip(*map(transform, params)))
    return {
            'media': media,
            'extra': extra
    }

def handle_open_original(params):
    open_uri(params['history'][os.path.basename(__file__)])
    return {}

def handle_capabilities(_):
    return { 'media': ('transform', None), 'actions': ['open-original'] }

listen({
    'capabilities/capabilities': handle_capabilities,
    'media/transform': handle_transform,
    'actions/open-original': handle_open_original
})
