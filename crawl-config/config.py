import logging
import os
import collections
import yaml

server_path = os.path.dirname(os.path.abspath(__file__))

bind_nonsecure = True
bind_address = ""
bind_port = 8080

ssl_options = None # No SSL

ssl_address = ""
ssl_port = 8443

password_db = "./crawl/server/passwd.db3"

static_path = "./crawl/server/static"
template_path = "./crawl/server/templates/"

server_id = ""

game_data_no_cache = True

games = collections.OrderedDict([
    ("dcss-0.29", dict(
        version = "0.29",
        name = "Play 0.29",
        crawl_binary = "./crawl/dcss-0.29/crawl-ref/source/crawl",
        rcfile_path = "./crawl/server/rcs-0.29/",
        macro_path = "./crawl/server/rcs-0.29/",
        morgue_path = "./crawl/server/rcs-0.29/%n",
        inprogress_path = "./crawl/server/rcs-0.29/running",
        ttyrec_path = "./crawl/server/rcs-0.29/ttyrecs/%n",
        socket_path = "./crawl/server/rcs-0.29/",
        client_path = "./crawl/dcss-0.29/crawl-ref/source/webserver/game_data/",
        morgue_url = None,
        show_save_info = True,
        allowed_with_hold = True,
        options = ["-seed"],
        )),
    ("dcss-0.30", dict(
        version = "0.30",
        name = "Play 0.30",
        crawl_binary = "./crawl/dcss-0.30/crawl-ref/source/crawl",
        rcfile_path = "./crawl/server/rcs-0.30/",
        macro_path = "./crawl/server/rcs-0.30/",
        morgue_path = "./crawl/server/rcs-0.30/%n",
        inprogress_path = "./crawl/server/rcs-0.30/running",
        ttyrec_path = "./crawl/server/rcs-0.30/ttyrecs/%n",
        socket_path = "./crawl/server/rcs-0.30/",
        client_path = "./crawl/dcss-0.30/crawl-ref/source/webserver/game_data/",
        morgue_url = None,
        show_save_info = True,
        allowed_with_hold = True,
        options = ["-seed"],
        )),
    ("dcss-0.31", dict(
        version = "0.31",
        name = "Play 0.31",
        crawl_binary = "./crawl/dcss-0.31/crawl-ref/source/crawl",
        rcfile_path = "./crawl/server/rcs-0.31/",
        macro_path = "./crawl/server/rcs-0.31/",
        morgue_path = "./crawl/server/rcs-0.31/%n",
        inprogress_path = "./crawl/server/rcs-0.31/running",
        ttyrec_path = "./crawl/server/rcs-0.31/ttyrecs/%n",
        socket_path = "./crawl/server/rcs-0.31/",
        client_path = "./crawl/dcss-0.31/crawl-ref/source/webserver/game_data/",
        morgue_url = None,
        show_save_info = True,
        allowed_with_hold = True,
        options = ["-seed"],
        )),
    ("dcss-0.32", dict(
        version = "0.32",
        name = "Play 0.32",
        crawl_binary = "./crawl/dcss-0.32/crawl-ref/source/crawl",
        rcfile_path = "./crawl/server/rcs-0.32/",
        macro_path = "./crawl/server/rcs-0.32/",
        morgue_path = "./crawl/server/rcs-0.32/%n",
        inprogress_path = "./crawl/server/rcs-0.32/running",
        ttyrec_path = "./crawl/server/rcs-0.32/ttyrecs/%n",
        socket_path = "./crawl/server/rcs-0.32/",
        client_path = "./crawl/dcss-0.32/crawl-ref/source/webserver/game_data/",
        morgue_url = None,
        show_save_info = True,
        allowed_with_hold = True,
        options = ["-seed"],
        )),
])

dgl_status_file = "./crawl/server/status"

init_player_program = "./crawl/server/init-player.sh"

lobby_url = "http://localhost:8080/"

smtp_host = "localhost"
smtp_port = 25
smtp_use_ssl = False
smtp_user = "" # set to None for no auth
smtp_password = ""
smtp_from_addr = "noreply@crawl.example.org"

uid = None  # If this is not None, the server will setuid to that (numeric) id
gid = None  # after binding its sockets.

umask = None # e.g. 0077

chroot = None

pidfile = None

player_url = None

hup_reloads_config = True

autologin = None
