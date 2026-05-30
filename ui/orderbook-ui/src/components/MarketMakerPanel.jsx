const MarketMakerPanel = () => {
  return (
    <section className="panel">
      <div className="panel-heading">
        <h2>Market Maker Bot</h2>
        <span>off</span>
      </div>
      <div className="wip-content">
        <div className="control-row">
          <span className="control-label">Strategy</span>
          <select>
            <option>Avellaneda-Stoikov</option>
            <option>Constant spread</option>
            <option>Inventory-skewed</option>
          </select>
          <button type="button" className="place-order">Start</button>
          <button type="button">Stop</button>
        </div>
        <div className="mm-row">
          <span className="mm-label">PnL (realized)</span>
          <span className="mm-value">—</span>
        </div>
        <div className="mm-row">
          <span className="mm-label">PnL (unrealized)</span>
          <span className="mm-value">—</span>
        </div>
        <div className="mm-row">
          <span className="mm-label">Inventory</span>
          <span className="mm-value">0</span>
        </div>
        <div className="mm-row">
          <span className="mm-label">Quote spread</span>
          <span className="mm-value">—</span>
        </div>
        <div className="mm-row">
          <span className="mm-label">Adverse selection</span>
          <span className="mm-value">—</span>
        </div>
        <div className="mm-row">
          <span className="mm-label">Fills (maker / taker)</span>
          <span className="mm-value">0 / 0</span>
        </div>
      </div>
    </section>
  );
};

export default MarketMakerPanel;
