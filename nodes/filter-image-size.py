from common import listen, validate_config

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
    return [
        all(
            apply_op(
                media['typeMetadata']['width'] if condition['lhs'] == 'w' else media['typeMetadata']['height'],
                condition['rhs'],
                condition['op']
            )
            for condition in conditions
        ) if media['type'] == 'image' else True
        for media in params
    ]

def handle_initialize(params):
    global conditions
    conditions, = validate_config(['conditions'], params, defaults={'conditions': []})

    return {
        'version': "b2a8d343480cbaf075c93fd47033db7a2f020773",
        'capabilities':  ["media/filter"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/filter': handle_filter
})
