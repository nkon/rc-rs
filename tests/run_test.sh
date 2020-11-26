#!/bin/sh

cargo test

cargo run -- -s tests/test.case > tests/test.result
diff tests/test.result tests/test.answer
if [ $? -ne 0 ]; then
   echo "*** test fail. ***"
   exit 1
fi

echo test success.
