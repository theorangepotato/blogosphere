# Blogosphere

A simple, single user, server-side-rendered RSS feed reader for the web. This is made to be a project with minimal dependencies. It should be very easy to deploy and manage, though that will come at the cost of a lot of features some people (myself included) may find valuable.

*WARNING: This project is a passion project in very alpha stages, and there is no guarantee that it works, nor that it will recieve further development.*

## Install
Currently, there is no formal installation process. You will have to build and run it yourself. To do this, see the next sections.

## Build
Just a good ol' `cargo build`. It should run on any reasonably new version of Rust:

```bash
git clone https://github.com/theorangepotato/blogosphere
cd blogoshpere
cargo build
```

## Run
Again, a classic `cargo run`. You should then be able to access it at [http://127.0.0.1:8080/](http://127.0.0.1:8080/), or whatever IP address and port you specified in `config.toml`. The feeds displayed can also be changed by modifying `config.toml`.

## License
This project is licensed under the AGPL v3+.
