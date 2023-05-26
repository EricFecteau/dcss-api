import dcss_api

def main():
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 100, "0.29")

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Log in (to a user called "Username", with a password "Password")
    gameid = webtile.login_with_credentials("Username", "Password")

    # Print the game id's that can be started
    print(gameid)

    # Empty message queue
    while (message := webtile.get_message()) != None:
        pass

    # Start a random game on 'dcss-web-trunk', for Minotaur berserker with a mace.
    webtile.start_game(gameid[0], "b", "i", "b")

    # Print the messages you get upon starting the game (should be processed)
    while (message := webtile.get_message()) != None:
        print(message)

    # Move up and back
    webtile.write_key("key_dir_n")
    webtile.write_key("key_dir_s")

    # Print the messages you while moving (should be processed)
    while (message := webtile.get_message()) != None:
        print(message)

    # Quit game (same as dying)
    webtile.quit_game()

    # Print the messages after you quit game
    while (message := webtile.get_message()) != None:
        print(message)

    # Disconnect from webtile
    webtile.disconnect()

if __name__ == "__main__":
    main()