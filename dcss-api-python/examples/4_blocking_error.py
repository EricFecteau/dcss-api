import dcss_api
from dcss_api import BlockingErr

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

    # Start a random game on 'dcss-0.32', for Minotaur berserker with a mace.
    webtile.start_game("dcss-0.32", "b", "f", "b")

    # Print the messages you get upon starting the game (should be processed)
    while (message := webtile.get_message()) != None:
        print(message)

    # Open inventory, drop everything
    webtile.write_key("i")
    webtile.read_until("menu", None, None)
    webtile.write_key("a")
    webtile.read_until("ui-push", None, None)
    webtile.write_key("d")
    webtile.read_until("player", None, None)
    webtile.write_key("i")
    webtile.read_until("menu", None, None)
    webtile.write_key("b")
    webtile.read_until("ui-push", None, None)
    webtile.write_key("d")
    webtile.read_until("player", None, None)

    # Print the messages you get upon doing these actions (should be processed)
    while (message := webtile.get_message()) != None:
        print(message)

    # Try to pick up what was dropped.
    webtile.write_key(",")

    # Normally when picking up ONE item on the ground, you would read until
    # DCSS Webtiles returns a "input_mode" of mode = 1 (ready for input),
    # but since there are two items on the ground, a menu will pop up so you can
    # select the item to pick up(can't be easily anticipated, so dealt with using
    # a BlockingError).
    try:
        webtile.read_until("input_mode", "mode", 1)
    except BlockingErr as e:
        if e.args[0] == "Pickup":
            print("Pickup menu pop-up -- decide what to do")
            webtile.write_key("key_esc")
            webtile.read_until("msgs", None, None)
    except APIErr as e:
        print(f"API Error: {e}")

    # Print the messages you get upon picking up an item (should be processed)
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