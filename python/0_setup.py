import dcss_api

def main():
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 100, "0.32")

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Register usernames for tests
    webtile.register_account("Username", "Password", None)
    webtile.register_account("Username2", "Password", None)
    webtile.register_account("Username3", "Password", None)
    webtile.register_account("Username4", "Password", None)

    # Disconnect from webtile
    webtile.disconnect()

if __name__ == "__main__":
    main()