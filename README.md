# dcss-api

`dcss-api` is an easy to use Rust and Python wrapper for [Dungeon Crawl Stone Soup's (DCSS) Webtile](https://crawl.develz.org/) websocket API. It supports logging in, starting a game and sending commands during game play.

## Documentation

The documentation for the `dcss-api` can be found [here]() for Rust and [here] for Python. The best way to start is to look at the examples [here]() for Rust and [here]() for Python. Please see the [Setup](#setup) in order to be able to run these examples.

In depth documentation about the DCSS websocket API can also be found [here]().

## Setup

### Building DCSS Webtile

The API works for both local and public version of DCSS Webtiles. To run on a public server, you must limit the connection to a maximum of one command every 100 milliseconds (i.e. 10 commands per seconds), by setting the `speed_ms` option in while connecting. Follow any other rules required by the server's owner. 

It is therefore preferred to run the API against a local version of DCSS Webtile. You can find installation information on the [DCSS Webtiles Server page](https://github.com/crawl/crawl/tree/master/crawl-ref/source/webserver).

A summary (after installing all prerequisites):

```bash
git clone "https://github.com/crawl/crawl.git"
cd crawl/crawl-ref/source/
git checkout stone_soup-0.29
make WEBTILES=y
python webserver/server.py
```

### Building Python

https://github.com/PyO3/pyo3

```bash
mkdir pyo3
python -m venv pyo3
source pyo3/bin/activate
pip install maturin
maturin develop -r
```

## Testing

In order to run `cargo test` or `pytest`, a local DCSS Webtile, with the `stone_soup-0.29` or `stone_soup-0.30` branch, must be exposed on `localhost:8080` and two users must be created: `Username` and `Username2`, both with the password set to `Password`.
