#!/usr/bin/env python

import requests
from xml.etree import ElementTree

from common import listen

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

def handle_capabilities(_):
    return {'media': ('source', None), 'query': {'set': True}}

listen({
    'capabilities/capabilities': handle_capabilities,
    'media/generate': handle_generate,
    'query/set': handle_query
})
