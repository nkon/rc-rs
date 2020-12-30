#!/bin/sh

cargo test

cargo run -- --init tests/_rc_rc_none -s tests/test.case > tests/test.result
diff tests/test.result tests/test.answer
if [ $? -ne 0 ]; then
   echo "*** test fail. ***"
   exit 1
fi

cargo run -- --init tests/_rc_rc_none -s tests/demo.case > tests/demo.result
diff tests/demo.result tests/demo.answer
if [ $? -ne 0 ]; then
   echo "*** test fail. ***"
   exit 1
fi

cargo run --  --init tests/_rc_rc_none --test > tests/cargo_run_test.result
diff tests/cargo_run_test.result tests/cargo_run_test.answer
if [ $? -ne 0 ]; then
   echo "*** test fail. ***"
   exit 1
fi

cargo run --  1 + 2+3 > tests/arg_test.result
diff tests/arg_test.result tests/arg_test.answer
if [ $? -ne 0 ]; then
   echo "*** test fail. ***"
   exit 1
fi

echo test success.
