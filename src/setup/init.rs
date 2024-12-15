use std::process::Command;
pub fn init(ip: String, port: u16, dbug: bool, worker: u8) {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
    println!("\x1b[0;32m");
    let ascii_art = format!(
        r#"
                    +++             ++      |
                    + ++           ++++     |
                   ++  +++        ++  +     |    created by: Mikhail Void Akhsakov 
                  ++     ++++++++++    +    |    info:
                  +                    +    |      ip: {}
          +       +                    +    |      port: {}
        ++++++   ++   ───       #==    +    |      debug_mode: {}
       ++    +++ +                     +    |      worker: {}
      ++      ++ +           ___       +    |      
    +++        +++       ___ \  \     ++    |
    +           ++        \-____|     +     |
    +         +++                    ++     |
    +         +                      +      |
    +        ++                      +      |
    +        ++                      +      |
    ++       +                      +       |
     +      ++       +  +        +  +       |
     ++    +++++++++ +  +        +  +       |
      ++   +       + +  +    +   +  +       |
       ++  +       + +  +    +   +  +       |
       +++++       +++  +    +   +  +       |
          ++       +++  ++++++   +  +       |
           +++++++++++  +++  +++++  +++     |
                     ++++++      ++++++     |
    "#,
        ip, port, dbug, worker
    );
    println!("{}", ascii_art);
}
