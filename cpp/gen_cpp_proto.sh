#!/bin/sh
protoc -I ./ --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_cpp_plugin` ../proto/helloworld.proto
protoc -I ./ --cpp_out=. ../proto/helloworld.proto
