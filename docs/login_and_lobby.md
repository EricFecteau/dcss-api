# Navigating the lobby

## Introduction

Once [connected](connection.md) to the DCSS webtile, the game will send various messages and will expect some interactions. 

## First messages and anatomy of a message

The first few messages that are received, before the user sends any commands, are the following.

```json
RECEIVED:
    {'msg': 'ping'}
    {'msg': 'lobby_clear'}
    {'msg': 'lobby_complete'}
```

All messages received from DCSS webtiles will have the `msg` dictionary key. This will indicate what type of message it is. The above three are simply information about the connection and the state of the lobby. Other messages will have other dictionary keys for additional information. Once the `lobby_complete` message is received, DCSS is ready to receive the next commands.

## Login

There are two ways of login in. The first way, and the way that must be completed first, is with a username and password combo. When sending the following (where this particular user is called `Username` and the password for that user is `Password`), you will receive a `login_success` message, if successful and a `login_fail`, if unsuccessful.

```json
SENT: 
    {'msg': 'login', 'username': 'Username', 'password': 'Password'}
RECEIVED (if good): 
    {'username': 'Username', 'admin': False, 'msg': 'login_success'}
RECEIVED (if bad): 
    {'msg': 'login_fail'}
```

The second way, is to log in with a [cookie](https://en.wikipedia.org/wiki/HTTP_cookie). Once you log on successfully, you can request a cookie from DCSS webtile in the following way: 

```json
SENT: 
    {'msg': 'set_login_cookie'}
RECEIVED: 
    {'cookie': 'Username%20123075677009872810135587785129954954508', 'expires': 7, 'msg': 'login_cookie'}
```

Once you have a cookie, you can send it back to log on the next time (you will receive a login_success). A new login cookie must be requested every time you use the cookie.

```json
SENT: 
    {'msg': 'token_login', 'cookie': 'Username%20123075677009872810135587785129954954508'}
RECEIVED (if good cookie): 
    {'username': 'Username', 'admin': False, 'msg': 'login_success'}
RECEIVED (if bad cookie):
    {'msg': 'login_fail'}
```

## Starting a game

Once you are logged on, you will received various html messages, such as the banner. The important one is the `set_game_links` message. In this message, you will receive the `game_id`. In the `content` of that message, you will find game_ids, such as `#play-dcss-web-trunk` and `#play-seeded-web-trunk` as html. If the `#play-` portion is removed, this can be sent back to DCSS webtile to start the game. 

```json
SENT: 
    {'msg': 'play', 'game_id': 'seeded-web-trunk'}
RECEIVED (if good game_id): 
    {'msg': 'game_started'}
RECEIVED (if bad game_id):
    {'msg': 'go_lobby'}
```

Once the game is started, it will send various data such as `player` information, `ui` messages, `msgs` from the log and `map` data and the game will accept any of the standard keyboard inputs.