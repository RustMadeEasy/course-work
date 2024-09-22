# MQTT Publisher Library

## Description

Provides MQTT message publishing functionality, including _simulcast_ to multiple brokers of differing versions.

Simulcast can be helpful because most brokers do not support messaging between clients using different versions of the protocol.  

Versions 3.1.1 and 5 of the MQTT protocol are supported.

## Usage Notes

This library does not support connections to a bare IP address with a self-signed certificate. One workaround, which only works under *nix/BSD-like systems, is to add an entry to wherever your DNS resolver looks (e.g. /etc/hosts) for the bare IP address and use that name in your code.