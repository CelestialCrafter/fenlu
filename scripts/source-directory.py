#!/usr/bin/env python

import os
import sys
import json
import traceback
import tomllib
from urllib.parse import quote
from PIL import Image, UnidentifiedImageError

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

    batch = files[batch_size * state:batch_size * (state + 1)]

    media = []
    for path in batch:
        try:
            image = Image.open(path)
        except UnidentifiedImageError:
            continue

        media.append({
            'title': os.path.basename(path),
            'uri': 'file:///' + quote(path, safe=':/'),
            'width': image.width,
            'height': image.height,
            'type': 'Image'
        })

    return {
        'media': media,
        'finished': len(files) <= batch_size * (state + 1)
    }

def handle_capabilities():
    return {
        'media': {
            'source': True
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

    try:
        match request['method']:
            case 'capabilities/capabilities':
                result = handle_capabilities()
            case 'media/generate':
                    result = handle_generate(params)
            case _:
                raise Exception("unknown method")
    except Exception:
                result = None
                error = traceback.format_exc()

    print(json.dumps({
        'id': id,
        'result': result,
        'error': error
    }))

