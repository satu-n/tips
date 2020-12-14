#!/bin/bash

. ${here}/valid_num_args.sh

function countdown() {
    valid_num_args $# 1
    local output="countdown.out"
    echo "counting down @ $output"
    local remain=$1
    while [ $remain -ge 0 ]; do
        echo "$((remain--))" >$output
        sleep 1
    done
    echo "fin"
    return 0
}
