use engine_core::{Orderbook, Side};

pub fn assert_book_invariants(book: &Orderbook) {
    // no empty levels
    for (price, level) in &book.bids {
        assert!(!level.orders.is_empty(), "empty bid level at price {price}");
    }

    for (price, level) in &book.asks {
        assert!(!level.orders.is_empty(), "empty ask level at price {price}");
    }

    // order sum matches total_qty
    for (price, level) in &book.bids {
        let actual: u64 = level.orders.iter().map(|order| order.qty).sum();

        assert_eq!(
            level.total_qty, actual,
            "bid qty mismatch at price {}: cached {}, actual: {}",
            price, level.total_qty, actual,
        );
    }

    for (price, level) in &book.asks {
        let actual: u64 = level.orders.iter().map(|order| order.qty).sum();

        assert_eq!(
            level.total_qty, actual,
            "ask qty mismatch at price {}: cached {}, actual: {}",
            price, level.total_qty, actual,
        );
    }

    // no zero qty orders
    for (price, level) in &book.bids {
        for order in &level.orders {
            assert!(
                order.qty > 0,
                "zero qty bid order: id {}, at price {}",
                order.id,
                price
            );
        }
    }

    for (price, level) in &book.asks {
        for order in &level.orders {
            assert!(
                order.qty > 0,
                "zero qty ask order: id {}, at price {}",
                order.id,
                price
            );
        }
    }

    // book not crossed
    if let (Some(best_bid), Some(best_ask)) = (book.best_bid_price(), book.best_ask_price()) {
        assert!(
            best_bid < best_ask,
            "book crossed: best bid {} >= best ask {}",
            best_bid,
            best_ask
        );
    }

    // FIFO per level
    for (price, level) in &book.bids {
        for (a, b) in level.orders.iter().zip(level.orders.iter().skip(1)) {
            assert!(
                a.id < b.id,
                "FIFO broken at bid price {}: order {} should have id less than {}",
                price,
                a.id,
                b.id,
            )
        }
    }

    for (price, level) in &book.asks {
        for (a, b) in level.orders.iter().zip(level.orders.iter().skip(1)) {
            assert!(
                a.id < b.id,
                "FIFO broken at ask price {}: order {} should have id less than {}",
                price,
                a.id,
                b.id,
            )
        }
    }

    // Buy and Sells sit correctly in corresponding VecDeq
    for (price, level) in &book.bids {
        for order in &level.orders {
            assert_eq!(
                order.side,
                Side::Buy,
                "non-buy order in bids at price {}: order {} has side {:?}",
                price,
                order.id,
                order.side
            )
        }
    }

    for (price, level) in &book.asks {
        for order in &level.orders {
            assert_eq!(
                order.side,
                Side::Sell,
                "non-sell order in asks at price {}: order {} has side {:?}",
                price,
                order.id,
                order.side
            )
        }
    }

    assert_index_consistent(book);
}

pub fn assert_index_consistent(book: &Orderbook) {
    use std::collections::HashSet;

    let mut from_levels: HashSet<(u64, u64, Side)> = HashSet::new();
    for (&price, level) in &book.bids {
        for order in &level.orders {
            from_levels.insert((order.id, price, Side::Buy));
        }
    }
    for (&price, level) in &book.asks {
        for order in &level.orders {
            from_levels.insert((order.id, price, Side::Sell));
        }
    }

    let from_index: HashSet<(u64, u64, Side)> =
        book.index.iter().map(|(&id, &(p, s))| (id, p, s)).collect();

    assert_eq!(from_levels, from_index, "index out of sync with book");
}
