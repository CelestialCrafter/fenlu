from common import listen, log

def handle_sink(params):
    for media in params:
        log(media)

def handle_initialize(_):
    return {
        'version': "95a247050de65c132541eabe3d93ca0b7c9b5a65",
        'capabilities':  ["media/sink"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/sink': handle_sink
})
