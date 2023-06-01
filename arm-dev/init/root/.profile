#! /bin/sh

set -e

nixos-rebuild switch

rm /root/.profile

shutdown now