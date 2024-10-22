pub mod api_games;
pub mod api_gaming_session;
pub mod api_health_and_docs;

/*  Single Player session:
    
        Player A:    
            Create a Gaming Session
            Subscribe to MQTT
            Create a Single-Player Game

        Player B:
            Join Gaming Session via invitation code
            Subscribe to MQTT
            On Game Started MQTT notification: Get Session Current Game

        Player A and Player B:
            Take Turn
            Take Turn
            Take Turn

        Either Player:
            End Game

        Either Player:
            Exit Gaming Session
*/

/*  Two Player session:

        Player A:
            Create a Gaming Session
            Subscribe to MQTT
            
        Player B:
            Join Gaming Session via invitation code
            Subscribe to MQTT
            Post Player Readiness

        Player A:
            On PlayerReady MQTT notification: Create Two Player Game
            
        Player B:
            On GameStarted MQTT notification: Call Get Session Current Game
     
        Player A and Player B:
            Take Turn
            Take Turn
            Take Turn
            
        Either Player:
            End Game
            
        Either Player:
            Exit Gaming Session
 */
