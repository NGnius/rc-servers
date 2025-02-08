# Connect to Photon

## TCP packet

Header 7 bytes
Connect Data 41 bytes

### Header

0..1 =251 (mode?)
1..5 packet length (big endian)
5..6 =0 channel ID
6..7 =1 reliable? (bool; 1->reliable, 0->unreliable)

### Connect Data

0..1 =243 1 byte (mode?)
1..2 =0
2..4 Version 2 bytes (always 1 6)
4..5 =30 1 byte (client ID shifted by 1 -- 15 is ID)
5..7 client version 3 bytes (4 bytes merged into 3 -- 0 is upper 4 bits of byte 0, 1 is OR-ed with 0, byte 1 is 2, byte 2 is 3)
8..9 =0
9..41 app ID string 32 bytes (if shorter than 32 bytes/chars the remaining bytes are 0; if longer than 32 bytes/chars the remaining app ID is ignored i.e. truncated)

### Ping Response Data

0..1 =240 1 byte
1..5 tick count at time of server's receipt (big endian) (unit of game ticks?)
5..9 tick count from client's request (big endian) (unit of game ticks?)

### Ping Request Data
0..1 =240
1..5 current tick count

### Operation Request & Response

220 Region info (in)
222 Friends list info (in)
225 Join lobby info (in)
226 Join lobby info (in)
227 Join lobby info (in)
228 Master server connect success (in)
229 Lobby joined success (in)
230 Join/connect info (in)
231 Join/connect info (in)
240 Ping (both)
243 Connect (out)
244 Connect? response? (in)
254 Disconnect to Reconnect

### Parameters

#### Op code 230

192 Reset/redo Encryption? (Map<u8, Typed>)
202 Nickname (string)
225 User ID (string)
230 Master server address (string)

#### Op code always???
221 Auth token (string)

### Event

14 Concurrent user check passed, i.e. this room still has space for you




