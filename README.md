# quadratauge

A discord bot that's using **slash commands** written in **Rust** with the help of [serenity](https://github.com/serenity-rs/serenity) and [songbird](https://github.com/serenity-rs/songbird).

> WARNING: quadratauge is not production ready! It might eat your laundry.

## Features

- **Play songs** from Youtube using a url or a search phrase
- Add complete Youtube **playlists** to the song queue
- **Maintain a queue** of songs which you can **pause**, **skip**, **resume**, **loop**, ...
- Look something up in the **Urban Dictionary**
- Look up a **subreddit** and get a list of the latest posts
- Let **quadratauge** say something **inspiring**
- Slap someone with a **trout** _(old IRC gag)_
- Calculate a number in the **fibonacci** sequence
- Roll a **dice** e.g. `2d6+3`
- And play a classic game of **ping pong**

## Getting Started

**Note:**

You may have to install cmake. The best way to test if you have those installed, is to run `cargo test` or `cargo run` and see if any errors mentions either package.

---

Run the following command to start the bot:

```bash
cargo run
```

## Docker

You can also run the bot using Docker. 

```yaml
version: '3.7'

services:
  quadratauge:
    image: ghcr.io/LukasW01/quadratauge:latest
    restart: unless-stopped
    container_name: quadratauge
    environment:
      - DISCORD_TOKEN: "TOKEN" 
    labels:
      - io.containers.autoupdate=registry
```

## Configuration

The following environment variables can be set to configure the bot:

| Variable Name                 | Default Value  | Description                                               | Required |
| ----------------------------- | -------------- | --------------------------------------------------------- | -------- |
| `DISCORD_TOKEN`               |                | Discord bot token                                         | `true`   |
| `RUST_LOG`                    | `bot=info`     | Log Level                                                 | `false`  |

You can create a bot and get the token from the [Discord Developer Portal](https://discord.com/developers/applications).

## Permissions

quadratauge requires the `bot` **scope** and several permissions on a server to work properly. Therefore, ensure to set these in the developer portal during the creation of the invite link:

- `Send Messages`
- `Connect`
- `Speak`

## License

This program is licensed under the MIT-License. See the "LICENSE" file for more information
