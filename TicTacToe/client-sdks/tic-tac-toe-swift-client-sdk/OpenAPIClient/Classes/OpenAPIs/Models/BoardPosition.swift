//
// BoardPosition.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation
#if canImport(AnyCodable)
import AnyCodable
#endif

/** Models a position on the Game board. */
public struct BoardPosition: Codable, JSONEncodable, Hashable {

    static let columnRule = NumericRule<Int>(minimum: 0, exclusiveMinimum: false, maximum: nil, exclusiveMaximum: false, multipleOf: nil)
    static let rowRule = NumericRule<Int>(minimum: 0, exclusiveMinimum: false, maximum: nil, exclusiveMaximum: false, multipleOf: nil)
    /** The position's column */
    public var column: Int
    /** The position's row */
    public var row: Int

    public init(column: Int, row: Int) {
        self.column = column
        self.row = row
    }

    public enum CodingKeys: String, CodingKey, CaseIterable {
        case column
        case row
    }

    // Encodable protocol methods

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(column, forKey: .column)
        try container.encode(row, forKey: .row)
    }
}

