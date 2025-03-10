import dcss_api

def main():
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 100, "0.32")

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Log in (to a user called "Username", with a password "Password")
    _gameid = webtile.login_with_credentials("Username", "Password")

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Get cookie from the game
    cookie = webtile.request_cookie()

    print(cookie)

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Disconnect from DCSS Webtile
    webtile.disconnect()

    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 100, "0.32")

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Login with cookie
    _gameid = webtile.login_with_cookie(cookie)

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Disconnect from webtile
    webtile.disconnect()

if __name__ == "__main__":
    main()