from common import listen

def apply_op(lhs, rhs, op):
    default = True

    if not (lhs and rhs):
        return default
    
    ops = {
        'gte': lambda x, y: x >= y,
        'lte': lambda x, y: x <= y,
        'gt': lambda x, y: x > y,
        'lt': lambda x, y: x < y,
        'neq': lambda x, y: x != y,
        'eq': lambda x, y: x == y
    }

    return ops.get(op, lambda x, y: default)(lhs, rhs)

def handle_filter(params):
    global config

    return [
        all(
            apply_op(
                media['typeMetadata']['width'] if condition['lhs'] == 'w' else media['typeMetadata']['height'],
                condition['rhs'],
                condition['op']
            )
            for condition in config['conditions']
        ) if media['type'] == 'image' else True
        for media in params
    ]

def handle_initialize(params):
    global config
    config = params['config']

    return {
        'version': "57969bae27de229c075fcba919924838f61ef2ff",
        'capabilities':  ["media/filter"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/filter': handle_filter
})
