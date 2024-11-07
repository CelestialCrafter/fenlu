from common import listen, validate_config

def has(fn, a, b):
    return fn([tag in a for tag in b])

def filter(media):
    try:
        tags = media['extraMetadata']['tags']
    except (TypeError, KeyError):
        return True

    return has(all, tags, incand) and (len(incor) < 1 or has(any, tags, incor)) and not has(any, tags, exc)

def handle_filter(params):
    return [filter(media) for media in params]

def handle_initialize(params):
    global incand, incor, exc
    incand, incor, exc = validate_config(
        ['included_and', 'included_or', 'excluded'],
        params,
        defaults={'included_and': [], 'included_or': [], 'excluded': []}
    )

    return {
        'version': "b2a8d343480cbaf075c93fd47033db7a2f020773",
        'capabilities':  ["media/filter"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/filter': handle_filter,
})
