# GamingSessionCreationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_plane_config** | [**models::EventPlaneConfig**](EventPlaneConfig.md) | Specifies the configuration required for clients to subscribe to real-time Game state updates | 
**initiating_player** | [**models::PlayerInfo**](PlayerInfo.md) | The Player who initiated the Gaming Session | 
**invitation_code** | **String** | Unique Code that is used to invite other participants to the Gaming Session | 
**other_player** | Option<[**models::PlayerInfo**](PlayerInfo.md)> | ID of the additional Player | [optional]
**session_id** | **String** | Identifies the Gaming Session. This also serves as the communication channel for MQTT notifications. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


