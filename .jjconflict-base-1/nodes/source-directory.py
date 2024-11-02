import os
from urllib.parse import quote
from PIL import Image, UnidentifiedImageError

from common import listen

files = []
config = {}
batch_size = 0

def handle_generate(params):
    global files

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

def handle_initialize(params):
    global batch_size
    global config
    global files

    config = params['config']
    batch_size = params['batch_size']
    files = files = [os.path.join(root, file) for root, _, files in os.walk(os.path.expanduser(config['path'])) for file in files]

    return {
        'version': "ed19eeb5298ecc9881cbb729fa427abb3ab36c40",
        'capabilities':  ["media/source"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/generate': handle_generate
})
