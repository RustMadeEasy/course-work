//
// NewSinglePlayerGameParams.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation
#if canImport(AnyCodable)
import AnyCodable
#endif

/** Models info needed to start a new Single-Player Game. */
public struct NewSinglePlayerGameParams: Codable, JSONEncodable, Hashable {

    public var computerSkillLevel: AutomaticPlayerSkillLevel
    public var sessionId: String

    public init(computerSkillLevel: AutomaticPlayerSkillLevel, sessionId: String) {
        self.computerSkillLevel = computerSkillLevel
        self.sessionId = sessionId
    }

    public enum CodingKeys: String, CodingKey, CaseIterable {
        case computerSkillLevel = "computer_skill_level"
        case sessionId = "session_id"
    }

    // Encodable protocol methods

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(computerSkillLevel, forKey: .computerSkillLevel)
        try container.encode(sessionId, forKey: .sessionId)
    }
}

