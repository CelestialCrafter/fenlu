import sys
import json

for line in sys.stdin:
    # strip off EOF
    request = json.loads(line.rstrip())

    result = {}
    error = None

    method = request['method']
    if method == 'initialize/initialize':
        result = {
            'version': 'ed19eeb5298ecc9881cbb729fa427abb3ab36c40',
            'capabilities': ['it/works', 'yayyyyyyyyy']
        }
    else:
        result = {'uh': 'oh'}

    print(json.dumps({
        'id': request['id'],
        'result': result,
        'error': error
    }))
