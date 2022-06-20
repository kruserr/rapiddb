#!/bin/bash

rm_container_name="rapiddb-bench-rm"
mmav_container_name="rapiddb-bench-mmav"

fn_test() {
    mkdir bench > /dev/null 2>&1
    mkdir bench/$1 > /dev/null 2>&1

    podman run -dit --rm --name $1 $1
    sleep 3

    printf "$(date --iso='seconds')\n" >> bench/$1/$2-$3.log
    printf "$1-$2-$3\n\n" >> bench/$1/$2-$3.log

    printf "STATS\n" >> bench/$1/$2-$3.log
    podman stats --no-stream >> bench/$1/$2-$3.log
    podman exec $1 sh -c "ps aux" >> bench/$1/$2-$3.log

    post=$(($5 * $3 / 100))
    get=$(($5 - $5 * $3 / 100))

    podman exec $1 sh -c "wrk -s wrk.lua -t1 -c1 -d1s http://127.0.0.1:3030/api/v0/bench-0-0" > /dev/null 2>&1

    if [ "$post" -gt 0 ]; then
        podman exec $1 sh -c "wrk -s wrk.lua -t$4 -c$post -d10s http://127.0.0.1:3030/api/v0/bench-0-0" >> bench/$1/$2-$3-post.log &
    fi

    if [ "$get" -gt 0 ]; then
        podman exec $1 sh -c "wrk -t$4 -c$get -d10s http://127.0.0.1:3030/api/v0/bench-0-0/latest" >> bench/$1/$2-$3-get.log
    fi

    printf "\nSTATS\n" >> bench/$1/$2-$3.log
    podman stats --no-stream >> bench/$1/$2-$3.log
    podman exec $1 sh -c "ps aux" >> bench/$1/$2-$3.log

    podman stop $1
}

for i in {0..39}
do
    for j in 0 5 50 95 100
    do
        fn_test $rm_container_name $i $j 4 400
        fn_test $mmav_container_name $i $j 4 400
    done
done
