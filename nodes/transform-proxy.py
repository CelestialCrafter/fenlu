from urllib.parse import urlparse, urlunparse

from common import listen

def transform(media):
    for target in config['targets']:
        if target['match'] not in media['url']:
            continue

        url = urlparse(media['url'])
        if url.path == '':
            url = url._replace(path = '/')

        # its called an AUTHORITY not a "netloc". i hate this language
        # https://www.rfc-editor.org/rfc/rfc2396#section-3.2
        url = url._replace(netloc=target['authority'], path=target['path'] + url.path)

        # "urlunparse".. really?
        media['extraMetadata']['originalUrl'] = media['url']
        media['url'] = urlunparse(url)

        break

    return media

def handle_transform(params):
    return [transform(media) for media in params]

def handle_initialize(params):
    global config
    config = params['config']

    return {
        'version': "57969bae27de229c075fcba919924838f61ef2ff",
        'capabilities':  ["media/transform"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/transform': handle_transform,
})
