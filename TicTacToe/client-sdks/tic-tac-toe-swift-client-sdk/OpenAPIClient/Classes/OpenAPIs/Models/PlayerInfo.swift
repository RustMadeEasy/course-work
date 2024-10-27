//
// PlayerInfo.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation
#if canImport(AnyCodable)
import AnyCodable
#endif

/** Models a Tic-Tac-Toe Game Player. */
public struct PlayerInfo: Codable, JSONEncodable, Hashable {

    /** Name of the Player. */
    public var displayName: String
    /** The Game Piece assigned to the Player. */
    public var gamePiece: GamePiece
    /** Indicates that this Player's moves are automated, i.e., guided by this service. */
    public var isAutomated: Bool
    /** Unique ID of the Player. */
    public var playerId: String

    public init(displayName: String, gamePiece: GamePiece, isAutomated: Bool, playerId: String) {
        self.displayName = displayName
        self.gamePiece = gamePiece
        self.isAutomated = isAutomated
        self.playerId = playerId
    }

    public enum CodingKeys: String, CodingKey, CaseIterable {
        case displayName = "display_name"
        case gamePiece = "game_piece"
        case isAutomated = "is_automated"
        case playerId = "player_id"
    }

    // Encodable protocol methods

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(displayName, forKey: .displayName)
        try container.encode(gamePiece, forKey: .gamePiece)
        try container.encode(isAutomated, forKey: .isAutomated)
        try container.encode(playerId, forKey: .playerId)
    }
}

