# Zenith Performance Measurement Methodology

## Overview
Zenith uses a tiered performance testing strategy to balance fast feedback during development with thorough regression testing before releases.

## Test Suites

### 1. Quick Suite (`quick`)
- **Objective**: Fast feedback for Pull Requests.
- **Parameters**: 
  - Warmup runs: 1
  - Measurement runs: 3
  - Max retries on instability: 1
- **Coverage**: Micro benchmarks for core layers and small macro scenarios.

### 2. Nightly Suite (`nightly`)
- **Objective**: Deep regression testing and long-term performance tracking.
- **Parameters**:
  - Warmup runs: 2
  - Measurement runs: 7
  - Max retries on instability: 2
- **Coverage**: All benchmarks including large projects and heavy stress scenarios.

## Metrics
- **Latency**: Measured in milliseconds (ms). Reports median, p95, and standard deviation.
- **Throughput**: Measured in operations per second (ops/sec).
- **Memory**: Peak Working Set (WS) in bytes.
- **Binary Size**: Final executable size in bytes.

## Evaluation Criteria
- **Budget**: Each benchmark has a predefined `warn` and `fail` budget.
- **Baseline**: Comparison against a "frozen" baseline for the platform.
- **Regression**: Any result > 10% worse than the baseline (or budget) is flagged as a failure.

## Outlier Handling
- Uses IQR (1.5x) to detect and filter transient spikes.
- Automatic rerun of unstable samples to ensure reliable results.
