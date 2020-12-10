#!/bin/sh

function countdown() {
    if [ $# -ne 1 ]; then
      return 1
    fi
    local remain=$1
    while [ $remain -ge 0 ]; do
        echo "$((remain--))" >countdown.out
        sleep 1
    done
    return 0
}
