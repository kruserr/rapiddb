#!/bin/bash

rm_container_name="rapiddb-bench-rm"
mmav_container_name="rapiddb-bench-mmav"

podman build -f Dockerfile-rm -t $rm_container_name .
podman build -f Dockerfile-mmav -t $mmav_container_name .
