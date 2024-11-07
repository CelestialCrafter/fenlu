import json

from common import listen, validate_config

def handle_sink(params):
    for media in params:
        file.write(json.dumps(media) + '\n')

def handle_initialize(params):
    global file
    path, = validate_config(['path'], params)
    file = open(path, 'w+')

    return {
        'version': "667430e325dda8b8949276d39b87c031a304c55b",
        'capabilities':  ["media/sink"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/sink': handle_sink
})
