# Basic grafana setup for the fullnode

Update `prometheus/prometheus.yaml` targets with url to the prometheus metrics.

```yaml
  - targets: ['localhost:9184']
```
