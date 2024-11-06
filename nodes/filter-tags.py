from common import listen

def has(a, b):
    return any([tag in a for tag in b])

def filter(media):
    if media['extraMetadata'] is None or 'tags' not in media['extraMetadata']:
        return True

    tags = media['extraMetadata']['tags']
    return all([tag in tags for tag in config['included']]) and not any([tag in tags for tag in config['excluded']])

def handle_filter(params):
    return [filter(media) for media in params]

def handle_initialize(params):
    global config
    config = params['config']

    return {
        'version': "57969bae27de229c075fcba919924838f61ef2ff",
        'capabilities':  ["media/filter"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/filter': handle_filter,
})
