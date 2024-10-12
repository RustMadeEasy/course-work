# \TicTacToeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_gaming_session**](TicTacToeApi.md#create_gaming_session) | **POST** /v1/gaming-sessions | Creates a new Gaming Session. Returns GamingSessionCreationResult.
[**create_single_player_game**](TicTacToeApi.md#create_single_player_game) | **POST** /v1/single-player-games | Creates a new Game. Returns Game Creation Result.
[**create_two_player_game**](TicTacToeApi.md#create_two_player_game) | **POST** /v1/two-player-games | Creates a new Two-Player Game. Returns Game Creation Result.
[**end_game**](TicTacToeApi.md#end_game) | **DELETE** /v1/games/{game_id} | Closes down the specified Game.
[**end_gaming_session**](TicTacToeApi.md#end_gaming_session) | **DELETE** /v1/gaming-sessions/{session_id} | Closes down the specified Gaming Session.
[**get_game_history**](TicTacToeApi.md#get_game_history) | **GET** /v1/games/{game_id}/turns | Retrieves the history of the Game States from the initial move (turn) to the latest
[**get_game_info**](TicTacToeApi.md#get_game_info) | **GET** /v1/games/{game_id} | Retrieves details of the specified Game.
[**join_gaming_session**](TicTacToeApi.md#join_gaming_session) | **POST** /v1/gaming-sessions/players | Adds a Player to the Gaming Session.
[**take_turn**](TicTacToeApi.md#take_turn) | **POST** /v1/games/{game_id}/turns | Make a Game move (turn) for the specified Player.



## create_gaming_session

> models::GamingSessionCreationResult create_gaming_session(new_gaming_session_params)
Creates a new Gaming Session. Returns GamingSessionCreationResult.

Creates a new Gaming Session. Returns GamingSessionCreationResult.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_gaming_session_params** | [**NewGamingSessionParams**](NewGamingSessionParams.md) |  | [required] |

### Return type

[**models::GamingSessionCreationResult**](GamingSessionCreationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_single_player_game

> models::GameCreationResult create_single_player_game(new_single_player_game_params)
Creates a new Game. Returns Game Creation Result.

Creates a new Game. Returns Game Creation Result.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_single_player_game_params** | [**NewSinglePlayerGameParams**](NewSinglePlayerGameParams.md) |  | [required] |

### Return type

[**models::GameCreationResult**](GameCreationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_two_player_game

> models::GameCreationResult create_two_player_game(new_two_player_game_params)
Creates a new Two-Player Game. Returns Game Creation Result.

Creates a new Two-Player Game. Returns Game Creation Result.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_two_player_game_params** | [**NewTwoPlayerGameParams**](NewTwoPlayerGameParams.md) |  | [required] |

### Return type

[**models::GameCreationResult**](GameCreationResult.md)

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


## end_gaming_session

> end_gaming_session(session_id)
Closes down the specified Gaming Session.

Closes down the specified Gaming Session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |

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
Retrieves the history of the Game States from the initial move (turn) to the latest

Retrieves the history of the Game States from the initial move (turn) to the latest Game State. This can be used, for instance, to create an animated time-lapse of the Game play.

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
Retrieves details of the specified Game.

Retrieves details of the specified Game.

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


## join_gaming_session

> models::GamingSessionCreationResult join_gaming_session(join_session_params)
Adds a Player to the Gaming Session.

Adds a Player to the Gaming Session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**join_session_params** | [**JoinSessionParams**](JoinSessionParams.md) |  | [required] |

### Return type

[**models::GamingSessionCreationResult**](GamingSessionCreationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## take_turn

> take_turn(game_id, game_turn_info)
Make a Game move (turn) for the specified Player.

Make a Game move (turn) for the specified Player.

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

