import sys
import json
import traceback
import subprocess

def open_uri(uri):
    if sys.platform == 'win32':
        command = 'start'
    elif sys.platform == 'darwin':
        command = 'open'
    else:
        command = 'xdg-open'
    subprocess.Popen([command, uri], stdout=sys.stderr, stderr=sys.stderr)

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
                result = handlers[method](request['params']) or {}
        except Exception:
                result = None
                error = traceback.format_exc()
    
        print(json.dumps({
            'id': request['id'],
            'result': result,
            'error': error
        }))
    
