#!/usr/bin/env python

import sqlite3
import tomllib
import json
from common import listen

with open('config-filter-sqlite.toml', 'rb') as file:
    config = tomllib.load(file)

connection = sqlite3.connect(config['path'])
cursor = connection.cursor()
cursor.execute('CREATE TABLE IF NOT EXISTS media (uri TEXT PRIMARY KEY, data TEXT NOT NULL)')

def handle_filter(params):
    cursor.executemany('INSERT INTO media VALUES(?, ?)', [
        [media['uri'], json.dumps(media)] for media in params
    ])
    connection.commit()

    return {'included': [True] * len(params)}

def handle_capabilities(_):
    return {'media': ('filter', None)}

listen({
    'capabilities/capabilities': handle_capabilities,
    'media/filter': handle_filter
})
