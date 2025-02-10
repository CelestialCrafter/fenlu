from urllib.parse import quote

import requests

from common import listen, validate_config


def transform(post):
    return {
        "url": post["file_url"],
        "type": "image",
        "essentialMetadata": {"title": post["title"], "creation": 0},
        "typeMetadata": {
            "height": post["width"],
            "width": post["height"],
        },
        "extraMetadata": {
            "tags": post["tags"].split(" "),
        },
    }


def handle_source(_):
    url = f"https://gelbooru.com/index.php?page=dapi&s=post&q=index&tags={quote(query, safe='')}&api_key={token}&user_id={id}&json=1"
    headers = {
        "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:131.0) Gecko/20100101 Firefox/131.0",
        "Cookie": f"PHPSESSID={id}_{token}",
    }

    data = requests.get(url, headers=headers).json()

    return {"media": [transform(post) for post in data["post"]], "finished": True}


def handle_initialize(params):
    global query, id, token
    query, id, token = validate_config(["query", "account.id", "account.token"], params)

    return {
        "version": "b2a8d343480cbaf075c93fd47033db7a2f020773",
        "capabilities": ["media/source"],
    }


listen(
    {
        "initialize/initialize": handle_initialize,
        "media/source": handle_source,
    }
)
