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
    
