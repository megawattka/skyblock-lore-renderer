import json
import urllib.request as ur

url = "http://127.0.0.1:8080/render"
with open("./examples/hyperion.txt") as fp:
    lore = fp.read()

payload = json.dumps({
    "lore": lore,
    "options": {
        "background": "#000000",
        "recolor_enchantments": True,
    }
}).encode()

headers = {"content-type": "application/json"} 
request = ur.Request(url, data=payload, headers=headers)

rendered = ur.urlopen(request).read()
r_json = json.loads(rendered)

print(r_json)
with open("output.png", "wb") as fp:
    fp.write(__import__("base64").b64decode(r_json["image"].encode()))