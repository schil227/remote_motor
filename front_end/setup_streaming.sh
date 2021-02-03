# /var/lib/streaming/setup_streaming.sh
mkdir -p /dev/shm/streaming
if [ ! -e /var/www/html/streaming ]; then
        ln -s /dev/shm/treaming /var/www/html/streaming
fi
if [ ! -e /var/www/html/streaming/index.html ]; then
        ln -s /var/lib/streaming/index.html /var/www/html/index.html
fi
if [ ! -e /var/www/html/streaming/streaming.html ]; then
        ln -s /var/lib/streaming/streaming.html /var/www/html/streaming/index.html
fi
