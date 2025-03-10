import dcss_api

def main():
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 100, "0.32")

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Log in (to a user called "Username", with a password "Password")
    gameid = webtile.login_with_credentials("Username", "Password")

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Create scenario
    webtile.start_game_with_scenario("dcss-0.32", "b", "f", "b", "./examples/scenarios/readme.yaml")

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    webtile.disconnect()

if __name__ == "__main__":
    main()