import pytest
import dcss_api
import json


def test_successful_connect():
    # Connect to DCSS Webtile
    webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 1000, "0.29")

    assert '{"msg":"ping"}' == webtile.get_message()
    assert '{"msg":"lobby_clear"}' == webtile.get_message()
    assert '{"msg":"lobby_complete"}' == webtile.get_message()


def test_failed_connect():
    webtile = dcss_api.WebtilePy("ws://localhost:XXXX/socket", 1000, "0.29")

    # let webtile = Webtile::connect("ws://localhost:XXXX/socket", 100, "0.29");
    # assert!(webtile.is_err());

    # let webtile = Webtile::connect("ws://localhost:0000/socket", 100, "0.29");
    # assert!(webtile.is_err());
