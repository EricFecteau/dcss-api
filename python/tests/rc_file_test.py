import pytest
import dcss_api
from dcss_api import APIErr
import json


def test_write_read_rc():
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.set_rc_file("dcss-0.32", "this is a test")

    rc_file = webtile.get_rc_file("dcss-0.32")

    assert rc_file == "this is a test"

    while (message := webtile.get_message()) != None:
        pass

    webtile.set_rc_file("dcss-0.32", "show_more = false\nrest_delay = -1")

    rc_file = webtile.get_rc_file("dcss-0.32")

    assert rc_file == "show_more = false\nrest_delay = -1"

    webtile.disconnect()


def test_blank_rc_file():
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.32")

    while (message := webtile.get_message()) != None:
        pass

    webtile.login_with_credentials("Username", "Password")

    while (message := webtile.get_message()) != None:
        pass

    webtile.set_rc_file("dcss-0.32", "")

    rc_file = webtile.get_rc_file("dcss-0.32")

    assert rc_file == ""

    while (message := webtile.get_message()) != None:
        pass

    webtile.set_rc_file("dcss-0.32", "show_more = false\nrest_delay = -1")

    rc_file = webtile.get_rc_file("dcss-0.32")

    assert rc_file == "show_more = false\nrest_delay = -1"

    webtile.disconnect()
