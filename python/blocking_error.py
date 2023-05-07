import dcss_api
from dcss_api import BlockingErr, APIErr

print(dcss_api.WebtilePy.read_until.__doc__)


# # Connect to DCSS Webtile
# webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 100, "0.29")

# # Print the messages you get upon connecting
# while (message := webtile.get_message()) != None:
#     print(message)

# # Log in (to a user called "Username", with a password "Password")
# webtile.login_with_credentials("Username", "Password")

# # Print the messages you get upon connecting
# while (message := webtile.get_message()) != None:
#     print(message)

# # Start game
# webtile.write_json("""{"msg": "play", "game_id": "seeded-web-trunk"}""")
# webtile.read_until("player", None, None)
# webtile.write_key("-")
# webtile.read_until("ui-state-sync", None, None)
# webtile.write_key("1")
# webtile.write_key("\t\t\t \r")
# webtile.read_until("ui-push", None, None)
# webtile.write_key("b")
# webtile.write_key("i")
# webtile.write_key("b")
# webtile.read_until("input_mode", "mode", 1)

# webtile.write_key("6")
# webtile.write_key("i")
# webtile.write_key("a")
# webtile.write_key("d")
# webtile.write_key("i")
# webtile.write_key("b")
# webtile.write_key("d")
# webtile.write_key(",")

# try:
#     webtile.read_until("ignore", None, None)
# except BlockingErr as e:
#     if e.args[0] == "Pickup":
#         print(f"Blocking (Pickup): {e}")
#     else:
#         print(f"Blocking: {e}")
# except APIErr as e:
#     print(f"API Error: {e}")
