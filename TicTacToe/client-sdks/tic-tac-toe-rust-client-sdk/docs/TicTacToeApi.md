# \TicTacToeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_gaming_session**](TicTacToeApi.md#create_gaming_session) | **POST** /v1/gaming-sessions | Creates a new Gaming Session. Returns Gaming Session Creation Response.
[**create_single_player_game**](TicTacToeApi.md#create_single_player_game) | **POST** /v1/gaming-sessions/{session_id}/games | Creates a new Single-Player Game. Returns Game Creation Response.
[**create_two_player_game**](TicTacToeApi.md#create_two_player_game) | **POST** /v1/gaming-session/{session_id}/two-player-games | Creates a new Two-Player Game. Returns Game Creation Response.
[**end_game**](TicTacToeApi.md#end_game) | **DELETE** /v1/games/{game_id} | Closes down the specified Game.
[**end_gaming_session**](TicTacToeApi.md#end_gaming_session) | **DELETE** /v1/gaming-sessions/{session_id} | Closes down the specified Gaming Session.
[**get_game_history**](TicTacToeApi.md#get_game_history) | **GET** /v1/games/{game_id}/turns | Retrieves the history of Game States from the initial move (turn) to the current Game State. This can be used, for instance, to create an animated time-lapse of the Game play.
[**get_latest_game_turn**](TicTacToeApi.md#get_latest_game_turn) | **GET** /v1/games/{game_id}/turns/latest | Retrieves the most recent Turn for the specified Game.
[**get_players_readiness**](TicTacToeApi.md#get_players_readiness) | **GET** /v1/games/{game_id}/players/readiness | Retrieves the readiness of the Game's Players, answering the questions: Have all Players been added to the Game and setup?
[**get_session_current_game**](TicTacToeApi.md#get_session_current_game) | **GET** /v1/gaming-sessions/{session_id}/current-game | Retrieves the Gaming Session's current Game.
[**join_current_game**](TicTacToeApi.md#join_current_game) | **PUT** /v1/gaming-sessions/{session_id}/current_game/players/{player_id} | Adds a Player to the Session's Current Game.
[**join_gaming_session**](TicTacToeApi.md#join_gaming_session) | **POST** /v1/gaming-sessions/players | Adds a Player to the Gaming Session.
[**take_turn**](TicTacToeApi.md#take_turn) | **POST** /v1/games/{game_id}/turns | Make a Game move (turn) for the specified Player. Returns the Turn Response.



## create_gaming_session

> models::GamingSessionCreationResponse create_gaming_session(new_gaming_session_params)
Creates a new Gaming Session. Returns Gaming Session Creation Response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_gaming_session_params** | [**NewGamingSessionParams**](NewGamingSessionParams.md) |  | [required] |

### Return type

[**models::GamingSessionCreationResponse**](GamingSessionCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_single_player_game

> models::GameCreationResponse create_single_player_game(session_id, new_single_player_game_params)
Creates a new Single-Player Game. Returns Game Creation Response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |
**new_single_player_game_params** | [**NewSinglePlayerGameParams**](NewSinglePlayerGameParams.md) |  | [required] |

### Return type

[**models::GameCreationResponse**](GameCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_two_player_game

> models::GameCreationResponse create_two_player_game(session_id)
Creates a new Two-Player Game. Returns Game Creation Response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |

### Return type

[**models::GameCreationResponse**](GameCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## end_game

> end_game(game_id, end_game_params)
Closes down the specified Game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**end_game_params** | [**EndGameParams**](EndGameParams.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## end_gaming_session

> end_gaming_session(session_id, end_gaming_session_params)
Closes down the specified Gaming Session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |
**end_gaming_session_params** | [**EndGamingSessionParams**](EndGamingSessionParams.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_game_history

> Vec<models::GameState> get_game_history(game_id)
Retrieves the history of Game States from the initial move (turn) to the current Game State. This can be used, for instance, to create an animated time-lapse of the Game play.

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


## get_latest_game_turn

> models::TurnResponse get_latest_game_turn(game_id)
Retrieves the most recent Turn for the specified Game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | Game ID | [required] |

### Return type

[**models::TurnResponse**](TurnResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_players_readiness

> models::PlayersReadinessResponse get_players_readiness(game_id)
Retrieves the readiness of the Game's Players, answering the questions: Have all Players been added to the Game and setup?

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** | Game ID | [required] |

### Return type

[**models::PlayersReadinessResponse**](PlayersReadinessResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_session_current_game

> models::GameCreationResponse get_session_current_game(session_id)
Retrieves the Gaming Session's current Game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | Session ID | [required] |

### Return type

[**models::GameCreationResponse**](GameCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join_current_game

> models::GameCreationResponse join_current_game(session_id, player_id)
Adds a Player to the Session's Current Game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |
**player_id** | **String** |  | [required] |

### Return type

[**models::GameCreationResponse**](GameCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join_gaming_session

> models::GamingSessionCreationResponse join_gaming_session(join_session_params)
Adds a Player to the Gaming Session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**join_session_params** | [**JoinSessionParams**](JoinSessionParams.md) |  | [required] |

### Return type

[**models::GamingSessionCreationResponse**](GamingSessionCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## take_turn

> models::TurnResponse take_turn(game_id, game_turn_params)
Make a Game move (turn) for the specified Player. Returns the Turn Response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **String** |  | [required] |
**game_turn_params** | [**GameTurnParams**](GameTurnParams.md) |  | [required] |

### Return type

[**models::TurnResponse**](TurnResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

