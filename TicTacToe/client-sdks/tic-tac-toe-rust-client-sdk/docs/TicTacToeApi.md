# \TicTacToeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_player**](TicTacToeApi.md#add_player) | **POST** /v1/games/players | * Defines and implements the public Gaming contract for this service.  *  * © 2024 Rust Made Easy. All rights reserved.  * @author Info@RustMadeEasy.com
[**create_game**](TicTacToeApi.md#create_game) | **POST** /v1/games | Creates a new Game. Returns the Game Info.
[**end_game**](TicTacToeApi.md#end_game) | **DELETE** /v1/games/{game_id} | Closes down the specified Game.
[**get_game_history**](TicTacToeApi.md#get_game_history) | **GET** /v1/games/{game_id}/turns | Retrieves the history of the Game States from the initial creation to the current
[**get_game_info**](TicTacToeApi.md#get_game_info) | **GET** /v1/games/{game_id} | Retrieves the specified Game info.
[**take_turn**](TicTacToeApi.md#take_turn) | **POST** /v1/games/{game_id}/turns | Make a game move for the specified Player.



## add_player

> models::GameInfo add_player(add_player_params)
* Defines and implements the public Gaming contract for this service.  *  * © 2024 Rust Made Easy. All rights reserved.  * @author Info@RustMadeEasy.com

* Defines and implements the public Gaming contract for this service.  *  * © 2024 Rust Made Easy. All rights reserved.  * @author Info@RustMadeEasy.com Adds a Player to the Game. Returns the Game Info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_player_params** | [**AddPlayerParams**](AddPlayerParams.md) |  | [required] |

### Return type

[**models::GameInfo**](GameInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_game

> models::GameInfo create_game(new_game_params)
Creates a new Game. Returns the Game Info.

Creates a new Game. Returns the Game Info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_game_params** | [**NewGameParams**](NewGameParams.md) |  | [required] |

### Return type

[**models::GameInfo**](GameInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## end_game

> end_game(game_id)
Closes down the specified Game.

Closes down the specified Game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_game_history

> Vec<models::GameState> get_game_history(game_id)
Retrieves the history of the Game States from the initial creation to the current

Retrieves the history of the Game States from the initial creation to the current Game State. This can be used, for instance, for the client to provide an animation that shows a time-lapse of the game play.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |

### Return type

[**Vec<models::GameState>**](GameState.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_game_info

> models::GameInfo get_game_info(game_id)
Retrieves the specified Game info.

Retrieves the specified Game info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | Game ID | [required] |

### Return type

[**models::GameInfo**](GameInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## take_turn

> take_turn(game_id, game_turn_info)
Make a game move for the specified Player.

Make a game move for the specified Player.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**game_turn_info** | [**GameTurnInfo**](GameTurnInfo.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

