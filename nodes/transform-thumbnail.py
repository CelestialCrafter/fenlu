import os
from urllib.parse import quote
from PIL import Image, UnidentifiedImageError

from common import listen, log

def calculate_height(cw, ch, nw):
    ar = cw / ch
    return int(nw / ar)

def transform(media):
    if (not media['url'].startswith('file://')) or media['type'] != 'image':
        return media

    try:
        path = os.path.join(config['directory'], quote(media['essentialMetadata']['title']))
        if os.path.exists(path):
            return media

        height = calculate_height(media['typeMetadata']['width'], media['typeMetadata']['height'], config['size'])

        image = Image.open(media["url"].replace('file://', ''))
        image.thumbnail((config['size'], height))
        image.save(path)

        media['url'] = 'file:///' + quote(path.lstrip('/'), safe=':/')
        media['typeMetadata']['width'] = config['size']
        media['typeMetadata']['height'] = height
    except (UnidentifiedImageError, OSError):
        pass

    return media

def handle_transform(params):
    return [transform(media) for media in params]

def handle_initialize(params):
    global config
    config = params['config']

    return {
        'version': "95a247050de65c132541eabe3d93ca0b7c9b5a65",
        'capabilities':  ["media/transform"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/transform': handle_transform,
})
