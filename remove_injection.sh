#!/usr/bin/env sh

sed -i -z 's/    printf("Injected!\\n");\n//g' example.cpp