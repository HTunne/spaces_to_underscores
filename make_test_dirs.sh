#!/bin/bash

rm -r test
mkdir test
pushd test
for i in {1..3}
do
  touch test_file\ ${i}.txt
  touch a_test_file_${i}.txt
  mkdir test_dir\ ${i}
  pushd test_dir\ ${i}
  for j in {1..3}
  do
    touch test_file\ ${j}.txt
    touch a_test_file_${j}.txt
    mkdir test_dir\ ${j}
    pushd test_dir\ ${j}
    for k in {1..3}
    do
      touch test_file\ ${k}.txt
      touch a_test_file_${k}.txt
    done
    popd
  done
  popd
  mkdir a_test_dir_${i}
  pushd a_test_dir_${i}
  for j in {1..3}
  do
    touch test_file\ ${j}.txt
    touch a_test_file_${j}.txt
    mkdir a_test_dir_${j}
    pushd a_test_dir_${j}
    for k in {1..3}
    do
      touch test_file\ ${k}.txt
      touch a_test_file_${k}.txt
    done
    popd
  done
  popd

done
