//
// NewTwoPlayerGameParams.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation
#if canImport(AnyCodable)
import AnyCodable
#endif

/** Models info needed to start a new Two-Player Game. */
public struct NewTwoPlayerGameParams: Codable, JSONEncodable, Hashable {

    public var sessionId: String

    public init(sessionId: String) {
        self.sessionId = sessionId
    }

    public enum CodingKeys: String, CodingKey, CaseIterable {
        case sessionId = "session_id"
    }

    // Encodable protocol methods

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(sessionId, forKey: .sessionId)
    }
}
