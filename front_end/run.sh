# /home/adrian/remote_motor_repo/remote_motor/front_end/setup_streaming.sh
mkdir -p /dev/shm/streaming
mkdir -p /dev/shm/streaming_front
mkdir -p /dev/shm/streaming_side
mkdir -p /dev/shm/streaming_top

if [ ! -e /var/www/html/streaming ]; then
        ln -s /dev/shm/streaming /var/www/html/streaming
fi

if [ ! -e /var/www/html/streaming_front ]; then
        ln -s /dev/shm/streaming_front /var/www/html/streaming_front
fi

if [ ! -e /var/www/html/streaming_side ]; then
        ln -s /dev/shm/streaming_side /var/www/html/streaming_side
fi

if [ ! -e /var/www/html/streaming_top ]; then
        ln -s /dev/shm/streaming_top /var/www/html/streaming_top
fi

if [ ! -e /var/www/html/index.html ]; then
        ln -s /home/adrian/remote_motor_repo/remote_motor/front_end/index.html /var/www/html/index.html
fi
if [ ! -e /var/www/html/streaming/streaming.html ]; then
        ln -s /home/adrian/remote_motor_repo/remote_motor/front_end/streaming.html /var/www/html/streaming/index.html
fi

#tmux new-session -d -s webapi 'exec cd /home/adrian/remote_motor_repo/remote_motor/webapi/'
#tmux send-keys 'exec cargo run'
tmux new-session -d -s frontcam 'exec ffmpeg -threads 2 -video_size 640x480 -i /dev/video2 -f video4linux2 -framerate 30 -c:v libx264 -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -min_seg_duration 250000 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 /dev/shm/streaming_front/manifest.mpd -c:v vp8 -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -min_seg_duration 250000 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 /dev/shm/streaming_front/manifest_mobile.mpd'
tmux new-session -d -s sidecam 'exec ffmpeg -threads 2 -i /dev/video0 -f video4linux2 -framerate 30 -c:v libx264 -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -min_seg_duration 250000 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 /dev/shm/streaming_side/manifest.mpd -c:v vp8 -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -min_seg_duration 250000 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 /dev/shm/streaming_side/manifest_mobile.mpd'
tmux new-session -d -s topcam 'exec ffmpeg -threads 2 -i /dev/video4 -f video4linux2 -framerate 30 -c:v libx264 -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -min_seg_duration 250000 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 /dev/shm/streaming_top/manifest.mpd -c:v vp8 -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -min_seg_duration 250000 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 /dev/shm/streaming_top/manifest_mobile.mpd'

cd /home/adrian/remote_motor_repo/remote_motor/webapi/
tmux new-session -d -s webapi 'exec cargo run'