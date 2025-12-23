#!/bin/bash

INP=$1

function print_qr_url () {
    echo
    echo $1
    qrcode-terminal $1
}

case $INP in
    1)
        print_qr_url "https://ahaslides.com/HPMTJ"
        ;;
    2)
        print_qr_url "https://github.com/dabevlohn/yp-course"
        ;;
esac
