import json

from common import listen, log

def handle_sink(params):
    for media in params:
        log(json.dumps(media))

def handle_initialize(_):
    return {
        'version': "667430e325dda8b8949276d39b87c031a304c55b",
        'capabilities':  ["media/sink"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/sink': handle_sink
})
