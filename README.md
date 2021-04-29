# robotarm.io
This repo contains all the code necessairy to run the entirity of the robotarm.io website, from the Angular front end application to the WebApi backend, and the physical robot arm interfacing software the runs on the Raspberry PI.

robotarm.io is a website which allows many people to control a robot arm using a simple interface. Users view the robot arm real-time via streaming video, adjust sliders to set the new desired state of the arm, and the server takes an average of all the input to produce the output. The users will have a common goal in mind in order to focus a desired outcome. Currently (4/28/21) the robot arm is set up in a "basket-ball" type arena, with a small ball, a hoop, and a scoreboard that updates when a shot is made. In the future, more arms and arenas will be built, including a 'vs' mode between two competing robot arms.

## Projects
### client  
This project is more or less defunct, outside of motor calibration. Originally, it was used to give live, unbounded motor control (e.g. move forarm forward and dont stop) using dedicated keybindings for each of the motors. It uses a tcp connection to talk directly to the *server* on the Pi.
### front_end
This is an angular project that interfaces with the user input, and communicates with a mixture of HTTP/REST and Websocket to the *WebApi* project. Users are able to view the robot via video streaming (using DASH format), adjust sliders that have a visual indicator of what the change will do, and are able to get real-time notifications from the server (e.g. a warning when the robot's about to move).
### models
Common models used by codebase, largly consumed by *server* and *client* projects, and the mostly empty *services* project.
### server
The code that runs on the Pi. Listens for TCP connections (only available locally, due to internet firewall).
### services
Largely defunct project, was used for creating common services that would be used by other rust apps (e.g. a config-reader) however work was abandoned in favor of getting other work done.
### webapi
The WebApi is a web-interfacing API that is consumed by the *front_end* UI, and interfaces with the backend *server* code. It uses Rocket (a rust based web framework) to for HTTP requests and responsces, and ws for handling websocket connections. The HTTP/REST and Websocket stacks are largly separated at the time of writing (4/28/21). The WebApi also keeps track of active users using an in-memory struct (in lieu of a database), which is routinely pruned based on the heartbeat from the frontend.



# old doc (TODO: Scrub)

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
    * Run the following command on the Pi ([see here](https://www.journaldev.com/34113/opening-a-port-on-linux)):
    `sudo iptables -A INPUT -p tcp --dport 7870 -j ACCEPT`
        * `-A` means append to the ip tables configuration
        * `-p` is the protocol (tcp)
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

 ## Hardware
 ### Servo Motors
 In order to figure out how to run servos, you need to know the frequency (hertz), and the duty cycles (how long the signal is 'high' for a period) to run the servos correctly.

 For example, the Tower Pro Micro Servo 9g runs at 50hz, has a min cycle at 2% and a max cycle at 12%. This means to turn the motor's rotor all the way down, the PWM would have the duty cycle be 2% - all the way up at 12%, and 7% to move it to the middle. 

 This is achieved in code using OutputPin's `set_pwm_frequency`. For example this code sets the rotor to the "middle" position:
 
 ```rust
    motor_pin.set_pwm_frequency(
        50.0,
        0.07
    ).unwrap();
 ```