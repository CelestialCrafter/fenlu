from common import listen

def has(fn, a, b):
    return fn([tag in a for tag in b])

def filter(media):
    if media['extraMetadata'] is None or 'tags' not in media['extraMetadata']:
        return True

    tags = media['extraMetadata']['tags']
    return has(all, tags, config['included_and']) and has(any, tags, config['included_or']) and not has(any, tags, config['excluded'])

def handle_filter(params):
    return [filter(media) for media in params]

def handle_initialize(params):
    global config
    config = params['config']

    return {
        'version': "667430e325dda8b8949276d39b87c031a304c55b",
        'capabilities':  ["media/filter"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/filter': handle_filter,
})
