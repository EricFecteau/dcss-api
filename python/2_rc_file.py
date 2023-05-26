import dcss_api

def main():
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 100, "0.29")

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Log in (to a user called "Username", with a password "Password")
    _gameid = webtile.login_with_credentials("Username", "Password")

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Write RC File
    webtile.set_rc_file("seeded-web-trunk", "show_more = false\nrest_delay = -1")

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Read RC File
    rc_file = webtile.get_rc_file("seeded-web-trunk")

    print("RC FILE: \n\n", rc_file, "\n\n")

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Disconnect from webtile
    webtile.disconnect()

if __name__ == "__main__":
    main()