#!/usr/bin/env sh

sed -i -z 's/    \/\* INJECTION-START \*\/ printf("(%5lu) \[[a-zA-Z0-9_]*\] \\n", std::chrono::system_clock::now()); \/\* INJECTION-END \*\/\n//g' example.cpp