# /home/adrian/remote_motor_repo/remote_motor/front_end/setup_streaming.sh
mkdir -p /dev/shm/streaming
if [ ! -e /var/www/html/streaming ]; then
        ln -s /dev/shm/streaming /var/www/html/streaming
fi
if [ ! -e /var/www/html/streaming/index.html ]; then
        ln -s /home/adrian/remote_motor_repo/remote_motor/front_end/index.html /var/www/html/index.html
fi
if [ ! -e /var/www/html/streaming/streaming.html ]; then
        ln -s /home/adrian/remote_motor_repo/remote_motor/front_end/streaming.html /var/www/html/streaming/index.html
fi
