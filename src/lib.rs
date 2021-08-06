use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GTableConfig {
    pub name: String,
    pub code: String,
    pub start_price: f32,
    pub start_numbers: f32,
    pub small_grid: f32,
    pub medium_grid: f32,
    pub large_grid: f32,
}

#[derive(Debug)]
pub struct GTable {
    pub name: String,
    pub code: String,
    pub start_price: f32,
    pub start_numbers: f32,
}

#[derive(Debug)]
pub struct GRow {
    pub gear: f32,
    pub buy_price: f32,
    pub buy_numbers: f32,
}

trait Rounder {
    fn round_100(input: f32) -> f32 {
        if input.round() as usize % 100 == 0 {
            return input;
        } else {
            return (input / 100.0).round() * 100.0;
        }
    }
}

impl Rounder for GTable {}

impl GTable {
    pub fn new(config: &GTableConfig) -> GTable {
        GTable {
            name: config.name.clone(),
            code: config.code.clone(),
            start_price: config.start_price,
            start_numbers: config.start_numbers,
        }
    }

    pub fn nth_row(&self, n: u32, grid: f32) -> GRow {
        let buy_price = self.start_price * (1.0 - n as f32 * grid);
        let current_buy_money = self.start_price * self.start_numbers * (1.0 + n as f32 * grid);
        let buy_numbers = current_buy_money / buy_price;

        GRow {
            gear: 1.0 - n as f32 * grid,
            buy_price: buy_price,
            buy_numbers: GTable::round_100(buy_numbers),
        }
    }
}

impl Rounder for GRow {}

impl GRow {
    pub fn new(gear: f32, buy_price: f32, buy_numbers: f32) -> GRow {
        GRow {
            gear,
            buy_price,
            buy_numbers,
        }
    }

    pub fn sell_price(&self, grid: f32) -> f32 {
        self.buy_price * (self.gear + grid) / self.gear
    }

    pub fn sell_numbers(&self, grid: f32) -> f32 {
        let t = if grid > 0.10 { grid } else { grid * 2.0 };
        GRow::round_100(self.buy_numbers * (1.0 - t))
    }

    pub fn buy_trigger_price(&self) -> f32 {
        self.buy_price + 0.005
    }

    pub fn buy_money(&self) -> f32 {
        self.buy_price * self.buy_numbers
    }

    pub fn sell_trigger_price(&self, grid: f32) -> f32 {
        self.sell_price(grid) - 0.005
    }
}

#[test]
fn round_100() {
    assert_eq!(1000.0, GTable::round_100(1001.0));
    assert_eq!(1000.0, GTable::round_100(1000.0));
    assert_eq!(1000.0, GTable::round_100(999.99));
    assert_eq!(1000.0, GTable::round_100(999.0));
    assert_eq!(1000.0, GTable::round_100(950.0));
    assert_eq!(900.0, GTable::round_100(949.0));
}

#[test]
fn gtable_new() {
    let config = GTableConfig {
        name: "中概互联".to_string(),
        code: "513050".to_string(),
        start_price: 1.0,
        start_numbers: 10000.0,
        small_grid: 0.05,
        medium_grid: 0.15,
        large_grid: 0.30,
    };

    let table = GTable::new(&config);
    assert_eq!("中概互联".to_string(), table.name);
    assert_eq!("513050".to_string(), table.code);
    assert_eq!(0.05, config.small_grid);
    assert_eq!(0.15, config.medium_grid);
    assert_eq!(0.30, config.large_grid);
}

#[test]
fn grow_new_1() {
    let row = GRow::new(1.0, 1.0, 10000.0);
    assert_eq!(1.0, row.gear);
    assert_eq!(1.0, row.buy_price);
    assert_eq!(10000.0, row.buy_numbers);
    assert_eq!(1.05, row.sell_price(0.05));
    assert_eq!(9000.0, row.sell_numbers(0.05));
}

#[test]
fn grow_new_2() {
    let row = GRow::new(1.0, 1.0, 10000.0);
    assert_eq!(1.0, row.gear);
    assert_eq!(1.0, row.buy_price);
    assert_eq!(10000.0, row.buy_numbers);
    assert_eq!(1.10, row.sell_price(0.10));
    assert_eq!(8000.0, row.sell_numbers(0.10));
}

#[test]
fn grow_new_3() {
    let row = GRow::new(1.0, 1.0, 10000.0);
    assert_eq!(1.0, row.gear);
    assert_eq!(1.0, row.buy_price);
    assert_eq!(10000.0, row.buy_numbers);
    assert_eq!(1.15, row.sell_price(0.15));
    assert_eq!(8500.0, row.sell_numbers(0.15));
}

#[test]
fn table_nth_row() {
    let config = GTableConfig {
        name: "中概互联".to_string(),
        code: "513050".to_string(),
        start_price: 1.0,
        start_numbers: 10012.0,
        small_grid: 0.05,
        medium_grid: 0.15,
        large_grid: 0.30,
    };

    let table = GTable::new(&config);
    let mut row = table.nth_row(0, 0.05);
    assert_eq!(1.0, row.gear);
    assert_eq!(1.0, row.buy_price);
    assert_eq!(10000.0, row.buy_numbers);
    assert_eq!(1.05, row.sell_price(config.small_grid));
    assert_eq!(9000.0, row.sell_numbers(config.small_grid));

    row = table.nth_row(1, 0.05);
    assert_eq!(0.95, row.gear);
    assert_eq!(0.95, row.buy_price);
    assert_eq!(11100.0, row.buy_numbers);
    assert_eq!(1.0, row.sell_price(config.small_grid));
    assert_eq!(10000.0, row.sell_numbers(config.small_grid));

    row = table.nth_row(2, 0.05);
    assert_eq!(0.9, row.gear);
    assert_eq!(0.9, row.buy_price);
    assert_eq!(12200.0, row.buy_numbers);
    assert_eq!(0.95, row.sell_price(config.small_grid));
    assert_eq!(11000.0, row.sell_numbers(config.small_grid));
}
