use std::error::Error;
use std::fs::File;

use griddle::{GRow, GTable, GTableConfig};

struct PrintKey<'a> {
    buy_price: f32,
    color: &'a str,
    grid_name: &'a str,
    grid: f32,
}

struct Printer<'a> {
    rows: Vec<(PrintKey<'a>, GRow)>,
}

impl<'a> Printer<'a> {
    fn print_header(name: &str, code: &str) {
        println!("#### {}（{}）", code, name);
        println!("\n");
        println!("{}", "| 种类 | 档位 | 买入触发价 | 买入价 | 买入金额 | 入股数 | 卖出触发价 | 卖出价 | 出股数 |");
        println!("{}", "| ---- | ---- | ---------- | ------ | -------- | ------ | ---------- | ------ | :----- |");
    }

    fn add_rows(
        &mut self,
        table: &'a GTable,
        color: &'a str,
        grid_name: &'a str,
        grid: f32,
        mut index: u32,
    ) {
        let mut gear = 1.0;
        let lowest_gear = 0.4;
        while gear > lowest_gear {
            let row = table.nth_row(index, grid);
            if row.gear < lowest_gear {
                break;
            }
            let key = PrintKey {
                buy_price: row.buy_price,
                color: color,
                grid_name: grid_name,
                grid: grid,
            };
            self.rows.push((key, row));
            index += 1;
            gear -= grid;
        }
    }

    fn print_row(&self, key: &PrintKey, row: &GRow) {
        println!(
            "|<span style=\"color:{}\"> {} </span>| {:.2} | {:.3} | {:.3} | {} | {} | {:.3} | {:.3} | {} |",
            key.color,
            key.grid_name,
            row.gear,
            row.buy_trigger_price(),
            row.buy_price,
            row.buy_money() as u32,
            row.buy_numbers as u32,
            row.sell_trigger_price(key.grid),
            row.sell_price(key.grid),
            row.sell_numbers(key.grid) as u32,
        );
    }

    fn print(&mut self) {
        self.rows
            .sort_by(|a, b| a.0.buy_price.partial_cmp(&b.0.buy_price).unwrap());

        for (key, row) in &self.rows {
            self.print_row(key, row);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("griddle.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let config: GTableConfig = result?;
        let table: GTable = GTable::new(&config);

        let mut printer = Printer { rows: vec![] };
        Printer::print_header(&table.code, &table.name);
        printer.add_rows(&table, "black", "小网", config.small_grid, 0);
        printer.add_rows(&table, "blue", "中网", config.medium_grid, 1);
        printer.add_rows(&table, "green", "大网", config.large_grid, 1);

        printer.print();
        println!("\n");
    }
    Ok(())
}
