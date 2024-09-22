//
//  GameInfoReceiver.swift
//
// © 2024 Rust Made Easy. All rights reserved.
// @author Info@RustMadeEasy.com
//

import Foundation
import MQTTNIO
import OpenAPIClient

/// Defines handler for game info events.
protocol GameInfoReceiverDelegate {
    func onGameEndedInStalemate()
    func onGameEndedInWin()
    func onPlayerAdded()
    func onTurnTaken()
}

/// Listens for game state changes published by our Tic Tac Toe service.
class GameInfoReceiver {
    
    private var client: MQTTClient!
    private var delegate: GameInfoReceiverDelegate!
    private var eventPlaneConfig: EventPlaneConfig
    
    private var topicGameEndedInStalemate: String!
    private var topicGameEndedInWin: String!
    private var topicPlayerAdded: String!
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
    private func buildTopic(topic: EventTopicNames) -> String {
        String(format: "%@/%@", self.eventPlaneConfig.topicPrefix, topic.rawValue)
    }
    
    /// Pre-builds the topics so that we are not parsing each time a message is received.
    private func prebuildTopics() {
        self.topicGameEndedInStalemate = buildTopic(topic: EventTopicNames.gameEndedInStalemate)
        self.topicGameEndedInWin = buildTopic(topic: EventTopicNames.gameEndedInWin)
        self.topicPlayerAdded = buildTopic(topic: EventTopicNames.playerAdded)
        self.topicTurnTaken = buildTopic(topic: EventTopicNames.turnTaken)
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
            case self.topicGameEndedInStalemate:
                self.delegate.onGameEndedInStalemate()
            case self.topicGameEndedInWin:
                self.delegate.onGameEndedInWin()
            case self.topicPlayerAdded:
                self.delegate.onPlayerAdded()
            case self.topicTurnTaken:
                self.delegate.onTurnTaken()
            default:
                print("Warning: GameInfoReceiver - Received unsupported message")
            }
        }
        
        return client
    }
}
