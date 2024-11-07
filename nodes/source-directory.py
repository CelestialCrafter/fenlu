import os
from urllib.parse import quote
from PIL import Image, UnidentifiedImageError

from common import listen, validate_config

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
    global files

    batch_size = params['batchSize']
    walk, directory = validate_config(['walk', 'directory'], params, defaults={'walk': True})

    directory = os.path.expanduser(directory)
    if walk:
        files = [os.path.join(root, file) for root, _, files in os.walk(directory) for file in files]
    else:
        files = [os.path.join(directory, file) for file in os.listdir(directory)]

    return {
        'version': "b2a8d343480cbaf075c93fd47033db7a2f020773",
        'capabilities':  ["media/source"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/source': handle_source
})
