# shome

A 'multitool' of several useful and fun commands and modules for devices you might commonly find in a home or on a LAN.

Built modularly, encourages the addition of custom commands and modules.

# Current Features

- Easy command line interface
- AirPlay device creation, flooding, and an AirPlay server
- UPnP device discovery

# Future Features

> Why is this idiot making a README for a repo that's 10% complete?

- A stronger and more configurable AirPlay server

- Act as a UPnP device
- Super niche UPnP man-in-the-middle
- Known UPnP DoS exploit

- Probably some tools for captive portals

# Build

I'm not uploading this to cargo until I get it into a more usable state. For now, `git clone` and `cargo build --release` will have to do.

# Organization

The project is a simple interactive command line that can be seen as a tree of submodules and commands, where each command is a leaf.

Each main module is placed within the core module file. For example, airplay.rs is the airplay module, and the airplay file holds commands for airplay (and could hold another submodule/CommandRouter).

Commands are structs that implement the Command trait from `core.rs`. CommandRouter is a stand-alone struct that holds a vec of both sub-routers and commands from `core.rs`.

When creating a new module, you must add a module level `router` function which creates a router and registers all sub-routers and commands. This should then be used in the parent router. For an example, look at `airplay.rs`. There is a `router` command that grabs the AirplayFlood struct from submodule flood.rs and registers it, returning the airplay router. This is directly below main, so I go back up to main and use `core::airplay::router()` to register this in main, building the tree.

# This is for fun

This repo is not as useful as a lot of the other things I like making. I'm making the repo out of curiosity for these network devices, and I hope this will help people with similar questions!

If you want to add something fun, PR and tell me what you did. Make sure to hit `cargo fmt --all` before. Super relaxed, I don't care about tests or really much else with this.