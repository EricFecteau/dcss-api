import pytest
import dcss_api
import json
import os
from dcss_api import ScenarioErr

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

def test_wizmode():
    game_id = os.environ['GAME_ID']
    
    reset_test("Username", game_id)

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    yaml_file = "./dcss-scenario-builder/tests/test_scenarios/simple_map.yaml"
    webtile.start_game_with_scenario(game_id, "b", "f", "b", yaml_file)

    webtile.save_game()

    webtile.continue_game(game_id)

    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "player":
            assert json_message["wizard"] == 1

    webtile.quit_game()

    while (message := webtile.get_message()) != None:
        pass

    webtile.disconnect()


def test_no_character():
    game_id = os.environ['GAME_ID']

    reset_test("Username", game_id)

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    yaml_file = "./dcss-scenario-builder/tests/test_scenarios/no_char.yaml"

    try: 
        webtile.start_game_with_scenario(game_id, "b", "f", "b", yaml_file)
    except ScenarioErr as e:
        if "Missing `@` on `D:1`" in e.args[0]:
            assert True
        else:
            assert False


def test_too_wide():
    game_id = os.environ['GAME_ID']

    reset_test("Username", game_id)

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    yaml_file = "./dcss-scenario-builder/tests/test_scenarios/too_wide.yaml"

    try: 
        webtile.start_game_with_scenario(game_id, "b", "f", "b", yaml_file)
    except ScenarioErr as e:
        if "Maximum width of map is 79 columns" in e.args[0]:
            assert True
        else:
            assert False


def test_too_long():
    game_id = os.environ['GAME_ID']

    reset_test("Username", game_id)

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    yaml_file = "./dcss-scenario-builder/tests/test_scenarios/too_long.yaml"

    try: 
        webtile.start_game_with_scenario(game_id, "b", "f", "b", yaml_file)
    except ScenarioErr as e:
        if "Maximum height of map is 69 rows" in e.args[0]:
            assert True
        else:
            assert False
