//
// GameCreationResult.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation
#if canImport(AnyCodable)
import AnyCodable
#endif

/** Models the results of a call to the Create Game and Add Player endpoints. */
public struct GameCreationResult: Codable, JSONEncodable, Hashable {

    public var eventPlaneConfig: EventPlaneConfig
    public var gameInfo: GameInfo

    public init(eventPlaneConfig: EventPlaneConfig, gameInfo: GameInfo) {
        self.eventPlaneConfig = eventPlaneConfig
        self.gameInfo = gameInfo
    }

    public enum CodingKeys: String, CodingKey, CaseIterable {
        case eventPlaneConfig = "event_plane_config"
        case gameInfo = "game_info"
    }

    // Encodable protocol methods

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(eventPlaneConfig, forKey: .eventPlaneConfig)
        try container.encode(gameInfo, forKey: .gameInfo)
    }
}

