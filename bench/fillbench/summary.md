
### 64MiB fill  (wall: median / min / max; cpu median; n rounds)
| candidate | wall median | wall min | wall max | cpu median | vs A |
|---|---|---|---|---|---|
| A push-loop | 47.76 ms | 46.85 ms | 52.26 ms | 46.88 ms | 1.000x |
| B vec![0;n] | 44.5 us | 42.6 us | 55.5 us | 0.0 us | 0.001x |
| C resize | 6.61 ms | 6.01 ms | 12.27 ms | 6.56 ms | 0.138x |
| D1 extend_from_slice 4K | 6.84 ms | 6.34 ms | 9.63 ms | 6.88 ms | 0.143x |
| D2 extend_from_slice 64K | 7.07 ms | 5.78 ms | 10.77 ms | 7.19 ms | 0.148x |
| E extend(repeat.take) | 6.46 ms | 5.97 ms | 14.85 ms | 6.25 ms | 0.135x |
| F resize_with | 6.65 ms | 5.89 ms | 11.99 ms | 6.56 ms | 0.139x |

### 64MiB fill+write  (wall: median / min / max; cpu median; n rounds)
| candidate | wall median | wall min | wall max | cpu median | vs A |
|---|---|---|---|---|---|
| A push-loop | 53.80 ms | 48.45 ms | 63.52 ms | 52.50 ms | 1.000x |
| B vec![0;n] | 8.28 ms | 7.74 ms | 13.09 ms | 8.12 ms | 0.154x |
| C resize | 9.83 ms | 8.94 ms | 19.27 ms | 9.69 ms | 0.183x |
| D1 extend_from_slice 4K | 9.63 ms | 8.65 ms | 16.77 ms | 9.69 ms | 0.179x |
| D2 extend_from_slice 64K | 9.71 ms | 8.51 ms | 12.53 ms | 9.69 ms | 0.180x |
| E extend(repeat.take) | 9.48 ms | 8.95 ms | 17.29 ms | 9.38 ms | 0.176x |
| F resize_with | 9.76 ms | 8.32 ms | 19.16 ms | 9.69 ms | 0.181x |

### 4MiB fill  (wall: median / min / max; cpu median; n rounds)
| candidate | wall median | wall min | wall max | cpu median | vs A |
|---|---|---|---|---|---|
| A push-loop | 2.53 ms | 2.49 ms | 2.59 ms | 2.48 ms | 1.000x |
| B vec![0;n] | 51.7 us | 48.2 us | 59.7 us | 58.6 us | 0.020x |
| C resize | 50.0 us | 48.2 us | 56.8 us | 58.6 us | 0.020x |
| D1 extend_from_slice 4K | 52.9 us | 51.9 us | 58.5 us | 58.6 us | 0.021x |
| D2 extend_from_slice 64K | 49.7 us | 48.8 us | 55.0 us | 58.6 us | 0.020x |
| E extend(repeat.take) | 50.7 us | 47.7 us | 53.5 us | 58.6 us | 0.020x |
| F resize_with | 50.1 us | 48.4 us | 59.1 us | 58.6 us | 0.020x |

### 4MiB fill+write  (wall: median / min / max; cpu median; n rounds)
| candidate | wall median | wall min | wall max | cpu median | vs A |
|---|---|---|---|---|---|
| A push-loop | 2.66 ms | 2.60 ms | 2.71 ms | 2.60 ms | 1.000x |
| B vec![0;n] | 0.19 ms | 0.18 ms | 0.21 ms | 0.20 ms | 0.072x |
| C resize | 0.19 ms | 0.18 ms | 0.21 ms | 0.20 ms | 0.070x |
| D1 extend_from_slice 4K | 0.19 ms | 0.18 ms | 0.20 ms | 0.20 ms | 0.070x |
| D2 extend_from_slice 64K | 0.19 ms | 0.18 ms | 0.21 ms | 0.18 ms | 0.071x |
| E extend(repeat.take) | 0.19 ms | 0.18 ms | 0.20 ms | 0.20 ms | 0.070x |
| F resize_with | 0.19 ms | 0.19 ms | 0.19 ms | 0.18 ms | 0.071x |

