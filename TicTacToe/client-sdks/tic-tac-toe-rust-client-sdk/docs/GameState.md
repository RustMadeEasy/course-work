# GameState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**game_board** | [**Vec<Vec<models::GamePiece>>**](Vec.md) | The board on which the Game is played. | 
**id_of_player_who_made_move** | **String** | ID of the Player who made this Move. | 
**play_status** | [**models::PlayStatus**](PlayStatus.md) |  | 
**winning_locations** | Option<[**Vec<models::BoardPosition>**](BoardPosition.md)> | If the Game has ended in a win, this contains the winning board positions. | [optional]
**winning_player_id** | Option<**String**> | If the Game has ended in a win, this indicates the ID of the winning Player. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


