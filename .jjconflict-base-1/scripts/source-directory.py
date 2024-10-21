#!/usr/bin/env python

import os
import tomllib
from urllib.parse import quote
from PIL import Image, UnidentifiedImageError

from common import listen

with open('config-source-directory.toml', 'rb') as file:
    config = tomllib.load(file)

files = []
initialized = False

def handle_generate(params):
    global files
    global initialized
    if not initialized:
        files = [os.path.join(root, file) for root, _, files in os.walk(os.path.expanduser(config['path'])) for file in files]
        initialized = True

    batch_size = params['batch_size']
    state = params['state']

    media = []
    for path in files[batch_size * state:batch_size * (state + 1)]:
        try:
            image = Image.open(path)
        except UnidentifiedImageError:
            continue

        media.append({
            'title': os.path.basename(path),
            'uri': 'file:///' + quote(path.lstrip('/'), safe=':/'),
            'width': image.width,
            'height': image.height,
            'type': 'Image'
        })

    return {
        'media': media,
        'finished': len(files) <= batch_size * (state + 1)
    }

def handle_capabilities(_):
    return { 'media': ('source', None) }

listen({
    'capabilities/capabilities': handle_capabilities,
    'media/generate': handle_generate
})
