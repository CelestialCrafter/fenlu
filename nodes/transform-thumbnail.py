import os
from PIL import Image, UnidentifiedImageError

from common import listen, validate_config

def calculate_height(cw, ch, nw):
    ar = cw / ch
    return int(nw / ar)

def transform(media):
    path = os.path.join(directory, os.path.basename(media['url']))
    if os.path.exists(path):
        media['url'] = 'file:///' + path.lstrip('/')
        return media

    if (not media['url'].startswith('file://')) or media['type'] != 'image':
        return media

    try:
        height = calculate_height(media['typeMetadata']['width'], media['typeMetadata']['height'], size)

        image = Image.open(media["url"].replace('file://', ''))
        image.thumbnail((size, height))
        image.save(path)

        media['url'] = 'file:///' + path.lstrip('/')
        media['typeMetadata']['width'] = size
        media['typeMetadata']['height'] = height
    
        if 'extraMetadata' not in media or media['extraMetadata'] is None:
            media['extraMetadata'] = {}
        media['extraMetadata']['thumbnailOriginalUrl'] = media['url']
    except (UnidentifiedImageError, OSError):
        pass

    return media

def handle_transform(params):
    return [transform(media) for media in params]

def handle_initialize(params):
    global directory, size
    directory, size = validate_config(['directory', 'size'], params, defaults={'size': 512})

    return {
        'version': "667430e325dda8b8949276d39b87c031a304c55b",
        'capabilities':  ["media/transform"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/transform': handle_transform,
})
