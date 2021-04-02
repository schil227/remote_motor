sudo ng build --prod

sudo rm -fr /var/www/html/*

sudo cp -RT ./dist/robotarm-frontend/ /var/www/html/

mkdir -p /dev/shm/streaming
mkdir -p /dev/shm/streaming_front
mkdir -p /dev/shm/streaming_side
mkdir -p /dev/shm/streaming_top

if [ ! -e /var/www/html/streaming ]; then
        sudo ln -s /dev/shm/streaming /var/www/html/streaming
fi

if [ ! -e /var/www/html/streaming_front ]; then
        sudo ln -s /dev/shm/streaming_front /var/www/html/streaming_front
fi

if [ ! -e /var/www/html/streaming_side ]; then
        sudo ln -s /dev/shm/streaming_side /var/www/html/streaming_side
fi

if [ ! -e /var/www/html/streaming_top ]; then
        sudo ln -s /dev/shm/streaming_top /var/www/html/streaming_top
fi

if [ ! -e /var/www/html/streaming/streaming.html ]; then
        sudo ln -s /home/adrian/remote_motor_repo/remote_motor/front_end/streaming.html /var/www/html/streaming/streaming.html
fi

sudo mount --bind /dev/shm/streaming_front/ ~/stmp/streaming_front/
sudo mount --bind /dev/shm/streaming_top/ ~/stmp/streaming_top/
sudo mount --bind /dev/shm/streaming_side/ ~/stmp/streaming_side/

tmux new-session -d -s frontcam 'exec ffmpeg -threads 2 -video_size 640x480 -i /dev/video2 -f video4linux2 -framerate 30 -c:v libx264 -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -min_seg_duration 250000 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 ~/stmp/streaming_front/manifest.mpd -c:v vp8 -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -min_seg_duration 250000 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 ~/stmp/streaming_front/manifest_mobile.mpd'
tmux new-session -d -s topcam 'exec ffmpeg -threads 2 -i /dev/video4 -f raw_video -b:v 300k -framerate 30 -c:v libx264 -g 15 -keyint_min 120 -preset veryfast -f dash -use_timeline 1 -use_template 1 -streaming 1 -min_seg_duration 250000 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 ~/stmp/streaming_top/manifest.mpd -c:v vp8 -g 15 -keyint_min 120 -preset veryfast -f dash -use_timeline 1 -use_template 1 -streaming 1 -seg_duration 1 -window_size 5 -remove_at_exit 1 ~/stmp/streaming_top/manifest_mobile.mpd'

cd /home/adrian/remote_motor_repo/remote_motor/webapi/
tmux new-session -d -s webapi 'exec cargo run'