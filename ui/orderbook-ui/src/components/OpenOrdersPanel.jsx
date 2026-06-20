import { useState } from "react";

const AmendCell = ({ order, onAmend }) => {
  const [amendQty, setAmendQty] = useState("");

  const submit = () => {
    const n = Number(amendQty);
    if (!Number.isFinite(n) || n <= 0) return;
    if (n >= order.qty) return;
    onAmend(order.id, Math.trunc(n));
    setAmendQty("");
  };

  return (
    <div className="amend-cell">
      <input
        type="number"
        min="1"
        max={order.qty - 1}
        placeholder={`new qty (< ${order.qty})`}
        value={amendQty}
        onChange={(e) => setAmendQty(e.target.value)}
      />
      <button
        type="button"
        onClick={submit}
        title="Reduce qty in place — preserves your FIFO queue position. Increasing qty would lose priority (do that by canceling and placing a new order)."
      >
        Amend
      </button>
    </div>
  );
};

const OpenOrdersPanel = ({ orders = [], onCancel, onAmend }) => {
  return (
    <section className="panel">
      <div className="panel-heading">
        <h2>Open Orders</h2>
        <span>
          {orders.length} {orders.length === 1 ? "order" : "orders"}
        </span>
      </div>

      {orders.length === 0 ? (
        <div className="open-orders-empty">
          No resting orders. Place a limit order from the Order Entry panel to
          see it here. Cancel and amend (size-down only) supported.
        </div>
      ) : (
        <div className="open-orders-scroll">
          <table className="open-orders-table">
            <thead>
              <tr>
                <th>ID</th>
                <th>Side</th>
                <th>Price</th>
                <th>Qty</th>
                <th>Filled</th>
                <th>Amend</th>
                <th>Cancel</th>
              </tr>
            </thead>
            <tbody>
              {orders.map((o) => {
                const filled = o.originalQty - o.qty;
                const isBuy = o.side === "buy";
                return (
                  <tr key={o.id}>
                    <td className="oo-id">#{o.id}</td>
                    <td className={isBuy ? "bid" : "ask"}>
                      {isBuy ? "BUY" : "SELL"}
                    </td>
                    <td>{o.price.toFixed(2)}</td>
                    <td>{o.qty}</td>
                    <td>{filled > 0 ? `${filled} / ${o.originalQty}` : "—"}</td>
                    <td>
                      <AmendCell order={o} onAmend={onAmend} />
                    </td>
                    <td>
                      <button
                        type="button"
                        className="oo-cancel"
                        onClick={() => onCancel(o.id)}
                      >
                        ×
                      </button>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}
    </section>
  );
};

export default OpenOrdersPanel;
