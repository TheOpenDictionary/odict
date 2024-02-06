// type closure func() error

// // t times the amount of time it takes for a closure to
// // execute then prints the elapsed time
// func t(c *cli.Context, cb closure) error {
// 	start := time.Now()

// 	err := cb()

// 	if !c.Bool("quiet") && err == nil {
// 		fmt.Printf("\n✨ Completed in %s\n", time.Since(start).String())
// 	}

// 	return err
// }

use std::error::Error;

pub fn t<F>(cb: F, quiet: bool) -> Result<(), Box<dyn Error>>
where
    F: FnOnce() -> Result<(), Box<dyn Error>>,
{
    let start = std::time::Instant::now();
    let err = cb();

    if let Err(msg) = err {
        println!("{}", msg)
    } else if !quiet {
        println!("✨ Completed in {:?}", start.elapsed());
    }

    Ok(())
}
