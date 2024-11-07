import sqlite3
import json

from common import listen, validate_config

def handle_sink(params):
    cursor.executemany('INSERT OR IGNORE INTO media VALUES(?, ?)', [
        [media['url'], json.dumps(media)] for media in params
    ])
    connection.commit()

def handle_initialize(params):
    global connection
    global cursor

    path, = validate_config(['path'], params)
    connection = sqlite3.connect(path)
    cursor = connection.cursor()
    cursor.execute('CREATE TABLE IF NOT EXISTS media (url TEXT PRIMARY KEY, data TEXT NOT NULL)')

    return {
        'version': "b2a8d343480cbaf075c93fd47033db7a2f020773",
        'capabilities':  ["media/sink"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/sink': handle_sink
})
