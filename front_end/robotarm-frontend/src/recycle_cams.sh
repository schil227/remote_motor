tmux kill-session -t frontcam
tmux kill-session -t topcam
tmux kill-session -t sidecam

tmux new-session -d -s frontcam 'exec ffmpeg -rtsp_transport tcp -i rtsp://66.188.188.238:554/front -c:v libx264 -preset ultrafast -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 /usr/stmp/streaming_front/manifest.mpd'

tmux new-session -d -s topcam 'exec ffmpeg -rtsp_transport tcp -i rtsp://66.188.188.238:554/top -c:v libx264 -preset ultrafast -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 /usr/stmp/streaming_top/manifest.mpd'

#tmux new-session -d -s sidecam 'exec ffmpeg -rtsp_transport tcp -i rtsp://66.188.188.238:554/side -c:v libx264 -preset ultrafast -g 15 -keyint_min 120 -f dash -use_timeline 1 -use_template 1 -streaming 1 -seg_duration 1 -window_size 5 -remove_at_exit 1 -hls_playlist 1 /usr/stmp/streaming_side/manifest.mpd'
