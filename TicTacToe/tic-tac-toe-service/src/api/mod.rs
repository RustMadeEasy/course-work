pub(crate) mod games;
pub(crate) mod gaming_session;
pub(crate) mod docs;
pub(crate) mod health;

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
