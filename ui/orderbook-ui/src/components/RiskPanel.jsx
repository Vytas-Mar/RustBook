import { useEffect, useState } from "react";
import { WasmEngine } from "engine_wasm";
import { toast } from "sonner";
import InfoTip from "./InfoTip";

const PRICE_SCALE = Number(WasmEngine.price_scale());

// Pull defaults from the WASM engine so UI and Rust never drift apart.
const RAW_DEFAULTS = WasmEngine.default_risk_gate();
const DEFAULTS = {
  max_order_qty: RAW_DEFAULTS.max_order_qty
    ? Number(RAW_DEFAULTS.max_order_qty)
    : null,
  max_notional: RAW_DEFAULTS.max_notional
    ? Number(RAW_DEFAULTS.max_notional)
    : null,
  max_price_deviation: RAW_DEFAULTS.max_price_deviation
    ? Number(RAW_DEFAULTS.max_price_deviation)
    : null,
};

const RiskPanel = ({ onApply }) => {
  const [maxQty, setMaxQty] = useState(DEFAULTS.max_order_qty ?? "");
  const [maxNotional, setMaxNotional] = useState(DEFAULTS.max_notional ?? "");
  const [maxDeviation, setMaxDeviation] = useState(
    DEFAULTS.max_price_deviation ?? "",
  );

  const apply = () => {
    const config = {
      max_order_qty: maxQty === "" ? null : BigInt(maxQty),
      max_notional: maxNotional === "" ? null : BigInt(maxNotional),
      max_price_deviation:
        maxDeviation === "" ? null : BigInt(maxDeviation),
    };
    try {
      onApply(config);
      toast.success("Risk gate updated");
    } catch (err) {
      toast.error(`Failed to update risk gate: ${err}`);
    }
  };

  const resetToDefaults = () => {
    setMaxQty(DEFAULTS.max_order_qty ?? "");
    setMaxNotional(DEFAULTS.max_notional ?? "");
    setMaxDeviation(DEFAULTS.max_price_deviation ?? "");
  };

  const clearAll = () => {
    setMaxQty("");
    setMaxNotional("");
    setMaxDeviation("");
  };

  // Convert max_notional and max_price_deviation for display since they're
  // in scaled units (price × qty for notional, raw price units for deviation).
  const notionalDisplay = maxNotional
    ? `($${(Number(maxNotional) / PRICE_SCALE).toLocaleString()})`
    : "";
  const deviationDisplay = maxDeviation
    ? `($${(Number(maxDeviation) / PRICE_SCALE).toFixed(2)})`
    : "";

  return (
    <section className="panel">
      <div className="panel-heading">
        <h2>Risk Gate</h2>
        <span>pre-trade checks</span>
      </div>
      <div>
        <div className="risk-note">
          Pre-trade checks applied before any order touches the book. Leave a
          field blank to disable that specific check. Market orders are only
          subject to the quantity check (their price is synthetic).
        </div>

        <div className="risk-form">
          <label className="risk-field">
            <span>
              Max order qty
              <InfoTip>
                Rejects any single order whose quantity exceeds this. The
                first line of fat-finger defence — caps how big any one
                instruction can be.
              </InfoTip>
            </span>
            <input
              type="number"
              min="0"
              value={maxQty}
              onChange={(e) => setMaxQty(e.target.value)}
              placeholder="(no limit)"
            />
          </label>

          <label className="risk-field">
            <span>
              Max notional <small>{notionalDisplay}</small>
              <InfoTip>
                Rejects any single order whose price × qty exceeds this.
                Numbers are in scaled units (PRICE_SCALE = {PRICE_SCALE}). At
                PRICE_SCALE=100, value 100,000,000 = $1,000,000.
              </InfoTip>
            </span>
            <input
              type="number"
              min="0"
              value={maxNotional}
              onChange={(e) => setMaxNotional(e.target.value)}
              placeholder="(no limit)"
            />
          </label>

          <label className="risk-field">
            <span>
              Max price deviation <small>{deviationDisplay}</small>
              <InfoTip>
                Rejects orders priced more than this far from current mid
                (best_bid + best_ask) / 2. Only fires when the book has both
                sides. In scaled units — value 500 at PRICE_SCALE=100 means $5.
              </InfoTip>
            </span>
            <input
              type="number"
              min="0"
              value={maxDeviation}
              onChange={(e) => setMaxDeviation(e.target.value)}
              placeholder="(no limit)"
            />
          </label>
        </div>

        <div className="control-row">
          <button type="button" onClick={apply} className="place-order">
            Apply
          </button>
          <button type="button" onClick={resetToDefaults}>
            Reset to defaults
          </button>
          <button type="button" onClick={clearAll}>
            Clear all (disable)
          </button>
        </div>
      </div>
    </section>
  );
};

export default RiskPanel;
