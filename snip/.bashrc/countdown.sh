#!/bin/bash

. ${here}/valid_num_args.sh

function countdown() {
    valid_num_args $# 1
    local remain=$1
    while [ $remain -ge 0 ]; do
        echo "$((remain--))" >countdown.out
        sleep 1
    done
    return 0
}
