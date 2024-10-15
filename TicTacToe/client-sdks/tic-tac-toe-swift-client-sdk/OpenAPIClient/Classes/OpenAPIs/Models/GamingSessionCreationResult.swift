//
// GamingSessionCreationResult.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation
#if canImport(AnyCodable)
import AnyCodable
#endif

/** Models the results of a call to the Create Gaming Session endpoint. */
public struct GamingSessionCreationResult: Codable, JSONEncodable, Hashable {

    public var eventPlaneConfig: EventPlaneConfig
    /** Unique Code that is used to invite others to the Gaming Session. */
    public var invitationCode: String
    /** ID of the Player added to the Gaming Session. */
    public var playerId: String
    /** Identifies the Gaming Session. This also serves as the communication channel for MQTT notifications. */
    public var sessionId: String

    public init(eventPlaneConfig: EventPlaneConfig, invitationCode: String, playerId: String, sessionId: String) {
        self.eventPlaneConfig = eventPlaneConfig
        self.invitationCode = invitationCode
        self.playerId = playerId
        self.sessionId = sessionId
    }

    public enum CodingKeys: String, CodingKey, CaseIterable {
        case eventPlaneConfig = "event_plane_config"
        case invitationCode = "invitation_code"
        case playerId = "player_id"
        case sessionId = "session_id"
    }

    // Encodable protocol methods

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(eventPlaneConfig, forKey: .eventPlaneConfig)
        try container.encode(invitationCode, forKey: .invitationCode)
        try container.encode(playerId, forKey: .playerId)
        try container.encode(sessionId, forKey: .sessionId)
    }
}

