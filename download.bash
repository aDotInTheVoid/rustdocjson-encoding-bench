#!/bin/bash
set -eoxu pipefail

rm -rf corpus/*.json

curl https://docs.rs/crate/libc/0.2.173/json/46 --compressed -L -o ./corpus/libc.json
curl https://docs.rs/crate/aws-sdk-ec2/1.139.0/json/46 --compressed -L -o ./corpus/aws-sdk-ec2.json
curl https://docs.rs/crate/quiche/0.24.4/json/46  --compressed -L -o ./corpus/quiche.json

sha256sum --check ./checksums.txt --strict
