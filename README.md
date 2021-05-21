# robotarm.io
This repo contains all the code necessairy to run the entirity of the robotarm.io website, from the Angular front end application to the WebApi backend, and the physical robot arm interfacing software the runs on the Raspberry PI.

robotarm.io is a website which allows many people to control a robot arm using a simple interface. Users view the robot arm real-time via streaming video, adjust sliders to set the new desired state of the arm, and the server takes an average of all the input to produce the output. The users will have a common goal in mind in order to focus a desired outcome. Currently (4/28/21) the robot arm is set up in a "basket-ball" type arena, with a small ball, a hoop, and a scoreboard that updates when a shot is made. In the future, more arms and arenas will be built, including a 'vs' mode between two competing robot arms.

# Projects
## client  
This project is more or less defunct, outside of motor calibration. Originally, it was used to give live, unbounded motor control (e.g. move forarm forward and dont stop) using dedicated keybindings for each of the motors. It uses a tcp connection to talk directly to the *server* on the Pi.
## front_end
This is an angular project that interfaces with the user input, and communicates with a mixture of HTTP/REST and Websocket to the *WebApi* project. Users are able to view the robot via video streaming (using DASH format), adjust sliders that have a visual indicator of what the change will do, and are able to get real-time notifications from the server (e.g. a warning when the robot's about to move).
## models
Common models used by codebase, largly consumed by *server* and *client* projects, and the mostly empty *services* project.
### server
The code that runs on the Pi. Listens for TCP connections (only available locally, due to internet firewall).
## services
Largely defunct project, was used for creating common services that would be used by other rust apps (e.g. a config-reader) however work was abandoned in favor of getting other work done.
## webapi
The WebApi is a web-interfacing API that is consumed by the *front_end* UI, and interfaces with the backend *server* code. It uses Rocket (a rust based web framework) to for HTTP requests and responsces, and ws for handling websocket connections. The HTTP/REST and Websocket stacks are largly separated at the time of writing (4/28/21). The WebApi also keeps track of active users using an in-memory struct (in lieu of a database), which is routinely pruned based on the heartbeat from the frontend.

# Setup

## webapi
The WebApi is run on a server that handles communication with the Pi, all the incoming HTTP requests, and the video feed from the webcams. (Currently) in front_end/robotarm-frontend, there is a deploy.sh script that handles all the work of setting up the server and the front end (run.sh is similar to deploy.sh, except it does not build/deploy the frontend code). It sets up the video capture (using `ffmpeg` commands) and writes the video data to the shared memory directory `/dev/shm` (which is actually accessed from the home directory, due to permissions)

### Video Streaming
For each camera, two streams are created with different encoding (one for mobile/firefox, one for everything else). Here's an example of an ffmpeg command:

`ffmpeg -threads 2 -i /dev/video4 -f raw_video -b:v 300k -framerate 30 -c:v libx264 -g 15 -keyint_min 120 -preset veryfast -f dash -use_timeline 1 -use_template 1 -streaming 1 -min_seg_duration 250000 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 ~/stmp/streaming_top/manifest.mpd -c:v vp8 -g 15 -keyint_min 120 -preset veryfast -f dash -use_timeline 1 -use_template 1 -streaming 1 -seg_duration 1 -window_size 5 -remove_at_exit 1 ~/stmp/streaming_top/manifest_mobile.mpd`

breaking that down:

`ffmpeg -threads 2 -i /dev/video4 -f raw_video -b:v 300k -framerate 30`
This is the video source (in this case, /dev/video4 is a usb webcam). It's formatted to return raw_video with a video bitrate of 300k bits, 30 fps.

`-c:v libx264 -g 15 -keyint_min 120 -preset veryfast`
This is the codec configuration; the video stream will be converted into libx246

`f dash -use_timeline 1 -use_template 1 -streaming 1 -min_seg_duration 250000 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 ~/stmp/streaming_top/manifest.mpd`
This is how the video data will be "chopped up" - in this case, it will be made into a `dash` format type, creating the manifest file. Most of the flags on here are to ensure that the video size is as small as possible (1 second), and that the retained video files dont exceed a certain amount (`window_size`).

### Connection
*  Open ports 8000 (HTTP) and 8001 (WebSocket) using the `iptables` command below
*  Install nginx to host the frontend.
*  Verify that the Domain Hosting Service points to the public IP address of the router.
*  Portforward 80 (frontend), 8000 (WebApi HTTP), and 8001 (WebApi Websocket) on the router to the server's local IP (same ports).

## front_end
The Frontend project (robotarm_frontend) is setup using the same deploy.sh script mentioned in the WebApi section. Specifically, the `sudo ng build --prod` line builds the frontend project (using the prod environment.ts script, thus configuring the target IP addresses), and further along the output of that build is copied to the /var/www/html directory. 

### Development Caveats
While there isn't much special configuration for this project, developing locally can be a bit of a hassle in terms of connecting to other resources.
*  The site cannot be accessed by typing robotarm.io in the browser when on the same network, as this will resolve to the router.
*  Connecting to the WebApi requires updating the environment.ts config to point to the local instance (e.g. 192.168.1.248)
*  When developing locally using localhost:4200, secure cookies are not stored in the browser, so there is never an associated user-id that sticks to the session. This can throw some things off (e.g. live usercount, command data can get thrown out, etc.) 
*  The video stream  is unavailable for localhost because the data is located in the /var/www/html... directory. The location is not accessable to the angular build location (under ./front_end/robotarm-frontend/...)

As for actually starting the localhost dev instance, navigate to /front_end/robotarm-frontend/ and run `sudo ng serve`

## server
### Connection
In order to connect to the raspberry pi, scan for the pi to determine the IP address (e.g. Angry IP Scanner) or manually hook up a mouse/keyboard/monitor and check with `ifconfig`. The pi supports 2G internet wifi connections (not 5G).

In order to communicate with the PI, a TCP Port must be opened (7870). Open the connecting port using the following command to open the TCP port.([see here](https://www.journaldev.com/34113/opening-a-port-on-linux)):
    `sudo iptables -A INPUT -p tcp --dport 7870 -j ACCEPT`
        * `-A` means append to the ip tables configuration
        * `-p` is the protocol (tcp)
        * `--deport` is the port to listen on (7870 in this case)
        * `-j` is the default behavior

If necessariy, update the local IP and port in the code.

### GPIO Wireup
TODO

### Servo Motors
The arm is composed of servo motors, which require power, ground, and a signal line which is driven using pulse width modulation (PWM). In order to figure out how to run servos, you need to know the frequency (hertz), and the duty cycles (how long the signal is 'high' for a period) to run the servos correctly.

 For example, the Tower Pro Micro Servo 9g runs at 50hz, has a min cycle at 2% and a max cycle at 12%. This means to turn the motor's rotor all the way down, the PWM would have the duty cycle be 2% - all the way up at 12%, and 7% to move it to the middle. Determining the exact max/min values will require some manual calibration, typically following these steps:
  *  Configure the server (pi) and client projects with the correct IPs
  *  Ensure that the motor is hooked up to an appropreate position on the arm (e.g. the motor is firmly fascened, the armature is fascened to the motor, and the current position wont "snap" when suddenly connected to the shoulder's duty cycle (see next))
  *  Connect the motor to be calubrated to the 'shoulder' motor's pin on the Pca9685. This is because the shoulder has the largest min/max with ~180 degrees of motion
  *  Control the motor with the keys, stopping when the min/max points have been reached. Look at the output of the server (pi) and record the duty values that are logged to the console.
  *  Update the appropreate `MotorData` constant in the Models project.

  # Development Notes
  ## Server Hosting
  According to Josh, since I'm pointing the url directly to my router, I leave myself open to DDoS attacks. Using a hosting service (e.g. AWS, Digital Ocean, Azure, etc.) would allow for the app to be hosted elsewhere on better hardware (and better internet bandwith), thus making a reliable connection. 
  
  In order to get it to work, the *FrontEnd* and *WebApi* would have to be hosted, and my router would have to grant the servers access to the Pi (control the arm) and my server (to get the webcam streams). FFMPEG may be able to beam the stream directly to an IP address (and, I assume, keep everything in memory)