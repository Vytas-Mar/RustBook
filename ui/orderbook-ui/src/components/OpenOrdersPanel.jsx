const OpenOrdersPanel = () => {
  return (
    <section className="panel">
      <div className="panel-heading">
        <h2>Open Orders</h2>
        <span>0</span>
      </div>
      <div className="wip-content">
        <table>
          <thead>
            <tr>
              <th>Side</th>
              <th>Price</th>
              <th>Qty</th>
              <th>Filled</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td className="bid">BUY</td>
              <td>101.25</td>
              <td>2.50</td>
              <td>0.00</td>
              <td>
                <button type="button">Cancel</button>
              </td>
            </tr>
            <tr>
              <td className="ask">SELL</td>
              <td>101.80</td>
              <td>1.00</td>
              <td>0.30</td>
              <td>
                <button type="button">Cancel</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>
  );
};

export default OpenOrdersPanel;
