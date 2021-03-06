// 901. Online Stock Span.
// https://leetcode.com/problems/online-stock-span/

use std::collections::BTreeMap;

pub struct StockSpanner {
    day: i32,
    map: BTreeMap<i32, i32>,
}

impl StockSpanner {
    pub fn new() -> Self {
        StockSpanner {
            day: 0,
            map: BTreeMap::new()
        }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        self.day += 1;
        let nearest_greater_day = *self.map.range((price + 1)..).map(|(_k, v)| v).max().unwrap_or(&0);
        let span = self.day - nearest_greater_day;
        self.map.insert(price, self.day);
        span
    }
}

pub struct StockSpannerV2 {
    stack: Vec<(i32, i32)>,
}

impl StockSpannerV2 {
    pub fn new() -> Self {
        StockSpannerV2 {
            stack: Vec::new(),
        }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        let mut res = 1;

        while let Some((prev_price, span)) = self.stack.pop() {
            if prev_price <= price {
                res += span;
            } else {
                // Push the greater price back.
                self.stack.push((prev_price, span));
                break;
            }
        }

        self.stack.push((price, res));

        res
    }
}