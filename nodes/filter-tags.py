from common import listen

def has(a, b):
    return any([tag in a for tag in b])

def filter(media):
    if 'tags' not in media['extraMetadata']:
        return True

    tags = media['extraMetadata']['tags']
    return has(config['included'], tags) and not has(config['excluded'], tags)

def handle_filter(params):
    return [filter(media) for media in params]

def handle_initialize(params):
    global config
    config = params['config']

    return {
        'version': "95a247050de65c132541eabe3d93ca0b7c9b5a65",
        'capabilities':  ["media/filter"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/filter': handle_filter,
})
