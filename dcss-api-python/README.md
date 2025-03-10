# dcss-api

`dcss-api` is an easy to use Python wrapper for [Dungeon Crawl Stone Soup's (DCSS) Webtile](https://crawl.develz.org/) websocket API. It supports logging in, starting a game and sending commands during game play.


## Documentation

The Rust documentation for the `dcss-api` can be found [here](https://docs.rs/dcss-api/latest/dcss_api/index.html). The best way to start is to look at the examples in Python [here](https://github.com/EricFecteau/dcss-api/tree/main/dcss-api-python/examples). Please see the [setup on GitHub](https://github.com/EricFecteau/dcss-api?tab=readme-ov-file#server--testing) in order to be able to run these examples. In depth documentation about the DCSS websocket API can also be found [here](https://ericfecteau.ca/dcss-api-docs/).

## Example

This example connects to DCSS on `localhost:8080`, logs in as `Username`, starts a random game as a minotaur berserker with a mace, moves up and then back, then quits the game and disconnects.

```Python
# Connect to DCSS Webtile
webtile = dcss_api.WebtilePy("ws://localhost:8080/socket", 100, "0.32")

# Empty message queue
while (message := webtile.get_message()) != None:
    pass

# Log in (to a user called "Username", with a password "Password")
gameid = webtile.login_with_credentials("Username", "Password")

# Print the game id's that can be started
print(gameid)

# Empty message queue
while (message := webtile.get_message()) != None:
    pass

# Start a random game on 'dcss-0.32', for Minotaur berserker with a mace.
webtile.start_game(gameid[0], "b", "f", "b")

# Print the messages you get upon starting the game (should be processed)
while (message := webtile.get_message()) != None:
    print(message)

# Move up and back
webtile.write_key("key_dir_n")
webtile.write_key("key_dir_s")

# Print the messages you while moving (should be processed)
while (message := webtile.get_message()) != None:
    print(message)

# Quit game (same as dying)
webtile.quit_game()

# Print the messages after you quit game
while (message := webtile.get_message()) != None:
    print(message)

# Disconnect from webtile
webtile.disconnect()
```