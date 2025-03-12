setup-dcss-server:
    rm -rf ./crawl
    mkdir ./crawl
    mkdir ./crawl/main
    git -C ./crawl/main/ clone "https://github.com/crawl/crawl.git"

    mkdir ./crawl/dcss-0.29
    cp -r ./crawl/main/crawl/. ./crawl/dcss-0.29
    git -C ./crawl/dcss-0.29 checkout stone_soup-0.29
    make -C ./crawl/dcss-0.29/crawl-ref/source WEBTILES=y

    mkdir ./crawl/dcss-0.30
    cp -r ./crawl/main/crawl/. ./crawl/dcss-0.30
    git -C ./crawl/dcss-0.30 checkout stone_soup-0.30
    make -C ./crawl/dcss-0.30/crawl-ref/source WEBTILES=y

    mkdir ./crawl/dcss-0.31
    cp -r ./crawl/main/crawl/. ./crawl/dcss-0.31
    git -C ./crawl/dcss-0.31 checkout stone_soup-0.31
    make -C ./crawl/dcss-0.31/crawl-ref/source WEBTILES=y

    mkdir ./crawl/dcss-0.32
    cp -r ./crawl/main/crawl/. ./crawl/dcss-0.32
    git -C ./crawl/dcss-0.32 checkout stone_soup-0.32
    make -C ./crawl/dcss-0.32/crawl-ref/source WEBTILES=y

    mkdir ./crawl/server
    cp -r ./crawl/dcss-0.32/crawl-ref/source/webserver/. ./crawl/server
    sed -i -e 's/subprocess.signal/signal_module/g' ./crawl/server/webtiles/process_handler.py
    sed -i -e 's/import subprocess/import signal as signal_module/g' ./crawl/server/webtiles/process_handler.py
    cp ./crawl-config/config.py ./crawl/server/config.py 
    cp ./crawl-config/init-player.sh ./crawl/server/init-player.sh

    rm -rf ./crawl/main

dcss-run:
    python3 crawl/server/server.py

dcss-create-users:
    cd ./dcss-api && cargo run -r --example 0_setup

dcss-clear:
    rm -rf ./saves
    rm -rf ./crawl/server/rcs-0.29/*
    rm -rf ./crawl/server/rcs-0.30/*
    rm -rf ./crawl/server/rcs-0.31/*
    rm -rf ./crawl/server/rcs-0.32/*
    rm -rf ./crawl/server/saves-0.29/*
    rm -rf ./crawl/server/saves-0.30/*
    rm -rf ./crawl/server/saves-0.31/*
    rm -rf ./crawl/server/saves-0.32/*
    rm -f ./crawl/server/passwd.db3

dcss-enable-logging:
    sed -i -e 's/# type: (str, Any) -> bool/print("SENT FROM DCSS: ", msg, data)/g' ./crawl/server/webtiles/ws_handler.py

dcss-disable-logging:
    sed -i -e 's/print("SENT FROM DCSS: ", msg, data)/# type: (str, Any) -> bool/g' ./crawl/server/webtiles/ws_handler.py

test-api:
    cd ./dcss-api && GAME_ID=dcss-0.29 cargo test
    cd ./dcss-api && GAME_ID=dcss-0.30 cargo test
    cd ./dcss-api && GAME_ID=dcss-0.31 cargo test
    cd ./dcss-api && GAME_ID=dcss-0.32 cargo test

test-scenario:
    cd ./dcss-scenario-builder && GAME_ID=dcss-0.29 cargo test
    cd ./dcss-scenario-builder && GAME_ID=dcss-0.30 cargo test
    cd ./dcss-scenario-builder && GAME_ID=dcss-0.31 cargo test
    cd ./dcss-scenario-builder && GAME_ID=dcss-0.32 cargo test

setup-python:
    rm -rf ./dcss-api-python/pyo3
    mkdir ./dcss-api-python/pyo3
    python -m venv ./dcss-api-python/pyo3
    source ./dcss-api-python/pyo3/bin/activate && cd ./dcss-api-python/ && pip install maturin patchelf pytest
    source ./dcss-api-python/pyo3/bin/activate && cd ./dcss-api-python/ && maturin develop -r

test-python:
    source ./dcss-api-python/pyo3/bin/activate && GAME_ID=dcss-0.29 pytest ./dcss-api-python/tests
    source ./dcss-api-python/pyo3/bin/activate && GAME_ID=dcss-0.30 pytest ./dcss-api-python/tests
    source ./dcss-api-python/pyo3/bin/activate && GAME_ID=dcss-0.31 pytest ./dcss-api-python/tests
    source ./dcss-api-python/pyo3/bin/activate && GAME_ID=dcss-0.32 pytest ./dcss-api-python/tests

cargo-update:
    cd ./dcss-api && cargo update
    cd ./dcss-scenario-builder && cargo update
    cd ./dcss-api-python && cargo update
    cd ./dcss-data && cargo update

cargo-outdated:
    cd ./dcss-api && cargo outdated
    cd ./dcss-scenario-builder && cargo outdated
    cd ./dcss-api-python && cargo outdated
    cd ./dcss-data && cargo outdated