### 256KiB fill  (wall: median / min / max; cpu median; n rounds)
| candidate | wall median | wall min | wall max | cpu median | vs A |
|---|---|---|---|---|---|
| A push-loop | 0.16 ms | 0.15 ms | 0.16 ms | 0.15 ms | 1.000x |
| B vec![0;n] | 2.6 us | 2.5 us | 2.7 us | 2.4 us | 0.016x |
| C resize | 2.6 us | 2.5 us | 2.8 us | 2.4 us | 0.017x |
| D1 extend_from_slice 4K | 2.7 us | 2.7 us | 3.0 us | 3.7 us | 0.017x |
| D2 extend_from_slice 64K | 2.5 us | 2.5 us | 2.8 us | 2.4 us | 0.016x |
| E extend(repeat.take) | 2.5 us | 2.5 us | 2.6 us | 2.4 us | 0.016x |
| F resize_with | 2.5 us | 2.4 us | 2.7 us | 2.4 us | 0.016x |

### 256KiB fill+write  (wall: median / min / max; cpu median; n rounds)
| candidate | wall median | wall min | wall max | cpu median | vs A |
|---|---|---|---|---|---|
| A push-loop | 0.16 ms | 0.16 ms | 0.17 ms | 0.16 ms | 1.000x |
| B vec![0;n] | 11.0 us | 10.9 us | 11.6 us | 11.0 us | 0.067x |
| C resize | 11.0 us | 10.7 us | 11.2 us | 11.0 us | 0.067x |
| D1 extend_from_slice 4K | 11.2 us | 11.0 us | 11.6 us | 11.0 us | 0.068x |
| D2 extend_from_slice 64K | 11.2 us | 10.7 us | 11.9 us | 11.0 us | 0.068x |
| E extend(repeat.take) | 10.9 us | 10.7 us | 11.5 us | 11.0 us | 0.066x |
| F resize_with | 11.0 us | 10.7 us | 11.5 us | 11.0 us | 0.067x |

### 1KiB fill  (wall: median / min / max; cpu median; n rounds)
| candidate | wall median | wall min | wall max | cpu median | vs A |
|---|---|---|---|---|---|
| A push-loop | 0.6 us | 0.6 us | 0.7 us | 0.6 us | 1.000x |
| B vec![0;n] | 0.0 us | 0.0 us | 0.0 us | 0.0 us | 0.051x |
| C resize | 0.0 us | 0.0 us | 0.0 us | 0.0 us | 0.040x |
| D1 extend_from_slice 4K | 0.0 us | 0.0 us | 0.0 us | 0.0 us | 0.041x |
| D2 extend_from_slice 64K | 0.0 us | 0.0 us | 0.0 us | 0.0 us | 0.041x |
| E extend(repeat.take) | 0.0 us | 0.0 us | 0.0 us | 0.0 us | 0.041x |
| F resize_with | 0.0 us | 0.0 us | 0.0 us | 0.0 us | 0.043x |

### 1KiB fill+write  (wall: median / min / max; cpu median; n rounds)
| candidate | wall median | wall min | wall max | cpu median | vs A |
|---|---|---|---|---|---|
| A push-loop | 0.7 us | 0.6 us | 0.7 us | 0.6 us | 1.000x |
| B vec![0;n] | 0.1 us | 0.1 us | 0.1 us | 0.0 us | 0.082x |
| C resize | 0.1 us | 0.1 us | 0.1 us | 0.0 us | 0.079x |
| D1 extend_from_slice 4K | 0.1 us | 0.1 us | 0.1 us | 0.0 us | 0.081x |
| D2 extend_from_slice 64K | 0.1 us | 0.1 us | 0.1 us | 0.0 us | 0.081x |
| E extend(repeat.take) | 0.1 us | 0.1 us | 0.1 us | 0.0 us | 0.082x |
| F resize_with | 0.1 us | 0.1 us | 0.1 us | 0.0 us | 0.079x |
