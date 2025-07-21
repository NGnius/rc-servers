# OpenJam Servers

![Revolt chat](https://img.shields.io/revolt/invite/jtVE0pD5?label=chat&style=flat-square)
![release version](https://img.shields.io/gitea/v/release/OpenJam/servers?gitea_url=https%3A%2F%2Fgit.ngram.ca%2F&style=flat-square&label=release)
![indev version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fgit.ngram.ca%2FOpenJam%2Fservers%2Fraw%2Fbranch%2Fmain%2FCargo.toml&query=workspace.package.version&label=indev&style=flat-square)


A collection of open source servers for FreeJam games

## Usage

### CardLife

To get CardLife to use these servers, replace the `ServerConfig.json` file in the game files with [this ServerConfig.json](assets/cardlife/ServerConfig.json).

### Robocraft

To get Robocraft to use these servers, place [this servenvmulti.config](assets/robocraft/servenvmulti.config) file in the game files.
You may need to change the `activegroup` from `dev` to `ngram` (or vice versa for locally-hosted servers).

## Privacy

The minimum amount of data is (and should be) collected to provide the expected functionality.
In most cases, this means no data is collected or logged except some debug log messages in development versions.
Some personal identifiers, such as IP addresses and session tokens, are sent but only exist ephemerally.
The exception is Robocraft servers, which store the minimum account info possible.
This includes a unique user identifier, username, vehicle data, and user configuration data.
If email and/or Steam account identifier are provided, those are also stored.
The current PC's MAC address is also sent to the server (this is a Robocraft client feature, it is not recorded by any server).

## Development

Please refer to the [wiki](/OpenJam/servers/wiki/Dev).

## Contributing

If you can program or are learning Rust, pull requests are appreciated! If you can't and would prefer not to learn, reporting issues is also welcome.

Note: New users cannot create repositories on this git server and so can't create pull requests. If you're interested in contributing code, please ask NGnius to lift that restriction on your account.

If you'd like to discuss, join the [Signal group](https://signal.group/#CjQKIEPim2GPSftMpRv03dhesLxwY9v7TWo2zyBVc8MhaC_zEhDWJ5kEkYBEsj4Fa-0gLcXs) for the project.

# Acknowledgements

A big thanks to the RC15 team for their work on deobfuscating Robocraft .NET assemblies and specifically to RandomScientist for hooking me up with a deobfuscated assembly. Without them, progress would have been too slow and frustrating.

Thanks for the moral support from community members who reached out and the few IRL people who I went into unnecessary detail with.

