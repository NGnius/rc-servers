# OpenJam Servers

A collection of open source servers for FreeJam games

## Usage

### CardLife

To get CardLife to use these servers, replace the `ServerConfig.json` file in the game files with [this ServerConfig.json](assets/cardlife/ServerConfig.json).

### Robocraft

To get Robocraft to use these servers, place [this servenvmulti.config](assets/robocraft/serenvmulti.config) file in the game files.

// TODO Don't expect people to run these servers on their own computer

## Privacy

The minimum amount of data is (and should be) collected to provide the expected functionality.
In most cases, these means no data is collected or logged except some debug log messages in development versions.
Some personal identifiers are sent but only exist ephemerally.
The exception is Robocraft servers, which store the minimum account info possible on disk.
This includes a unique user identifier, username, vehicle data, and user configuration data.
The current PC's MAC address is also sent to the server (this is a Robocraft client feature, it is not recorded by any server).

## Development

### Robocraft

Run all of the servers using their respective `run_debug.sh` scripts and use the `dev` profile in `servenvmulti.config` to point the game to your local dev servers.

It is possible to remove the obfuscation from the Robocraft's `Assembly-CSharp.dll` if you need to figure out how it expects the server to behave.

## Contributing

If you can program or are learning Rust, pull requests are appreciated! If you can't and would prefer not to learn, reporting issues is also welcome.

If you'd like to discuss, contact NGnius on Signal `rfc.1149` or email `ngniusness@gmail.com`. I'll make a Signal group chat if there's enough interest.

# Acknowledgements

A big thanks to the RC15 team for their work on deobfuscating Robocraft .NET assemblies and specifically to RandomScientist for hooking me up with a deobfuscated assembly. Without them, progress would have been too slow and frustrating.

Thanks for the moral support from community members who reached out and the few IRL people who I went into unnecessary detail with.

