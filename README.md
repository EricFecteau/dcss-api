# dcss-api


## Documentation

## DCSS Webtile

The API works for both local and public version of DCSS Webtiles. To run on a public server, you must limit the connection to one command every 100 milliseconds (i.e. 10 commands per seconds), by setting the `speed_ms` option in while connecting. Follow any other rules required by the server's owner. 

It is therefore preferred to run the API against a local version of DCSS Webtile. You can find installation information on the [DCSS Webtiles Server page](https://github.com/crawl/crawl/tree/master/crawl-ref/source/webserver).

A summary (after installing all prerequisites):

```bash
git clone "https://github.com/crawl/crawl.git"
cd crawl/crawl-ref/source/
git checkout stone_soup-0.29
make WEBTILES=y
python webserver/server.py
```

## Testing

In order to run `cargo test` or `x`, a local DCSS Webtile, with the `stone_soup-0.29` or `stone_soup-0.30` branch, must be exposed on `localhost:8080` and two users must be created: `Username` and `Username2`, both with the password set to `Password`.
