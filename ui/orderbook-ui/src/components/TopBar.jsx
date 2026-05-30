const TopBar = ({ statusText }) => {
  return (
    <header className="topbar">
      <div className="brand">
        <div className="brand-mark">RUSTBOOK</div>
        <div className="brand-sub">matching engine · wasm · rust</div>
      </div>

      <div className="topbar-center">
        <select className="symbol-select" defaultValue="BTC/USDT">
          <option>BTC/USDT</option>
          <option disabled>ETH/USDT (wip)</option>
          <option disabled>ES futures (wip)</option>
        </select>
      </div>

      <div className="topbar-meta">
        <div className="topbar-meta-item">
          <span>seq</span>
          <strong>0</strong>
        </div>
        <div className="topbar-meta-item">
          <span>p50</span>
          <strong>—</strong>
        </div>
        <div className="topbar-meta-item">
          <span>p99</span>
          <strong>—</strong>
        </div>
        <span className="status">{statusText}</span>
      </div>
    </header>
  );
};

export default TopBar;
