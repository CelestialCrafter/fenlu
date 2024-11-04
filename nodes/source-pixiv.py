import re
import time
import requests
from itertools import chain
from datetime import datetime

from common import listen

def transform(post):
    dateSegment = re.sub(
        r'\+.+',
        '',
        post['updateDate'].replace('-', '/').replace('T', '/').replace(':', '/'),
    )

    return [{
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
            'page': page,
            'pixivUrl': 'https://www.pixiv.net/artworks/' + post['id']
        }
    } for page in range(post['pageCount'])]

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
    media = list(chain.from_iterable(map(transform, filter(
            lambda post: post['illustType'] == 0 and post['updateDate'] != '1970-01-01T00:00:00+09:00',
            data['body']['works']
    ))))

    return {'media': media, 'finished': len(data['body']['works']) < max}

def handle_initialize(params):
    global config
    config = params['config']

    return {
        'version': "95a247050de65c132541eabe3d93ca0b7c9b5a65",
        'capabilities':  ["media/source"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/source': handle_source,
})
