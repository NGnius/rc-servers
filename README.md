# OpenJam Servers

[![Stoat chat](https://img.shields.io/revolt/invite/jtVE0pD5?label=chat&style=flat-square)](https://stt.gg/jtVE0pD5)
[![release version](https://img.shields.io/gitea/v/release/OpenJam/servers?gitea_url=https%3A%2F%2Fgit.ngram.ca%2F&style=flat-square&label=release)](https://git.ngram.ca/OpenJam/rc-servers/releases)
[![indev version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fgit.ngram.ca%2FOpenJam%2Frc-servers%2Fraw%2Fbranch%2Fmain%2FCargo.toml&query=workspace.package.version&label=indev&style=flat-square)](#)
[![patrons](https://img.shields.io/liberapay/patrons/NGram.svg?logo=liberapay)](https://liberapay.com/NGram/)


The official servers shut down in January 2025, but [Robocraft](https://store.steampowered.com/app/301520/Robocraft/) never stops.
This project re-implements Robocraft web services to get the latest version working again in ${current_year}.
This is how you sunset a game, FreeJam.

The main repository is https://git.ngram.ca/OpenJam/rc-servers -- anything else is a mirror or a fork.
Some links in this document may not work on mirrors/forks.

## Usage

Please refer to the latest [release](/OpenJam/rc-servers/releases) for instructions for accessing the public OpenJam server instances.

If you're trying to run your own local server, refer to the [quickstart guide](https://git.ngram.ca/OpenJam/rc-servers/wiki/Dev).

## Privacy

The minimum amount of data is (and should be) collected to provide the expected functionality.
This means no data is collected or logged except some debug log messages in development versions.
Some personal identifiers, such as IP addresses and session tokens, are sent but only exist ephemerally.
The Robocraft servers store the minimum account info possible.
This includes username, vehicle data, user configuration data, and other non-identifying gameplay data.
If email and/or Steam account identifier are provided during registration, those are also stored.
This information is all associated with a unique user identifier.
The current PC's MAC address is also sent to the server (this is a Robocraft client feature, it is not recorded by any server).

## Contributing

Please refer to the [wiki](/OpenJam/rc-servers/wiki/Dev), especially the page on [contributing](https://git.ngram.ca/OpenJam/rc-servers/wiki/Contributing).

# Acknowledgements

A big thanks to the [RC15](https://discord.gg/jZHDAaacS5) team for their work on reverse-engineering early Robocraft .NET assemblies.
Thanks to KptKosmit91 (a RC15 team member) for giving me a working battle arena team base.
Thanks to RandomScientist for hooking me up with a deobfuscated assembly.
Without them, progress would have been too slow and frustrating.

Thanks for the moral support from community members who reached out and the few IRL people who I went into unnecessary detail with.

