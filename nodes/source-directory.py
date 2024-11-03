import os
from urllib.parse import quote
from PIL import Image, UnidentifiedImageError

from common import listen

files = []
config = {}
batch_size = 0

def handle_source(params):
    global files

    state = params['state']

    media = []
    for path in files[batch_size * state:batch_size * (state + 1)]:
        try:
            image = Image.open(path)
        except UnidentifiedImageError:
            continue

        media.append({
            'url': 'file:///' + quote(path.lstrip('/'), safe=':/'),
            'type': 'image',
            'essentialMetadata': {
                'title': os.path.basename(path),
                'creation': int(os.path.getmtime(path) * 1000)
            },
            'typeMetadata': {
                'width': image.width,
                'height': image.height,
            }
        })

    return {
        'media': media,
        'finished': len(files) <= batch_size * (state + 1)
    }

def handle_initialize(params):
    global batch_size
    global config
    global files

    config = params['config']
    batch_size = params['batchSize']
    files = files = [os.path.join(root, file) for root, _, files in os.walk(os.path.expanduser(config['path'])) for file in files]

    return {
        'version': "95a247050de65c132541eabe3d93ca0b7c9b5a65",
        'capabilities':  ["media/source"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/source': handle_source
})
