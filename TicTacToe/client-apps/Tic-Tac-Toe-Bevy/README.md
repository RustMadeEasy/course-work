# Tic-Tac-Toe Bevy Client App

## Description

A Tic-Tac-Toe client app written in Rust, using the Bevy game engine.

NOTE: This is sample code, part of the RustMadeEasy.com course: Intro to Rust. This code is not suitable for
production.

Roadmap:

1. Refactor the ServiceClient so that none of its methods return models from the client SDK. We want to do this so that the app code is separated from client SDK code.  
2. Resilient communication with the service - retries, etc.
3. Option to replay the game once it has ended - using the Tic-Tac-Toe service's Game History endpoint.
4. Support MQTT for game state updates once it is supported by the Tic-Tac-Toe service.
5. Publish the StatusTextPlugin to crates.io as open-source.
6. Tell local player when the other player has abandoned the game.
