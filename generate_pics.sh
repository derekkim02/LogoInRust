#!/bin/bash 

cd logo_examples
cargo build

for file in *.lg; do
	prefix=$(echo $file | sed 's/\(.*\)\..*/\1/')
	./../target/debug/rslogo  $file ${prefix}_mine.svg 500 500
done

mv *.svg ../user_images
cd -