echo "Starting Server Setup."
sudo rm -fr /var/www/html/*

sudo cp -RT ./dist/robotarm-frontend/ /var/www/html/

echo "Loaded Website"

echo "Making Shared memory directories"
mkdir -p /dev/shm/streaming
mkdir -p /dev/shm/streaming_front
mkdir -p /dev/shm/streaming_side
mkdir -p /dev/shm/streaming_top

echo "checking for /var/www/html/streaming"
if [ ! -e /var/www/html/streaming ]; then
	echo "DNE: linking /var/www/html/streaming"
	sudo ln -s /dev/shm/streaming /var/www/html/streaming
fi

echo "checking for streaming_front"
if [ ! -e /var/www/html/streaming_front ]; then
	echo "DNE: linking /var/www/html/streaming_front"
        sudo ln -s /dev/shm/streaming_front /var/www/html/streaming_front
fi

echo "checking for streaming_side"
if [ ! -e /var/www/html/streaming_side ]; then
	echo "DNE: linking /var/www/html/streaming_side"
        sudo ln -s /dev/shm/streaming_side /var/www/html/streaming_side
fi

echo "checking for streaming_top"
if [ ! -e /var/www/html/streaming_top ]; then
	echo "DNE: linking /var/www/html/streaming_top"
        sudo ln -s /dev/shm/streaming_top /var/www/html/streaming_top
fi

echo "checking for streaming.html"
if [ ! -e /var/www/html/streaming/streaming.html ]; then
	echo "DNE: linking streaming.html"
        sudo ln -s /usr/remote_motor/front_end/streaming.html /var/www/html/streaming/streaming.html
fi

echo "Binding /usr/stmp to /dev/shm"
sudo mount --bind /dev/shm/streaming_front/ /usr/stmp/streaming_front/
sudo mount --bind /dev/shm/streaming_top/ /usr/stmp/streaming_top/
sudo mount --bind /dev/shm/streaming_side/ /usr/stmp/streaming_side/

echo "Tmuxing front cam"

tmux new-session -d -s frontcam 'exec ffmpeg -rtsp_transport tcp -i rtsp://66.188.188.238:554/front -c:v vp8 -preset ultrafast -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -seg_duration 1 -window_size 5 -remove_at_exit 1 -dash_segment_type webm -hls_playlist 1 /usr/stmp/streaming_front/manifest.mpd'

tmux new-session -d -s topcam 'exec ffmpeg -rtsp_transport tcp -i rtsp://66.188.188.238:554/top -c:v vp8 -preset ultrafast -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -seg_duration 1 -window_size 5 -remove_at_exit 1 -dash_segment_type webm -hls_playlist 1 /usr/stmp/streaming_top/manifest.mpd'

tmux new-session -d -s sidecam 'exec ffmpeg -rtsp_transport tcp -i rtsp://66.188.188.238:554/side -c:v vp8 -preset ultrafast -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -seg_duration 1 -window_size 5 -remove_at_exit 1 -dash_segment_type webm -hls_playlist 1 /usr/stmp/streaming_side/manifest.mpd'


echo "Cd and tmuxing webapi"
cd /usr/remote_motor/webapi/
tmux new-session -d -s webapi 'exec ./run.sh'
