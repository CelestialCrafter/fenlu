from common import listen

def has(fn, a, b):
    return fn([tag in a for tag in b])

def filter(media):
    try:
        tags = media['extraMetadata']['tags']
    except (TypeError, KeyError):
        return True

    return has(all, tags, config['included_and']) and (len(config['included_or']) < 1 or has(any, tags, config['included_or'])) and not has(any, tags, config['excluded'])

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
