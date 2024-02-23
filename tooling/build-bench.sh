#!/bin/bash

mmav_container_name="rapiddb-bench-mmav"

podman build -f Dockerfile-mmav -t $mmav_container_name .
