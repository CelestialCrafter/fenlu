import sqlite3
import json

from common import listen

def handle_sink(params):
    cursor.executemany('INSERT OR IGNORE INTO media VALUES(?, ?)', [
        [media['url'], json.dumps(media)] for media in params
    ])
    connection.commit()

def handle_initialize(params):
    global connection
    global cursor

    connection = sqlite3.connect(params['config']['path'])
    cursor = connection.cursor()
    cursor.execute('CREATE TABLE IF NOT EXISTS media (url TEXT PRIMARY KEY, data TEXT NOT NULL)')

    return {
        'version': "95a247050de65c132541eabe3d93ca0b7c9b5a65",
        'capabilities':  ["media/sink"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/sink': handle_sink
})
