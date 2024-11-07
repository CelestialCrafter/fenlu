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
        'version': "b2a8d343480cbaf075c93fd47033db7a2f020773",
        'capabilities':  ["media/sink"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/sink': handle_sink
})
