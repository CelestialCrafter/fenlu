#!/usr/bin/env python

import re
import sys
import traceback
import requests
import json
import tomllib

with open("config-source-pixiv.toml", "rb") as file:
    config = tomllib.load(file)

max = 100
prev_posts = max

def handle_generate(params):
    global max
    global prev_posts

    state = params["state"]

    if config["nsfw"]:
        nsfw = "hide"
    else:
        nsfw = "show"

    url = f'https://www.pixiv.net/ajax/user/{config["account"]["user_id"]}/illusts/bookmarks?tag=&offset={max * state}&limit={max}&rest={nsfw}&lang=en'
    headers = {
        "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:131.0) Gecko/20100101 Firefox/131.0",
        "Cookie": f'PHPSESSID={config["account"]["user_id"]}_{config["account"]["token"]}',
    }

    data = requests.get(url, headers=headers).json()
    if data["error"]:
        raise Exception(data["message"])

    media = [
        item
        for post in filter(
            lambda post: post["illustType"] == 0
            and post["updateDate"] != "1970-01-01T00:00:00+09:00",
            data["body"]["works"],
            )
        for item in transform(post)
    ]

    return {"media": media, "finished": len(data["body"]["works"]) < max}

def transform(post):
    date = re.sub(
        r"\+.+",
        "",
        post["updateDate"].replace("-", "/").replace("T", "/").replace(":", "/"),
    )

    return [{
        "title": post["title"],
        "uri": f'http://i.pximg.net/img-master/img/{date}/{post["id"]}_p{page}_master1200.jpg',
        "height": post["width"],
        "width": post["height"],
        "type": "Image",
        "tags": post["tags"],
    } for page in range(post["pageCount"])]


def handle_capabilities():
    return {"media": {"source": True}}


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
            case _:
                raise Exception("unknown method")
    except Exception:
        result = None
        error = traceback.format_exc()

    print(json.dumps({"id": id, "result": result, "error": error}))
