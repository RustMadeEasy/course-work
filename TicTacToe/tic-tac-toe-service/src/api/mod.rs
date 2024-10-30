// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

pub(crate) mod games;
pub(crate) mod gaming_session;
pub(crate) mod docs;
pub(crate) mod health;

/*  Single Player session:
    
        Player:    
            Create a Gaming Session
            Subscribe to MQTT
            Create Single-Player Game
            Join Current Game

        Player and Automatic Player:
            Take Turn
            On Turn Taken Event: Get latest game state
            Take Turn
            On Turn Taken Event: Get latest game state
            Take Turn
            On Turn Taken Event: Get latest game state

        Player:
            End Game
            Exit Gaming Session
*/

/*  Two Player session:

        Player A:
            Create a Gaming Session
            Subscribe to MQTT
            Create Two Player Game
            Join Current Game
            Manually send Invitation Code to Player B
            
        Player B:
            Join Gaming Session via Invitation Code
            Subscribe to MQTT
            Join Current Game

        Player A:
            On Player Readiness Event: Call Get Session Current Game

        Player A and Player B:
            Take Turn
            On Turn Taken Event: Get latest game state
            Take Turn
            On Turn Taken Event: Get latest game state
            Take Turn
            On Turn Taken Event: Get latest game state
            
        Either Player:
            End Game
            
        Either Player:
            Exit Gaming Session
 */
