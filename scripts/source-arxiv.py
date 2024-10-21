#!/usr/bin/env python

import sys
import traceback
import requests
import json
from xml.etree import ElementTree

def transform(paper):
    return {
        'title': paper.findtext('{*}title'),
        'uri': paper.find("{*}link[@title='pdf']").attrib['href'],
        'author': paper.findtext('.//{*}author/{*}name'),
        'summary': paper.findtext('{*}summary'),
        'tags': [category.attrib['term'] for category in paper.findall('{*}category')],
        'type': 'PDF',
    }

query = ''

def handle_query(params):
    global query
    query = params['query']
    return

def handle_generate(params):
    global query
    if query.strip() == '':
        return {
            'media': [],
            'finished': True
        }

    state = params['state']
    batch_size = params['batch_size']

    url = f'http://export.arxiv.org/api/query?start={state * batch_size}&max_results={batch_size}&search_query={query}'
    response = requests.get(url).text
    root = ElementTree.fromstring(response)

    return {
        'media': [transform(paper) for paper in root.findall('{*}entry')],
        'finished': True,
    }

def handle_capabilities():
    return {"media": ("source", None), "query": {"set": True}}

for line in sys.stdin:
    # strip off EOF
    line = line.rstrip()

    request = json.loads(line)
    params = request["params"]
    id = request["id"]

    result = {}
    error = None

    try:
        match request["method"]:
            case "capabilities/capabilities":
                result = handle_capabilities()
            case "media/generate":
                result = handle_generate(params)
            case "query/set":
                result = handle_query(params)
            case _:
                raise Exception("unknown method")
    except Exception:
        result = None
        error = traceback.format_exc()

    print(json.dumps({"id": id, "result": result, "error": error}))
