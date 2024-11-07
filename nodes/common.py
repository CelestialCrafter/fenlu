import sys
import json
import traceback

def log(*args, **kwargs):
    print(*args, **kwargs, file=sys.stderr)

def listen(handlers):
    for line in sys.stdin:
        # strip off EOF
        request = json.loads(line.rstrip())
    
        result = {}
        error = None
    
        method = request['method']
        try:
            if method not in handlers:
                error = 'unknown method'
            else:
                result = handlers[method](request['params'])
        except Exception:
                result = None
                error = traceback.format_exc()
    
        print(json.dumps({
            'id': request['id'],
            'result': {} if result is None else result,
            'error': error
        }))

def set_em_key(media, key, value):
    if 'extraMetadata' not in media or media['extraMetadata'] is None:
        media['extraMetadata'] = {}
    media['extraMetadata'][key] = value

    return media

def validate_config(paths, params, defaults={}):
    if 'config' not in params:
        raise Exception('config not set')

    config = params['config']
    paths = [path.split('.') for path in paths]
    values = []

    for path in paths:
        node = config
        default_node = defaults
        for (i, segment) in enumerate(path):
            nodein = segment in node
            defin = segment in default_node
            if not nodein and not defin:
                raise Exception(f'{'.'.join(path)} not in config')
            if not nodein:
                node = default_node[segment]
            else:
                node = node[segment]
                default_node = node

            if i == len(path) - 1:
                values.append(node)

    return values
