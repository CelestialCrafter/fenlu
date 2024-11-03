import re
import time
import requests
from datetime import datetime

from common import listen

config = {}

def transform(post):
    dateSegment = re.sub(
        r'\+.+',
        '',
        post['updateDate'].replace('-', '/').replace('T', '/').replace(':', '/'),
    )

    return [({
        'url': f'http://i.pximg.net/img-master/img/{dateSegment}/{post['id']}_p{page}_master1200.jpg',
        'type': 'image',
        'essentialMetadata': {
            'title': post['title'],
            'creation': datetime.fromisoformat(post['createDate']).timestamp()
        },
        'typeMetadata': {
            'height': post['width'],
            'width': post['height'],
        },
        'extraMetadata': {
            'tags': post['tags'],
        }
    }, post['id']) for page in range(post['pageCount'])]

def handle_source(params):
    time.sleep(config['request_delay'])

    max = 100
    state = params['state']

    if config['nsfw']:
        nsfw = 'hide'
    else:
        nsfw = 'show'

    url = f'https://www.pixiv.net/ajax/user/{config['id']}/illusts/bookmarks?tag=&offset={max * state}&limit={max}&rest={nsfw}&lang=en'
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:131.0) Gecko/20100101 Firefox/131.0',
        'Cookie': f'PHPSESSID={config['id']}_{config['token']}',
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

def handle_initialize(params):
    global config
    config = params['config']

    return {
        'version': "95a4fc300cc044cebe957d4fbd829b822bf59a77",
        'capabilities':  ["media/source"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/source': handle_source,
})
