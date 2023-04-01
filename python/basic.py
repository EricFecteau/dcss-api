import dcss_api
import json

webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 1000)


webtile.write_json(
    """{
        "msg": "login",
        "username": "Username",
        "password": "Password"
    }"""
)
webtile.read_until("login_success", None, None)


print(json.loads(webtile.print_return()))
