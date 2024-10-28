#!/usr/bin/env python

import sqlite3
import tomllib
import json
from common import listen

with open('config-source-sqlite.toml', 'rb') as file:
    config = tomllib.load(file)

connection = sqlite3.connect(config['path'])
cursor = connection.cursor()
cursor.execute('CREATE TABLE IF NOT EXISTS media (uri TEXT PRIMARY KEY, data TEXT NOT NULL)')

def handle_generate(params):
    batch_size = params['batch_size']
    state = params['state']
    rows = cursor.execute('SELECT * FROM media LIMIT ?, ?', (state * batch_size, batch_size)).fetchall()
    media = [json.loads(text[0]) for text in rows]

    return {
        'media': media,
        'finished': len(media) < batch_size
    }

def handle_capabilities(_):
    return {'media': ('source', None)}

listen({
    'capabilities/capabilities': handle_capabilities,
    'media/generate': handle_generate
})
