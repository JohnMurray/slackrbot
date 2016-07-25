# SlackrBot

[![Build Status](https://travis-ci.org/JohnMurray/slackrbot.svg?branch=master)](https://travis-ci.org/JohnMurray/slackrbot)

A library for building Slack bots in Rust.

A simple example for getting started


```rust

SlackRBot::new("UNIQUE_CLIENT_ID", "UNIQUE_CLIENT_SECRET", vec!["channels:read", "channels:write"])
    .handle(|m: Message| {
        match m {
            Connect              => // join channels
            ChannelJoin(channel) => // say hello!
            _                    => // do nothing
        }
    })
    .listen(port);

```
