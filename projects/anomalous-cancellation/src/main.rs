use std::{fs::File, io::Write};

use anomalous_cancellation::debug_latex;

fn main() -> std::io::Result<()> {
    let limit = 10;
    // write to file
    let mut file = File::create("debug.md")?;
    println!("finding: 1/2");
    file.write_all(debug_latex(1, 2, limit).as_bytes())?;
    println!("finding: 1/3");
    file.write_all(debug_latex(1, 3, limit).as_bytes())?;
    println!("finding: 2/3");
    file.write_all(debug_latex(2, 3, limit).as_bytes())?;
    println!("finding: 1/4");
    file.write_all(debug_latex(1, 4, limit).as_bytes())?;
    println!("finding: 3/4");
    file.write_all(debug_latex(3, 4, limit).as_bytes())?;
    println!("finding: 1/5");
    file.write_all(debug_latex(1, 5, limit).as_bytes())?;
    println!("finding: 2/5");
    file.write_all(debug_latex(2, 5, limit).as_bytes())?;
    println!("finding: 3/5");
    file.write_all(debug_latex(3, 5, limit).as_bytes())?;
    println!("finding: 4/5");
    file.write_all(debug_latex(4, 5, limit).as_bytes())?;
    println!("finding: 1/6");
    file.write_all(debug_latex(1, 6, limit).as_bytes())?;
    println!("finding: 5/6");
    file.write_all(debug_latex(5, 6, limit).as_bytes())?;
    println!("finding: 1/7");
    file.write_all(debug_latex(1, 7, limit).as_bytes())?;
    println!("finding: 2/7");
    file.write_all(debug_latex(2, 7, limit).as_bytes())?;
    println!("finding: 3/7");
    file.write_all(debug_latex(3, 7, limit).as_bytes())?;
    println!("finding: 4/7");
    file.write_all(debug_latex(4, 7, limit).as_bytes())?;
    println!("finding: 5/7");
    file.write_all(debug_latex(5, 7, limit).as_bytes())?;
    println!("finding: 6/7");
    file.write_all(debug_latex(6, 7, limit).as_bytes())?;
    println!("finding: 1/8");
    file.write_all(debug_latex(1, 8, limit).as_bytes())?;
    println!("finding: 3/8");
    file.write_all(debug_latex(3, 8, limit).as_bytes())?;
    println!("finding: 5/8");
    file.write_all(debug_latex(5, 8, limit).as_bytes())?;
    println!("finding: 7/8");
    file.write_all(debug_latex(7, 8, limit).as_bytes())?;

    write!(
        file,
        r#"
### 1/9
???
"#
    )
    .unwrap();

    // println!("#finding: 1/9");
    // file.write_all(debug_latex(1, 9, limit).as_bytes())?;
    println!("finding: 2/9");
    file.write_all(debug_latex(2, 9, limit).as_bytes())?;
    println!("finding: 4/9");
    file.write_all(debug_latex(4, 9, limit).as_bytes())?;
    println!("finding: 5/9");
    file.write_all(debug_latex(5, 9, limit).as_bytes())?;
    println!("finding: 7/9");
    file.write_all(debug_latex(7, 9, limit).as_bytes())?;
    println!("finding: 8/9");
    file.write_all(debug_latex(8, 9, limit).as_bytes())?;
    Ok(())
}
