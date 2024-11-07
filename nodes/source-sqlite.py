import json
import sqlite3
from common import listen, validate_config

def handle_source(params):
    global cursor
    global batch_size

    rows = cursor.execute('SELECT data FROM media LIMIT ?, ?', (params['state'] * batch_size, batch_size)).fetchall()
    media = [json.loads(data[0]) for data in rows]

    return {
        'media': media,
        'finished': len(media) < batch_size
    }

def handle_initialize(params):
    global batch_size
    global connection
    global cursor

    batch_size = params['batchSize']
    
    path, = validate_config(['path'], params)
    connection = sqlite3.connect(path)
    cursor = connection.cursor()
    cursor.execute('CREATE TABLE IF NOT EXISTS media (url TEXT PRIMARY KEY, data TEXT NOT NULL)')

    return {
        'version': "b2a8d343480cbaf075c93fd47033db7a2f020773",
        'capabilities':  ["media/source"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/source': handle_source
})
