import json
import sqlite3
from common import listen

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
    connection = sqlite3.connect(params['config']['path'])
    cursor = connection.cursor()
    cursor.execute('CREATE TABLE IF NOT EXISTS media (url TEXT PRIMARY KEY, data TEXT NOT NULL)')

    return {
        'version': "57969bae27de229c075fcba919924838f61ef2ff",
        'capabilities':  ["media/source"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/source': handle_source
})
