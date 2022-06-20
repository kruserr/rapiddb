#!/bin/bash

container_name="rapiddb-devel"

podman stop $container_name
podman rm $container_name

podman build -f Dockerfile-devel -t $container_name .

podman run -dit --rm -p 6379:6379 -p 3306:3306 --name $container_name $container_name

sleep 5

podman exec $container_name mysql -e "UPDATE mysql.user SET host='%' WHERE user='root'"
podman exec $container_name mysql -e "UPDATE mysql.user SET plugin='mysql_native_password' WHERE user='root'"
podman exec $container_name mysql -e "FLUSH PRIVILEGES"
