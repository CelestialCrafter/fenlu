import pykakasi

from common import listen

def transform(media):
    if isinstance(media, str):
        return ' '.join([entry['hepburn'] for entry in kks.convert(media)])
    elif isinstance(media, dict):
        return {key: value if key in ['url', 'type'] else transform(value) for key, value in media.items()}
    elif isinstance(media, list):
        return [transform(item) for item in media]
    return media

def handle_transform(params):
    return [transform(media) for media in params]

def handle_initialize(_):
    global kks
    kks = pykakasi.kakasi()

    return {
        'version': "667430e325dda8b8949276d39b87c031a304c55b",
        'capabilities':  ["media/transform"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/transform': handle_transform,
})