import os
from PIL import Image, UnidentifiedImageError

from common import listen, validate_config, set_em_key

def calculate_height(cw, ch, nw):
    ar = cw / ch
    return int(nw / ar)

def transform(media):
    path = os.path.join(directory, os.path.basename(media['url']))
    exists = os.path.exists(path)

    if not exists and (not media['url'].startswith('file://')) or media['type'] != 'image':
        return media

    try:
        height = calculate_height(media['typeMetadata']['width'], media['typeMetadata']['height'], size)

        if exists:
            image =  Image.open(path)
        else:
            image = Image.open(media["url"].replace('file://', ''))
            image.thumbnail((size, height))
            image.save(path)

        media['url'] = 'file:///' + path.lstrip('/')
        media['typeMetadata']['width'] = size
        media['typeMetadata']['height'] = height
    
        media = set_em_key(media, 'thumbnailOriginalUrl', media['url'])
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
