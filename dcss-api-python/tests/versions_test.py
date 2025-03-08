import pytest
import dcss_api
from dcss_api import BlockingErr
import json


def reset_test(username, version):
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials(username, "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game(version, "b", "i", "b")

    while (message := webtile.get_message()) != None:
        pass

    webtile.quit_game()

    while (message := webtile.get_message()) != None:
        pass

    webtile.disconnect()


def test_start_game_seeded_0_29():
    reset_test("Username", "dcss-0.29")

    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game_seeded("dcss-0.29", "1", True, "b", "i", "b")

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

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game_seeded("dcss-0.29", "158985", True, "b", "i", "b")

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


def test_start_game_seeded_0_30():
    reset_test("Username", "dcss-0.30")

    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.30")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game_seeded("dcss-0.30", "1", True, "b", "i", "b")

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

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.30")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game_seeded("dcss-0.30", "158985", True, "b", "i", "b")

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


def test_start_game_seeded_0_31():
    reset_test("Username", "dcss-0.31")

    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.31")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game_seeded("dcss-0.31", "1", True, "b", "i", "b")

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

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.31")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game_seeded("dcss-0.31", "158985", True, "b", "i", "b")

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


def test_start_game_seeded_0_32():
    reset_test("Username", "dcss-0.32")

    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game_seeded("dcss-0.32", "1", True, "b", "i", "b")

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

    webtile.start_game_seeded("dcss-0.32", "158985", True, "b", "i", "b")

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