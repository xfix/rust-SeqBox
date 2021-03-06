#!/bin/bash

exit_code=0

echo "Generating test data"
truncate -s 1K dummy

mkdir out_test &>/dev/null
rm out_test/* &>/dev/null

echo "Testing encode output with no provided path"
rm dummy.sbx &>/dev/null
./rsbx encode dummy &>/dev/null

if [ -f "dummy.sbx" ]; then
  echo "==> Okay"
else
  echo "==> NOT okay"
  exit_code=1
fi

echo "Testing encode output with provided full file path"
./rsbx encode dummy out_test/dummy1.sbx &>/dev/null

if [ -f "out_test/dummy1.sbx" ]; then
  echo "==> Okay"
else
  echo "==> NOT okay"
  exit_code=1
fi

echo "Testing encode output with provided directory path"
./rsbx encode dummy out_test &>/dev/null

if [ -f "out_test/dummy.sbx" ]; then
  echo "==> Okay"
else
  echo "==> NOT okay"
  exit_code=1
fi

echo "Testing decode output with no provided path"
rm dummy &>/dev/null
./rsbx decode dummy.sbx &>/dev/null

if [ -f "dummy" ]; then
  echo "==> Okay"
else
  echo "==> NOT okay"
  exit_code=1
fi

echo "Testing decode output with provided full file path"
./rsbx decode dummy.sbx out_test/decoded &>/dev/null

if [ -f "out_test/decoded" ]; then
  echo "==> Okay"
else
  echo "==> NOT okay"
  exit_code=1
fi

echo "Testing decode output with provided directory path"
./rsbx decode dummy.sbx out_test &>/dev/null

if [ -f "out_test/dummy" ]; then
  echo "==> Okay"
else
  echo "==> NOT okay"
  exit_code=1
fi

echo "Regenrating test data"
truncate -s 1K dummy

echo "Encode with no metadata"
rm dummy.sbx &>/dev/null
./rsbx encode dummy --no-meta &>/dev/null

echo "Repeating same tests for decoding"
echo "Testing decode output with no provided path"
rm dummy &>/dev/null
./rsbx decode dummy.sbx &>/dev/null

if [ ! -f "dummy" ]; then
  echo "==> Okay"
else
  echo "==> NOT okay"
  exit_code=1
fi

rm out_test/* &>/dev/null

echo "Testing decode output with provided full file path"
./rsbx decode dummy.sbx out_test/decoded &>/dev/null

if [ -f "out_test/decoded" ]; then
  echo "==> Okay"
else
  echo "==> NOT okay"
  exit_code=1
fi

echo "Testing decode output with provided directory path"
./rsbx decode dummy.sbx out_test &>/dev/null

if [ ! -f "out_test/dummy" ]; then
  echo "==> Okay"
else
  echo "==> NOT okay"
  exit_code=1
fi

exit $exit_code
