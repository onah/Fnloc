#!/bin/bash

# Generate test results for Fnloc project
set -e

echo "Running tests and generating results..."

# Run tests and capture output
cargo test --verbose 2>&1 | tee test-output.txt

# Create test-results directory
mkdir -p test-results

# Extract test results from output
TOTAL=$(grep "running.*tests" test-output.txt | tail -1 | grep -o "[0-9]\+" | head -1 || echo "0")
PASSED=$(grep "test result: ok" test-output.txt | grep -o "[0-9]\+ passed" | grep -o "[0-9]\+" || echo "0")
FAILED=$(grep "test result:" test-output.txt | grep -o "[0-9]\+ failed" | grep -o "[0-9]\+" || echo "0")

# If no explicit failed count, calculate it
if [ "$FAILED" = "0" ] && grep -q "test result: ok" test-output.txt; then
  FAILED=0
  if [ "$PASSED" = "0" ]; then
    PASSED=$TOTAL
  fi
fi

# Create markdown summary
cat > test-results/test-summary.md << EOF
# Fnloc Test Results

**Date:** $(date -u +"%Y-%m-%d %H:%M:%S UTC")
**Commit:** ${GITHUB_SHA:-"local"}
**Branch:** ${GITHUB_REF_NAME:-"local"}

## Test Summary
- **Total Tests:** $TOTAL
- **Passed:** $PASSED
- **Failed:** $FAILED

EOF

if [ "$FAILED" -eq "0" ]; then
  echo "**Status:** ✅ All tests passed" >> test-results/test-summary.md
else
  echo "**Status:** ❌ Some tests failed" >> test-results/test-summary.md
fi

# Create JSON format
cat > test-results/test-results.json << EOF
{
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "commit": "${GITHUB_SHA:-"local"}",
  "branch": "${GITHUB_REF_NAME:-"local"}",
  "total_tests": $TOTAL,
  "passed_tests": $PASSED,
  "failed_tests": $FAILED,
  "success": $(if [ "$FAILED" -eq "0" ]; then echo "true"; else echo "false"; fi)
}
EOF

# Display results for debugging
echo "=== Test Results ==="
cat test-results/test-summary.md
echo ""
echo "=== JSON Results ==="
cat test-results/test-results.json

echo "Test results generated successfully!"
