use std::collections::HashMap;

// Objectives
//
// Initialize an empty HashMap to represent the inventory.
// Insert items into the inventory with initial quantities.
// Access the quantity of specific items.
// Update item quantities based on sales and new stock.
// Remove items that are no longer sold in the store.
// Iterate over the inventory to display all items and their quantities.
// Check if certain items are in stock.
// Merge a restock list with the inventory.

fn main() {
    let mut inventory: HashMap<String, i32> = HashMap::new();

    inventory.insert("Apples".to_string(), 10);
    inventory.insert("Bananas".to_string(), 10);
    inventory.insert("Oranges".to_string(), 10);

    check_stock(&inventory, "Apples");
    check_stock(&inventory, "Grappes");

    sell_item(&mut inventory, "Apples", 2);
    restock_item(&mut inventory, "Bananas", 10);
    restock_item(&mut inventory, "Grapes", 5);

    discontinue_item(&mut inventory, "Oranges");

    print_inventory(&inventory);

    // Bulk restock
    let mut restock = HashMap::new();
    restock.insert("Grapes".to_string(), 15);
    restock.insert("Bananas".to_string(), 5);

    bulk_restock(&mut inventory, restock);

    print_inventory(&inventory);

    inventory.clear();

    print_inventory(&inventory);
}

fn check_stock(inventory: &HashMap<String, i32>, key: &str) {
    match inventory.get(key) {
        Some(value) => println!("{key}: {value}"),
        None => println!("{key} are not available"),
    }
}

fn sell_item(inventory: &mut HashMap<String, i32>, key: &str, amount: i32) {
    if let Some(value) = inventory.get_mut(key) {
        if *value >= amount {
            *value -= amount;
            println!("Sold {amount} {key}");
        } else {
            println!("Not enough {key} in stock. Only {value} available");
        }
    } else {
        println!("{key} is not in stock");
    }
}

fn restock_item(inventory: &mut HashMap<String, i32>, key: &str, amount: i32) {
    inventory
        .entry(key.to_string())
        .and_modify(|v| *v += amount)
        .or_insert(amount);

    println!("Restocked {amount} {key}");
}

fn discontinue_item(inventory: &mut HashMap<String, i32>, key: &str) {
    inventory.remove(key);
}

fn print_inventory(inventory: &HashMap<String, i32>) {
    println!("Inventory:");
    for (k, v) in inventory {
        println!("{k}: {v} item(s)");
    }
}

fn bulk_restock(inventory: &mut HashMap<String, i32>, restock: HashMap<String, i32>) {
    // inventory.extend(restock);

    for (key, amount) in restock {
        inventory
            .entry(key)
            .and_modify(|v| *v += amount)
            .or_insert(amount);
    }
}
