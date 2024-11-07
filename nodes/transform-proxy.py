from urllib.parse import urlparse, urlunparse

from common import listen, validate_config

def transform(media):
    for target in targets:
        if target['match'] not in media['url']:
            continue

        url = urlparse(media['url'])
        if url.path == '':
            url = url._replace(path = '/')

        # its called an AUTHORITY not a "netloc". i hate this language
        # https://www.rfc-editor.org/rfc/rfc2396#section-3.2
        url = url._replace(netloc=target['authority'], path=target['path'] + url.path)

        if 'extraMetadata' not in media or media['extraMetadata'] is None:
            media['extraMetadata'] = {}
        media['extraMetadata']['proxyOriginalUrl'] = media['url']
        # "urlunparse".. really?
        media['url'] = urlunparse(url)

        break

    return media

def handle_transform(params):
    return [transform(media) for media in params]

def handle_initialize(params):
    global targets
    targets, = validate_config(['targets'], params)

    return {
        'version': "667430e325dda8b8949276d39b87c031a304c55b",
        'capabilities':  ["media/transform"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/transform': handle_transform,
})
