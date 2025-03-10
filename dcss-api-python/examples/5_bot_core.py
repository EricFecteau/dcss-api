import dcss_api
import json
import sys


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

    # Start a random game on 'dcss-0.29', for Minotaur berserker with a mace.
    webtile.start_game(gameid[0], "b", "f", "b")

    # Print the messages you get upon starting the game (should be processed)
    while (message := webtile.get_message()) != None:
        processor(message)

    # Depending on what is found in the "map" data, a move up may make sense (up to the
    # bot to decide this) -- note this may if a north wall exists (no bot intelligence here).
    write_key_bot(webtile, "key_dir_n", "player")
    write_key_bot(webtile, "key_dir_s", "player")

    # Quit game (same as dying)
    webtile.quit_game()

    # Disconnect from webtile
    webtile.disconnect()

def processor(message):
    json_message = json.loads(message)

    match json_message["msg"]:
        # Ignore
        case _ if message in ["ping", "lobby_clear", "go_lobby", "html", "set_game_links",
                              "game_client", "chat", "version", "options", "layout",
                              "ui-state-sync", "text_cursor", "cursor", "ui_state", 
                              "flash", "ui-stack", "ui-state", "update_menu_items", 
                              "close_all_menus", "delay", "menu_scroll", "ui-scroller-scroll"
                              "ui_cutoff", "lobby_complete", "login_success", "game_started"]:
            pass
        # Input & blocking
        case "input_mode":
            print("PROCESS: input_mode")
        # Messages
        case "msgs":
            print("PROCESS: game log")

        # Lobby
        case "update_spectators":
            print("PROCESS: number of spectators")

        # Player
        case "player":
            print("PROCESS: player data")

        # Dungeon
        case "map":
            print("PROCESS: map data")

        # Menu
        case _ if message in ["menu", "update_menu", "close_menu", "ui-push", "ui-pop"]:
            print("PROCESS: menu data")

def write_key_bot(webtile, to_send, to_receive):
    print("SEND: ", to_send)

    webtile.write_key(to_send)

    try:
        webtile.read_until("input_mode", "mode", 1)
    except BlockingErr as e:
        match e.args[0]:
            case "More":
                webtile.write_key(" ")
            case "TextInput":
                print("ERROR: Likely level up choice")
            case "Pickup":
                print("ERROR: Pickup")
            case "Acquirement":
                print("ERROR: Acquirement")
            case "Identify":
                print("ERROR: Identify")
            case "EnchantWeapon":
                print("ERROR: EnchantWeapon")
            case "EnchantItem":
                print("ERROR: EnchantItem")
            case "BrandWeapon":
                print("ERROR: BrandWeapon")
            case "Died":
                print("ERROR: Died")
                sys.exit()

    except APIErr as e:
        print(f"API Error: {e}")

    # Process the data based on what was done (e.g. new map revealed, health of player...)
    while (message := webtile.get_message()) != None:
        processor(message)

if __name__ == "__main__":
    main()