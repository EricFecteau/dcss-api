import pytest
import dcss_api
import json
import os
from dcss_api import BlockingErr


def reset_test(username, game_id):
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials(username, "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game(game_id, "b", "f", "b")

    while (message := webtile.get_message()) != None:
        pass

    webtile.quit_game()

    while (message := webtile.get_message()) != None:
        pass

    webtile.disconnect()


def test_start_game_seeded():
    game_id = os.environ['GAME_ID']

    reset_test("Username", game_id)

    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game_seeded(game_id, "1", True, "b", "f", "b")

    msgs = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "msgs":
            msgs = json_message
            break

    found = False
    for msg in msgs["messages"]:
        if "Game seed: 1 (custom seed)" in msg["text"]:
            found = True
            break

    assert found

    webtile.quit_game()

    while (message := webtile.get_message()) != None:
        pass

    webtile.disconnect()

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game_seeded(game_id, "158985", True, "b", "f", "b")

    msgs = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "msgs":
            msgs = json_message
            break

    found = False
    for msg in msgs["messages"]:
        if "Game seed: 158985 (custom seed)" in msg["text"]:
            found = True
            break

    assert found

    webtile.quit_game()

    while (message := webtile.get_message()) != None:
        pass

    webtile.disconnect()


def test_start_game():
    game_id = os.environ['GAME_ID']

    reset_test("Username", game_id)

    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game(game_id, "b", "f", "b")

    msgs = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "msgs":
            msgs = json_message
            break

    found = False
    for msg in msgs["messages"]:
        if "Game seed" in msg["text"]:
            found = True
            break

    assert not found

    webtile.quit_game()


def test_start_game():
    game_id = os.environ['GAME_ID']

    reset_test("Username", game_id)

    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game(game_id, "b", "f", "b")

    json_message = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)

    assert json_message["msg"] == "map"

    webtile.save_game()

    json_message = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)

    assert json_message["msg"] == "go_lobby"

    webtile.continue_game(game_id)

    json_message = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)

    assert json_message["msg"] == "map"

    webtile.quit_game()

    webtile.disconnect()


def test_start_game_two_accounts():
    game_id = os.environ['GAME_ID']

    reset_test("Username", game_id)

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game(game_id, "b", "f", "b")

    msgs = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "msgs":
            msgs = json_message
            break

    found = False
    for msg in msgs["messages"]:
        if "Welcome, Username " in msg["text"]:
            found = True
            break

    assert found

    json_message = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)

    assert json_message["msg"] == "map"

    webtile.quit_game()

    webtile.disconnect()

    reset_test("Username2", game_id)

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username2", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game(game_id, "b", "f", "b")

    msgs = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "msgs":
            msgs = json_message
            break

    found = False
    for msg in msgs["messages"]:
        if "Welcome, Username2 " in msg["text"]:
            found = True
            break

    assert found

    json_message = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)

    assert json_message["msg"] == "map"

    webtile.quit_game()

    webtile.disconnect()


def test_start_game_two_accounts_combined():
    game_id = os.environ['GAME_ID']

    reset_test("Username", game_id)
    reset_test("Username2", game_id)

    webtile1 = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")
    webtile2 = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile1.get_message()) != None:
        pass
    while (message := webtile2.get_message()) != None:
        pass

    webtile1.login_with_credentials("Username", "Password")
    webtile2.login_with_credentials("Username2", "Password")

    while (message := webtile1.get_message()) != None:
        pass
    while (message := webtile2.get_message()) != None:
        pass

    webtile1.start_game(game_id, "b", "f", "b")
    webtile2.start_game(game_id, "b", "f", "b")

    msgs = None
    while (message := webtile1.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "msgs":
            msgs = json_message
            break

    found = False
    for msg in msgs["messages"]:
        if "Welcome, Username " in msg["text"]:
            found = True
            break

    assert found

    msgs = None
    while (message := webtile2.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "msgs":
            msgs = json_message
            break

    found = False
    for msg in msgs["messages"]:
        if "Welcome, Username2 " in msg["text"]:
            found = True
            break

    assert found

    json_message = None
    while (message := webtile1.get_message()) != None:
        json_message = json.loads(message)

    assert json_message["msg"] == "map"

    json_message = None
    while (message := webtile2.get_message()) != None:
        json_message = json.loads(message)

    assert json_message["msg"] == "map"

    webtile1.quit_game()
    webtile2.quit_game()

    webtile1.disconnect()
    webtile2.disconnect()


def test_real_blocking_error():
    game_id = os.environ['GAME_ID']

    reset_test("Username", game_id)

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game(game_id, "b", "f", "b")

    msgs = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "msgs":
            msgs = json_message
            break

    found = False
    for msg in msgs["messages"]:
        if "Welcome, Username " in msg["text"]:
            found = True
            break

    assert found

    json_message = None
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)

    assert json_message["msg"] == "map"

    webtile.write_key("6iadibd")

    webtile.read_until("close_all_menus", None, None)

    webtile.write_key(",")

    try:
        webtile.read_until("whatever", None, None)
        assert False
    except BlockingErr as e:
        if "Pickup" in e.args[0]:
            assert True
        else:
            assert False

    webtile.write_key("key_esc")

    webtile.quit_game()

    webtile.disconnect()
