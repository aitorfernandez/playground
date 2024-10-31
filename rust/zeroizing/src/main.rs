use zeroize::Zeroize;

fn main() {
    let mut sensitive_data = String::from("password");

    sensitive_data.zeroize();
    println!("data after zeroizing: {sensitive_data}");
}
