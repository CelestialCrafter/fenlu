#!/usr/bin/env python

import re
import os
import requests
import tomllib

from common import listen, open_uri

with open('config-source-pixiv.toml', 'rb') as file:
    config = tomllib.load(file)

max = 100
prev_posts = max

def transform(post):
    date = re.sub(
        r'\+.+',
        '',
        post['updateDate'].replace('-', '/').replace('T', '/').replace(':', '/'),
    )

    return [({
        'title': post['title'],
        'uri': f'http://i.pximg.net/img-master/img/{date}/{post['id']}_p{page}_master1200.jpg',
        'height': post['width'],
        'width': post['height'],
        'type': 'Image',
        'tags': post['tags'],
    }, post['id']) for page in range(post['pageCount'])]

def handle_generate(params):
    global max
    global prev_posts

    state = params['state']

    if config['nsfw']:
        nsfw = 'hide'
    else:
        nsfw = 'show'

    url = f'https://www.pixiv.net/ajax/user/{config['account']['user_id']}/illusts/bookmarks?tag=&offset={max * state}&limit={max}&rest={nsfw}&lang=en'
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:131.0) Gecko/20100101 Firefox/131.0',
        'Cookie': f'PHPSESSID={config['account']['user_id']}_{config['account']['token']}',
    }

    data = requests.get(url, headers=headers).json()
    if data['error']:
        raise Exception(data['message'])

    media, extra = list(zip(*[
        x
        for post in filter(
            lambda post: post['illustType'] == 0
            and post['updateDate'] != '1970-01-01T00:00:00+09:00',
            data['body']['works'],
            )
        for x in transform(post)
    ]))

    return {'media': media, 'extra': extra, 'finished': len(data['body']['works']) < max}

def handle_open_original(params):
    open_uri('https://pixiv.net/artworks/' + params['history'][os.path.basename(__file__)])
    return {}

def handle_capabilities(_):
    return {'media': ('source', config['request_delay'] * 1000), 'actions': ['open-original']}

listen({
    'capabilities/capabilities': handle_capabilities,
    'media/generate': handle_generate,
    'actions/open-original': handle_open_original
})
