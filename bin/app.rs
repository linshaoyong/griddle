use std::error::Error;
use std::fs::File;

use griddle::{GRow, GTable, GTableConfig};

struct Printer;

impl Printer {
    fn print_header(name: &str, code: &str) {
        println!("#### {}（{}）", code, name);
        println!("\n");
        println!("{}", "| 种类 | 档位 | 买入触发价 | 买入价 | 买入金额 | 入股数 | 卖出触发价 | 卖出价 | 出股数 |");
        println!("{}", "| ---- | ---- | ---------- | ------ | -------- | ------ | ---------- | ------ | :----- |");
    }

    fn print_rows(table: &GTable, color: &str, grid_name: &str, grid: f32, mut index: u32) {
        let mut gear = 1.0;
        let lowest_gear = 0.3;
        while gear > lowest_gear {
            let row = table.nth_row(index, grid);
            if row.gear < lowest_gear {
                break;
            }
            Printer::print_row(row, color, grid_name, grid);
            index += 1;
            gear -= grid;
        }
    }

    fn print_row(row: GRow, color: &str, grid_name: &str, grid: f32) {
        println!(
            "|<span style=\"color:{}\"> {} </span>| {:.2} | {:.3} | {:.3} | {} | {} | {:.3} | {:.3} | {} |",
            color,
            grid_name,
            row.gear,
            row.buy_trigger_price(),
            row.buy_price,
            row.buy_money() as u32,
            row.buy_numbers as u32,
            row.sell_trigger_price(grid),
            row.sell_price(grid),
            row.sell_numbers(grid) as u32,
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("griddle.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let config: GTableConfig = result?;
        let table: GTable = GTable::new(&config);

        Printer::print_header(&table.code, &table.name);
        Printer::print_rows(&table, "black", "小网", config.small_grid, 0);
        Printer::print_rows(&table, "blue", "中网", config.medium_grid, 1);
        Printer::print_rows(&table, "green", "大网", config.large_grid, 1);
        println!("\n");
    }
    Ok(())
}
