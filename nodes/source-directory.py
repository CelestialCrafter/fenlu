import os
from urllib.parse import quote
from PIL import Image, UnidentifiedImageError

from common import listen

def handle_source(params):
    global files

    state = params['state']

    media = []
    for path in files[batch_size * state:batch_size * (state + 1)]:
        try:
            image = Image.open(path)
        except (UnidentifiedImageError, IsADirectoryError):
            continue

        media.append({
            'url': 'file:///' + quote(path.lstrip('/'), safe=':/'),
            'type': 'image',
            'essentialMetadata': {
                'title': os.path.basename(path),
                'creation': os.path.getmtime(path)
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
    dir = os.path.expanduser(config['path'])
    if config['walk']:
        files = [os.path.join(root, file) for root, _, files in os.walk(dir) for file in files]
    else:
        files = [os.path.join(dir, file) for file in os.listdir(dir)]

    return {
        'version': "667430e325dda8b8949276d39b87c031a304c55b",
        'capabilities':  ["media/source"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/source': handle_source
})
