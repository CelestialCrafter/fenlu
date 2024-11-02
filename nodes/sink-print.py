from common import listen, log

def handle_sink(params):
    for media in params:
        log(media)

def handle_initialize(_):
    return {
        'version': "95a4fc300cc044cebe957d4fbd829b822bf59a77",
        'capabilities':  ["media/sink"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/sink': handle_sink
})
