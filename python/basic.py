import dcss_api

# # Connect to DCSS Webtile
# webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 1000, "0.29")

# # Print the messages you get upon connecting
# while (message := webtile.get_message()) != None:
#     print(message)

# # Log in (to a user called "Username", with a password "Password")
# webtile.login_with_credentials("Username", "Password")

# # Print the messages you get upon connecting
# while (message := webtile.get_message()) != None:
#     print(message)

try:
    dcss_api.WebtilePy("ws://localhost:XXXX/socket", 100, "0.29")
except Exception as e:
    print(e.args)
    if e == "invalid port number":
        print("YEO")
