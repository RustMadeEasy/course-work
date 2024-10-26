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
            Create a Single-Player Game

        Player and Automatic Player:
            Take Turn
            Take Turn
            Take Turn

        Player:
            End Game
            Exit Gaming Session
*/

/*  Two Player session:

        Player A:
            Create a Gaming Session
            Subscribe to MQTT
            Create Two Player Game
            
        Player B:
            Join Gaming Session via invitation code
            Subscribe to MQTT
            Call Get Session Current Game
            Post Player Readiness

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
