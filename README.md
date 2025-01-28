# OpenJam Servers

A collection of open source servers for FreeJam games

## Usage

### CardLife

To get CardLife to use these servers, replace the ServerConfig.json file in the game files with [this ServerConfig.json](assets/cardlife/ServerConfig.json).

### Robocraft

To get Robocraft to use these servers, please add the following to your OS's `hosts` file:

```
127.0.0.1       robocraftstaticdata.s3.amazonaws.com
127.0.0.1       services-1.servers.robocraftgame.com
```

The `hosts` file can be found at `/etc/hosts` on Linux and `C:\Windows\system32\drivers\etc\hosts` on Windows. Usually this requires elevated permissions (root/admin) to edit.

// TODO Don't expect people to run these servers on their own computer

## Privacy

No data is collected or logged, except in dev mode. Some personal identifiers are sent but only exist ephemerally.
