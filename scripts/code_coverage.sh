cargo tarpaulin --verbose --workspace --out xml --out html --output-dir ./coverage > coverage_output.txt 2>&1
cat coverage_output.txt

COVERAGE=$(grep -oP '\d+\.\d+(?=% coverage)' coverage_output.txt | head -1)
          
if [ -z "$COVERAGE" ]; then
  echo "Failed to extract coverage"
  exit 1
fi

echo "Coverage: ${COVERAGE}%"
if (( $(echo "$COVERAGE < 70" | bc -l) )); then
  exit 1
fi