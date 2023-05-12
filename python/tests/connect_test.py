import pytest
import dcss_api
from dcss_api import APIErr


def test_successful_connect():
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 0, "0.29")

    assert '{"msg":"ping"}' == webtile.get_message()
    assert '{"msg":"lobby_clear"}' == webtile.get_message()
    assert '{"msg":"lobby_complete"}' == webtile.get_message()

    webtile.disconnect()


def test_failed_connect():
    try:
        webtile = dcss_api.WebtilePy("ws://localhost:XXXX/socket", 0, "0.29")
        assert False
    except APIErr as e:
        if "Url error" in e.args[0]:
            assert True
        else:
            assert False

    try:
        webtile = dcss_api.WebtilePy("ws://localhost:0000/socket", 0, "0.29")
        assert False
    except APIErr as e:
        if "Tungstenite error" in e.args[0]:
            assert True
        else:
            assert False
