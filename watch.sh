#!/bin/bash

cargo watch -c \
    -x "check --tests                " \
    -x "fmt                          " \
    -x "clippy --tests               " \
    -x "doc --document-private-items " \
    -x "test                         " \
    -x "bench                        " \
