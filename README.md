# dcss-api

This repo contains a series of libraries in Rust and Python to work with [Dungeon Crawl Stone Soup's (DCSS) Webtile](https://crawl.develz.org/).

## Crates

### [dcss-api](https://github.com/EricFecteau/dcss-api/blob/main/dcss-api/) [![Crates.io Version](https://img.shields.io/crates/v/dcss-api)](https://crates.io/crates/dcss-api) [![docs.rs](https://img.shields.io/docsrs/dcss-api)](https://docs.rs/dcss-api/0.2.0/dcss_api/) 

`dcss-api` is an easy to use Rust wrapper for DCSS Webtile websocket API. It works with version `0.29`, `0.30`, `0.31` or `0.32` of DCSS.

### [dcss-scenario-builder](https://github.com/EricFecteau/dcss-api/blob/main/dcss-api/)

`dcss-scenario-builder` is a crate to build scenarios in DCSS (wizmode) from a yaml file by providing features, items and monsters and mapping them on a tile map. This is great for testing other crates in this repository.

### [dcss-api-python](https://github.com/EricFecteau/dcss-api/tree/main/dcss-api-python) [![PyPI - Version](https://img.shields.io/pypi/v/dcss-api)](https://pypi.org/project/dcss-api/)

`dcss-api` is an easy to use Python wrapper for DCSS Webtile websocket API, that includes the `dcss-scenario-builder` functionalities. It works with version `0.29`, `0.30`, `0.31` or `0.32` of DCSS.

## Docs

Documentation about the DCSS websocket API can also be found [here](https://ericfecteau.ca/dcss-api-docs/).

## Server & testing

The [Justfile](https://github.com/EricFecteau/dcss-api/blob/main/Justfile) has commands to setup a DCSS Webtile server, with the correct configurations. Run `just setup-dcss-server` to setup the server, `just dcss-run` to run the server and `just dcss-create-users` to create the required users for testing. For the server prerequisites and other options, see the [DCSS Webtiles Server page](https://github.com/crawl/crawl/tree/master/crawl-ref/source/webserver#dungeon-crawl-stone-soup-webtiles-server). To build the python library, run `just setup-python`. Testing can be run with `just test-rust-api` for Rust and `just test-python` for Python.