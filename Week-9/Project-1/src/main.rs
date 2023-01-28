use std::io::Write;

fn main() {
    let lager = vec!["33 Export ", "Desperados", "Goldberg  ", "Guilder   ", "Heineken  ", "Star      "];
    let stout = vec!["Legend    ", "Turbo King", "Williams  ", "          ", "          ", "          "];
    let non_alcoholic = vec!["Maltina   ", "Amstel Malta", "Malta Gold", "Fayrouz   ", "          ", "          "];

    let mut nbl_file = std::fs::File::create("Nigerian_Brewery_Limited.txt")
        .unwrap_or_else(|e| {
            eprintln!("Error creating file: {}", e);
            std::process::exit(1);
        });

    let header = "LAGER      |   STOUT    | NON-ALCOHOLIC\n_______________________________________\n";
    nbl_file.write_all(header.as_bytes())
        .unwrap_or_else(|e| {
            eprintln!("Error writing header: {}", e);
            std::process::exit(1);
        });

    for i in 0..stout.len() {
        let line = format!("{} | {} | {}\n", lager[i], stout[i], non_alcoholic[i]);
        nbl_file.write_all(line.as_bytes())
            .unwrap_or_else(|e| {
                eprintln!("Error writing line {}: {}", i+1, e);
                std::process::exit(1);
            });
    }
}

