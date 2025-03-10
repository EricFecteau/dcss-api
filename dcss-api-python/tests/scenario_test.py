import pytest
import dcss_api
import json
from dcss_api import ScenarioErr

def reset_test(username):
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials(username, "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.start_game("dcss-0.32", "b", "f", "b")

    while (message := webtile.get_message()) != None:
        pass

    webtile.quit_game()

    while (message := webtile.get_message()) != None:
        pass

    webtile.disconnect()

def test_wizmode():
    reset_test("Username")

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    yaml_file = "./dcss-scenario-builder/tests/test_scenarios/simple_map.yaml"
    webtile.start_game_with_scenario("dcss-0.32", "b", "f", "b", yaml_file)

    webtile.save_game()

    webtile.continue_game("dcss-0.32")

    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "player":
            assert json_message["wizard"] == 1

    webtile.quit_game()

    while (message := webtile.get_message()) != None:
        pass

    webtile.disconnect()


def test_no_character():
    reset_test("Username")

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    yaml_file = "./dcss-scenario-builder/tests/test_scenarios/no_char.yaml"

    try: 
        webtile.start_game_with_scenario("dcss-0.32", "b", "f", "b", yaml_file)
    except ScenarioErr as e:
        if "Missing `@` on `D:1`" in e.args[0]:
            assert True
        else:
            assert False


def test_too_wide():
    reset_test("Username")

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    yaml_file = "./dcss-scenario-builder/tests/test_scenarios/too_wide.yaml"
    webtile.start_game_with_scenario("dcss-0.32", "b", "f", "b", yaml_file)

    # try: 
    #     webtile.start_game_with_scenario("dcss-0.32", "b", "f", "b", yaml_file)
    # except ScenarioErr as e:
    #     if "Missing `@` on `D:1`" in e.args[0]:
    #         assert True
    #     else:
    #         assert False


# def test_too_long():
#     reset_test("Username")

#     webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

#     while (message := webtile.get_message()) != None:
#         pass

#     webtile.login_with_credentials("Username", "Password")

#     while (message := webtile.get_message()) != None:
#         pass

#     yaml_file = "./dcss-scenario-builder/tests/test_scenarios/too_long.yaml"
#     webtile.start_game_with_scenario("dcss-0.32", "b", "f", "b", yaml_file)

#     # try: 
#     #     webtile.start_game_with_scenario("dcss-0.32", "b", "f", "b", yaml_file)
#     # except ScenarioErr as e:
#     #     if "Missing `@` on `D:1`" in e.args[0]:
#     #         assert True
#     #     else:
#     #         assert False
