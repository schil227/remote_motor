# Remote Motor
## What it is
*Remote Motor* is a Rust application that enables communiation from a cilent to a server (hosted on a Raspberry Pi) which triggers actuators (motors, LEDs, etc.) remotely. Messages are sent in UDP payloads to trigger the desired output.

## Getting Started
There are two projects: the client and the server.

  * **The Client**: Can be run on any machine; it listens for key inputs from the console and sends them to the server.
  * **The Server**: Runs on a Raspberry Pi - listens for UDP messages, and when received it will perform the requested action.

### Setup:
1. Find out the IP addresses of the client and server
    *  This can be done in windows by windows by running `ipconfig` in the terminal, and for linux `ifconfig`. If setting it up on the local network, you'll be looking for something like 192.168.1.XXX
1. Open the server's port if necessairy
    * Run the following command on the Pi:
    `sudo iptables -A INPUT -p udp --dport 7870 -j ACCEPT`
        * `-A` means append to the ip tables configuration
        * `-p` is the protocol (udp)
        * `--deport` is the port to listen on (7870 in this case)
        * `-j` is the default behavior
1. Update the client project to bind to their ip address to their client: 
```rust
 let client = UdpSocket::bind("192.168.1.186:7870").expect("Failed to bind client UDP socket.");
 ```
4. Update the client's target to the server's IP address:
```rust
client.send_to(&buf, "192.168.1.226:7870").expect("Failed to send message!");
 ```
5. Update the server's connection address to the server's IP:
```rust
 let client = UdpSocket::bind("192.168.1.226:7870").expect("Failed to bind client UDP socket.");
 ```
 6. Run the server, verify it started successfully by switching to a different terminal instance (`Alt+Ctrl+F2`, or any function key) and running `netstat -lntu`. You should see the `192.168.1.226` ip address listed on the opened port (`7870`)
 1. Run the client - verify the server is getting the messages by pressing a few action keys.