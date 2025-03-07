setup-dcss-server:
    # mkdir ./crawl
    # rm -rf ./crawl/main
    # mkdir ./crawl/main
    # git -C ./crawl/main/ clone "https://github.com/crawl/crawl.git"

    # rm -rf ./crawl/dcss-0.29
    # mkdir ./crawl/dcss-0.29
    # cp -r ./crawl/main/crawl/. ./crawl/dcss-0.29
    # git -C ./crawl/dcss-0.29 checkout stone_soup-0.29
    # make -C ./crawl/dcss-0.29/crawl-ref/source WEBTILES=y

    # rm -rf ./crawl/dcss-0.30
    # mkdir ./crawl/dcss-0.30
    # cp -r ./crawl/main/crawl/. ./crawl/dcss-0.30
    # git -C ./crawl/dcss-0.30 checkout stone_soup-0.30
    # make -C ./crawl/dcss-0.30/crawl-ref/source WEBTILES=y

    # rm -rf ./crawl/dcss-0.31
    # mkdir ./crawl/dcss-0.31
    # cp -r ./crawl/main/crawl/. ./crawl/dcss-0.31
    # git -C ./crawl/dcss-0.31 checkout stone_soup-0.31
    # make -C ./crawl/dcss-0.31/crawl-ref/source WEBTILES=y

    # rm -rf ./crawl/dcss-0.32
    # mkdir ./crawl/dcss-0.32
    # cp -r ./crawl/main/crawl/. ./crawl/dcss-0.32
    # git -C ./crawl/dcss-0.32 checkout stone_soup-0.32
    # make -C ./crawl/dcss-0.32/crawl-ref/source WEBTILES=y

    rm -rf ./crawl/server
    mkdir ./crawl/server
    cp -r ./crawl/dcss-0.32/crawl-ref/source/webserver/. ./crawl/server
    sed -i -e 's/subprocess.signal/signal/g' ./crawl/server/webtiles/process_handler.py
    sed -i -e 's/import subprocess/import signal/g' ./crawl/server/webtiles/process_handler.py
    cp ./tests/config/config.py ./crawl/server/config.py 
    cp ./tests/config/init-player.sh ./crawl/server/init-player.sh
    echo save_dir = ./crawl/server/saves-0.29 > ./crawl/server/init-0.29.txt
    echo save_dir = ./crawl/server/saves-0.30 > ./crawl/server/init-0.30.txt
    echo save_dir = ./crawl/server/saves-0.31 > ./crawl/server/init-0.31.txt
    echo save_dir = ./crawl/server/saves-0.32 > ./crawl/server/init-0.32.txt

    # rm -rf ./crawl/main

update-dcss-server:
    git -C ./crawl/ pull
    make -C ./crawl/crawl-ref/source/ WEBTILES=y

run-dcss:
    python crawl/server/server.py &

stop-dcss:
    echo stop