import re
import time
import requests
from itertools import chain
from datetime import datetime

from common import listen, validate_config

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
    time.sleep(request_delay)

    max = 100
    state = params['state']

    url = f'https://www.pixiv.net/ajax/user/{id}/illusts/bookmarks?tag=&offset={max * state}&limit={max}&rest={nsfw}&lang=en'
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:131.0) Gecko/20100101 Firefox/131.0',
        'Cookie': f'PHPSESSID={id}_{token}',
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
    global nsfw, request_delay, id, token
    nsfw_bool, request_delay, id, token = validate_config(
        ['nsfw', 'request_delay', 'account.id', 'account.token'],
        params,
        defaults={'request_delay': 2, 'nsfw': False}
    )
    if nsfw_bool:
        nsfw = 'hide'
    else:
        nsfw = 'show'

    return {
        'version': "b2a8d343480cbaf075c93fd47033db7a2f020773",
        'capabilities':  ["media/source"]
    }

listen({
    'initialize/initialize': handle_initialize,
    'media/source': handle_source,
})
