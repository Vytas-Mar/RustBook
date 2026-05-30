const ReplayPanel = () => {
  return (
    <section className="panel">
      <div className="panel-heading">
        <h2>Replay</h2>
        <span>not recording</span>
      </div>
      <div className="wip-content">
        <div className="control-row">
          <span className="control-label">Recording</span>
          <button type="button">Record</button>
          <button type="button">Stop</button>
          <button type="button">Load…</button>
          <button type="button">Save…</button>
        </div>
        <div className="control-row">
          <span className="control-label">Playback</span>
          <div className="transport">
            <button type="button" title="Reset">⏮</button>
            <button type="button" title="Step back">⏪</button>
            <button type="button" title="Pause">⏸</button>
            <button type="button" title="Play">▶</button>
            <button type="button" title="Step forward">⏩</button>
            <button type="button" title="Skip to end">⏭</button>
          </div>
          <span className="control-label" style={{ marginLeft: '1rem' }}>Speed</span>
          <select>
            <option>0.25x</option>
            <option>0.5x</option>
            <option>1x</option>
            <option>2x</option>
            <option>10x</option>
          </select>
        </div>
        <div className="control-row">
          <span className="control-label">Timeline</span>
          <div className="scrubber"></div>
          <span style={{ fontFamily: "'JetBrains Mono', monospace", fontSize: '0.75rem', color: 'var(--text-dim)' }}>
            0 / 0
          </span>
        </div>
      </div>
    </section>
  );
};

export default ReplayPanel;
