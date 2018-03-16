#!/bin/bash

VERSIONS=(17 18 19)

corrupt() {
    dd if=/dev/zero of=$2 bs=1 count=1 seek=$1 conv=notrunc &>/dev/null
}

file_size=$[1024 * 1024 * 1]

# generate test data
dd if=/dev/urandom of=dummy bs=$file_size count=1 &>/dev/null

for ver in ${VERSIONS[*]}; do
    for (( i=0; i < 3; i++ )); do
        data_shards=$((1 + RANDOM % 128))
        parity_shards=$((1 + RANDOM % 128))

        container_name=corrupt_$data_shards\_$parity_shards\_$ver.sbx

        echo "Encoding in version $ver, data = $data_shards, parity = $parity_shards"
        ./rsbx encode --sbx-version $ver -f dummy $container_name \
               --rs-data $data_shards --rs-parity $parity_shards &>/dev/null

        echo "Corrupting at $parity_shards random positions"
        for (( p=0; p < $parity_shards; p++ )); do
            pos=$((RANDOM % $file_size))
            # echo "#$p corruption, corrupting byte at position : $pos"
            corrupt $pos $container_name
        done

        echo "Repairing $container_name"
        ./rsbx repair -y $container_name &>/dev/null

        output_name=dummy_$data_shards\_$parity_shards

        echo "Decoding $container_name"
        ./rsbx decode -f $container_name $output_name &>/dev/null

        echo "Comparing decoded data to original"
        cmp dummy $output_name
        if [[ $? == 0 ]]; then
            echo "==> Okay"
        else
            echo "==> NOT okay"
        fi
    done
done
