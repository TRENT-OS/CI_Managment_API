#!/bin/bash

rsync -avh --progress target/release/CI_Managment_API migrations/20240725080658_runners.sql t1.hg127571@ac.ebs.corp@10.70.192.2:/tmp/