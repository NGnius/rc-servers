# OpenJam Servers

A collection of open source servers for FreeJam games

## Usage

### CardLife

To get CardLife to use these servers, replace the `ServerConfig.json` file in the game files with [this ServerConfig.json](assets/cardlife/ServerConfig.json).

### Robocraft

To get Robocraft to use these servers, place [this servenvmulti.config](assets/robocraft/servenvmulti.config) file in the game files.

// TODO Document how to run the servers yourself

## Privacy

The minimum amount of data is (and should be) collected to provide the expected functionality.
In most cases, these means no data is collected or logged except some debug log messages in development versions.
Some personal identifiers are sent but only exist ephemerally.
The exception is Robocraft servers, which store the minimum account info possible on disk.
This includes a unique user identifier, username, vehicle data, and user configuration data.
The current PC's MAC address is also sent to the server (this is a Robocraft client feature, it is not recorded by any server).

## Development

### Robocraft

#### Extra crates

There are two crates that are expected to be in the same parent folder as this project, since they may change as a result of OpenJam server development.

- https://git.ngram.ca/OpenJam/libfj (Public-facing FreeJam API and data structures)
- https://git.ngram.ca/OpenJam/polariton (Photon Unity Network packet and server)

#### Running

Run all of the servers using their respective `run_debug.sh` scripts and use the `dev` profile in `servenvmulti.config` to point the game to your local dev servers.
The debug scripts will tell each non-HTTP server\* to run in single connection mode which makes them shut down when an unrecognised request is received or the client quits.
The default configuration will place data in `data/robocraft` and use the assets in `assets/robocraft` (relative to the project root).
When running for the first time, you will need to create the file `data/robocraft/token_secret.key`, otherwise most servers will crash on startup.
The key may contain any data, but I'd recommend against leaving it empty since it is used for cryptography.
By default, account data will be stored in a sqlite database at `data/robocraft/accounts.sqlite.db` (created automatically).
That is good enough for most development tasks, but a real database (MySQL or Postgres) is expected to be used in production.

It is possible to remove the obfuscation from the Robocraft's `Assembly-CSharp.dll` if you need to figure out how it expects the server to behave.

\* non-HTTP servers use the Photon Unity Network (PUN) packet system to communicate. These are `rc_chat`, `rc_services`, `rc_singleplayer`, `rc_social`, and their respective `_room` variants.

## Contributing

If you can program or are learning Rust, pull requests are appreciated! If you can't and would prefer not to learn, reporting issues is also welcome.

If you'd like to discuss, contact NGnius on Signal `rfc.1149` or email `ngniusness@gmail.com`. I'll make a Signal group chat if there's enough interest.

# Acknowledgements

A big thanks to the RC15 team for their work on deobfuscating Robocraft .NET assemblies and specifically to RandomScientist for hooking me up with a deobfuscated assembly. Without them, progress would have been too slow and frustrating.

Thanks for the moral support from community members who reached out and the few IRL people who I went into unnecessary detail with.

