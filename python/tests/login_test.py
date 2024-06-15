import pytest
import dcss_api
from dcss_api import APIErr
import json


def test_successful_credential_login():
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    webtile.get_message()

    assert (
        '{"admin":false,"msg":"login_success","username":"Username"}'
        == webtile.get_message()
    )

    webtile.disconnect()


def test_multiple_login_same_user():
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    webtile.get_message()

    assert (
        '{"admin":false,"msg":"login_success","username":"Username"}'
        == webtile.get_message()
    )

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    found = False
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "login_success":
            base_message = '{"admin":false,"msg":"login_success","username":"Username"}'
            if base_message == message:
                found = True

    assert found

    webtile.disconnect()


def test_multiple_login_diff_user():
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    webtile.get_message()

    assert (
        '{"admin":false,"msg":"login_success","username":"Username"}'
        == webtile.get_message()
    )

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username2", "Password")

    found = False
    while (message := webtile.get_message()) != None:
        json_message = json.loads(message)
        if json_message["msg"] == "login_success":
            base_message = (
                '{"admin":false,"msg":"login_success","username":"Username2"}'
            )
            if base_message == message:
                found = True

    assert found

    webtile.disconnect()


def test_failed_credential_login():
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    try:
        webtile.login_with_credentials("XXXXXXX", "XXXXXXX")
        assert False
    except APIErr as e:
        if "Failed to login" in e.args[0]:
            assert True
        else:
            assert False

    webtile.disconnect()


def test_failed_credential_login_and_retry():
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    try:
        webtile.login_with_credentials("XXXXXXX", "XXXXXXX")
        assert False
    except APIErr as e:
        if "Failed to login" in e.args[0]:
            assert True
        else:
            assert False

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    assert (
        '{"admin":false,"msg":"login_success","username":"Username"}'
        == webtile.get_message()
    )

    webtile.disconnect()


def test_get_cookie_and_login():
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    webtile.get_message()

    assert (
        '{"admin":false,"msg":"login_success","username":"Username"}'
        == webtile.get_message()
    )

    cookie = webtile.request_cookie()

    assert "Username%" == cookie[0:9]

    webtile.disconnect()

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_cookie(cookie)

    webtile.get_message()

    assert (
        '{"admin":false,"msg":"login_success","username":"Username"}'
        == webtile.get_message()
    )

    webtile.disconnect()


def test_failed_cookie_login():
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    try:
        webtile.login_with_cookie("Username%123456789123456789123456789")
        assert False
    except APIErr as e:
        if "Failed to login" in e.args[0]:
            assert True
        else:
            assert False


def test_using_old_cookie_login():
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    webtile.get_message()

    assert (
        '{"admin":false,"msg":"login_success","username":"Username"}'
        == webtile.get_message()
    )

    first_cookie = webtile.request_cookie()

    assert "Username%" == first_cookie[0:9]

    webtile.disconnect()

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_cookie(first_cookie)

    webtile.get_message()

    assert (
        '{"admin":false,"msg":"login_success","username":"Username"}'
        == webtile.get_message()
    )

    second_cookie = webtile.request_cookie()

    assert "Username%" == second_cookie[0:9]

    webtile.disconnect()

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    try:
        webtile.login_with_cookie(first_cookie)
        assert False
    except APIErr as e:
        if "Failed to login" in e.args[0]:
            assert True
        else:
            assert False

    webtile.disconnect()


def test_credential_login_gameid():
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    gameid = webtile.login_with_credentials("Username", "Password")

    if len(gameid) == 5:
        assert gameid == [
            "dcss-web-trunk",
            "seeded-web-trunk",
            "descent-web-trunk",
            "tut-web-trunk",
            "sprint-web-trunk",
        ]
    else:
        assert gameid == [
            "dcss-web-trunk",
            "seeded-web-trunk",
            "tut-web-trunk",
            "sprint-web-trunk",
        ]

    webtile.disconnect()


def test_cookie_login_gameid():
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    gameid = webtile.login_with_credentials("Username", "Password")

    if len(gameid) == 5:
        assert gameid == [
            "dcss-web-trunk",
            "seeded-web-trunk",
            "descent-web-trunk",
            "tut-web-trunk",
            "sprint-web-trunk",
        ]
    else:
        assert gameid == [
            "dcss-web-trunk",
            "seeded-web-trunk",
            "tut-web-trunk",
            "sprint-web-trunk",
        ]

    cookie = webtile.request_cookie()

    assert "Username%" == cookie[0:9]

    webtile.disconnect()

    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    while (message := webtile.get_message()) != None:
        pass

    gameid = webtile.login_with_cookie(cookie)

    if len(gameid) == 5:
        assert gameid == [
            "dcss-web-trunk",
            "seeded-web-trunk",
            "descent-web-trunk",
            "tut-web-trunk",
            "sprint-web-trunk",
        ]
    else:
        assert gameid == [
            "dcss-web-trunk",
            "seeded-web-trunk",
            "tut-web-trunk",
            "sprint-web-trunk",
        ]

    webtile.disconnect()
