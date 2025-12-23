#!/bin/bash

function print_qr_url () {
    echo
    echo $1
    echo
    qrcode-terminal $1
    echo
}


print_qr_url "https://ahaslides.com/HPMTJ"

print_qr_url "https://github.com/dabevlohn/yp-course"
