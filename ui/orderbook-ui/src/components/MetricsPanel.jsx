const MetricsPanel = () => {
  return (
    <section className="panel">
      <div className="panel-heading">
        <h2>Metrics</h2>
        <span>live</span>
      </div>
      <div className="wip-content">
        <div className="stat-grid">
          <div className="stat">
            <div className="stat-label">Throughput</div>
            <div className="stat-value muted">— /s</div>
          </div>
          <div className="stat">
            <div className="stat-label">p50 latency</div>
            <div className="stat-value muted">— μs</div>
          </div>
          <div className="stat">
            <div className="stat-label">p99 latency</div>
            <div className="stat-value muted">— μs</div>
          </div>
          <div className="stat">
            <div className="stat-label">p99.9 latency</div>
            <div className="stat-value muted">— μs</div>
          </div>
          <div className="stat">
            <div className="stat-label">Max latency</div>
            <div className="stat-value muted">— μs</div>
          </div>
          <div className="stat">
            <div className="stat-label">Open orders</div>
            <div className="stat-value muted">0</div>
          </div>
          <div className="stat">
            <div className="stat-label">Heap (KB)</div>
            <div className="stat-value muted">—</div>
          </div>
          <div className="stat">
            <div className="stat-label">Alloc/sec</div>
            <div className="stat-value muted">—</div>
          </div>
        </div>
        <div className="chart-placeholder">latency histogram</div>
        <div style={{ height: '0.5rem' }}></div>
        <div className="chart-placeholder">throughput sparkline</div>
      </div>
    </section>
  );
};

export default MetricsPanel;
