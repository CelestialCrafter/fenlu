from common import listen, log

def handle_sink(params):
    for media in params:
        log(media)

def handle_initialize(_):
    return {
        'version': "57969bae27de229c075fcba919924838f61ef2ff",
        'capabilities':  ["media/sink"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/sink': handle_sink
})
