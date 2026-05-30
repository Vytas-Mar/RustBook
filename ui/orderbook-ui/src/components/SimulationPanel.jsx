const SimulationPanel = () => {
  return (
    <section className="panel">
      <div className="panel-heading">
        <h2>Simulation</h2>
        <span>idle</span>
      </div>
      <div className="wip-content">
        <div className="control-row">
          <span className="control-label">Generator</span>
          <select>
            <option>Poisson λ=10/s</option>
            <option>Burst</option>
            <option>Vol clustering</option>
            <option>Replay sample</option>
          </select>
          <span className="control-label">Duration</span>
          <input type="number" defaultValue="60" style={{ width: '5rem' }} />
        </div>
        <div className="control-row">
          <button type="button">Burst 100</button>
          <button type="button">Burst 1k</button>
          <button type="button">Burst 10k</button>
          <button type="button" className="place-order">Run Sim</button>
          <button type="button">Clear Book</button>
        </div>
        <div className="stat-grid">
          <div className="stat">
            <div className="stat-label">Throughput</div>
            <div className="stat-value muted">—</div>
          </div>
          <div className="stat">
            <div className="stat-label">Orders sent</div>
            <div className="stat-value muted">0</div>
          </div>
          <div className="stat">
            <div className="stat-label">Trades</div>
            <div className="stat-value muted">0</div>
          </div>
          <div className="stat">
            <div className="stat-label">Elapsed</div>
            <div className="stat-value muted">0.00s</div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default SimulationPanel;
