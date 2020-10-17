#!/bin/bash

cargo watch -c                            \
    -x "check  --workspace              " \
    -x "fmt                             " \
    -x "clippy --workspace              " \
    -x "doc    --document-private-items " \
    -x "test   --workspace -q           "
