# TurnResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_player** | Option<[**models::PlayerInfo**](PlayerInfo.md)> | Player who will take the next turn | [optional]
**new_game_state** | [**models::GameState**](GameState.md) | The state of the Game after the turn has been taken | 
**winning_locations** | Option<[**Vec<models::BoardPosition>**](BoardPosition.md)> | If the Game has ended in a win, this contains the winning board positions | [optional]
**winning_player** | Option<[**models::PlayerInfo**](PlayerInfo.md)> | If the Game has ended in a win, this indicates the winning Player | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


