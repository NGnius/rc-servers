# cdn

Content Delivery "Network" for images and the client's static JSON.

This doesn't do much on its own, but it does support authenticated image uploads for user and clan avatars.
The authentication uses the `token_secret.key` to prove this server and client (the other server) are part of the same rc-servers instance.
