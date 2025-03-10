# dcss-scenario-builder

`dcss-scenario-builder` creates scenarios in [Dungeon Crawl Stone Soup's (DCSS) Webtile](https://crawl.develz.org/) using [dcss-api](https://docs.rs/dcss-api/latest/dcss_api/index.html) in `wizmode` from a yaml file. It can create any floor layout, from any feature, and add any item or monster. 

## Example

This example will create a two floor dungeon (`D:1` and `D:2`), with a scroll and sword on the first floor (second room, past the door) and with a Kobold on the second floor.

```yaml
options:
  default_feature: "floor"

levels:
  - level:
      name: D:1
      features:
        - '< = exit_dungeon'
        - '> = stone_stairs_down_i'
        - '# = rock_wall'
        - '. = floor'
        - '+ = closed_door'
      items:
        - 'x = Scroll of identify'
      map: |-
          #########
          #...#...#
          #.<.#.x.#
          #...#...#
          #.@.+...#
          #...#...#
          #...#.>.#
          #...#...#
          #########

  - level:
      name: D:2
      features:
        - '< = stone_stairs_up_i'
        - '# = rock_wall'
        - '. = floor'
      monsters:
        - 'k = Kobold'
      map: |-
          #########
          #.......#
          #.<.....#
          #.......#
          #.......#
          #.......#
          #.....k.#
          #.......#
          #########
```

`start_game_with_scenario`, in combination with the `Webtile` from `dcss-api` is used to build the scenario in Rust.

```Rust
// Connect to DCSS Webtile
let mut webtile = Webtile::connect("ws://localhost:8080/socket", 0, "0.30")?;

// Empty message queue;
while webtile.get_message().is_some() {}

// Log in (to a user called "Username", with a password "Password")
let _gameid = webtile.login_with_credentials("Username", "Password")?;

// Create scenario
start_game_with_scenario(
    &mut webtile,
    "dcss-0.32",
    "b",
    "i",
    "c",
    "./scenarios/docs/readme.yaml",
)?;

// Empty message queue;
while webtile.get_message().is_some() {}

webtile.disconnect().expect("Failed");
```