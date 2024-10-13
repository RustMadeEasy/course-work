//
//  GameInfoReceiver.swift
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com
//

import Foundation
import MQTTNIO
import OpenAPIClient

/// Defines handler for game info events.
protocol GameInfoReceiverDelegate {    
    func onGameDeleted()
    func onGameEndedInStalemate()
    func onGameEndedInWin()
    func onGameStarted()
    func onPlayerAddedToSession()
    func onSessionDeleted()
    func onTurnTaken()
}

/// Listens for game state changes published by our Tic Tac Toe service.
class GameInfoReceiver {
    
    private var client: MQTTClient!
    private var delegate: GameInfoReceiverDelegate!
    private var eventPlaneConfig: EventPlaneConfig
    
    private var topicGameDeleted: String!
    private var topicGameEndedInStalemate: String!
    private var topicGameEndedInWin: String!
    private var topicGameStarted: String!
    private var topicPlayerAddedToSession: String!
    private var topicSessionDeleted: String!
    private var topicTurnTaken: String!
    
    init(eventPlaneConfig: EventPlaneConfig, delegate: GameInfoReceiverDelegate) {
        self.delegate = delegate
        self.eventPlaneConfig = eventPlaneConfig
        prebuildTopics()
        self.client = setupMqttClient(eventPlaneConfig: eventPlaneConfig)
    }
    
}

extension GameInfoReceiver {
    
    /// Builds a full topic string based on the specified topic name.
    private func buildTopic(topic: EventPlaneTopicNames) -> String {
        String(format: "%@/%@", self.eventPlaneConfig.topicPrefix, topic.rawValue)
    }
    
    /// Pre-builds the topics so that we are not parsing each time a message is received.
    private func prebuildTopics() {
        self.topicGameDeleted = buildTopic(topic: EventPlaneTopicNames.gameDeleted)
        self.topicGameEndedInStalemate = buildTopic(topic: EventPlaneTopicNames.gameEndedInStalemate)
        self.topicGameEndedInWin = buildTopic(topic: EventPlaneTopicNames.gameEndedInWin)
        self.topicGameStarted = buildTopic(topic: EventPlaneTopicNames.gameStarted)
        self.topicPlayerAddedToSession = buildTopic(topic: EventPlaneTopicNames.playerAddedToSession)
        self.topicSessionDeleted = buildTopic(topic: EventPlaneTopicNames.sessionDeleted)
        self.topicTurnTaken = buildTopic(topic: EventPlaneTopicNames.turnTaken)
    }

    /// Initializes the MQTT client and sets up the callbacks used to inform the delegate.
    private func setupMqttClient(eventPlaneConfig: EventPlaneConfig) -> MQTTClient {
                
        let client = MQTTClient(
            configuration: .init(
                target: .host(eventPlaneConfig.brokerAddress, port: eventPlaneConfig.brokerPort)
            ),
            eventLoopGroupProvider: .createNew
        )
        
        client.connect()
        
        let allMessagesForSession = String(format: "%@/#", eventPlaneConfig.topicPrefix)
        client.subscribe(to: allMessagesForSession)

        client.whenConnected { response in
            print("MQTT connected. Is session present: \(response.isSessionPresent)")
        }

        client.whenDisconnected { reason in
            print("MQTT disconnected: \(reason)")
        }

        client.whenMessage { message in
            // Inform the delegate of the event
            switch message.topic {
            case self.topicGameDeleted:
                self.delegate.onGameDeleted()
            case self.topicGameEndedInStalemate:
                self.delegate.onGameEndedInStalemate()
            case self.topicGameEndedInWin:
                self.delegate.onGameEndedInWin()
            case self.topicGameStarted:
                self.delegate.onGameStarted()
            case self.topicPlayerAddedToSession:
                self.delegate.onPlayerAddedToSession()
            case self.topicSessionDeleted:
                self.delegate.onSessionDeleted()
            case self.topicTurnTaken:
                self.delegate.onTurnTaken()
            default:
                print("Warning: GameInfoReceiver - Received unsupported message")
            }
        }
        
        return client
    }
}
