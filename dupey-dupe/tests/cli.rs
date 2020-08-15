use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command; // Run programs

#[test]
fn test_cli() -> Result<(), Box<dyn std::error::Error>> {
    let test_args = vec![
        ("chocolate lava cake lava cake", "cake\n"),
        ("chocolate lava cake cake lava", "lava\n"),
        ("chocolate lava cake cake cake", "cake\n"),
        ("chocolate lava lava cake cake", "cake\n"),
        ("chocolate lava dupe cake dupe cake cake", "cake\n"),
        ("be careful with this dupe", "no dupey dupe :(\n"),
        (
            "chocolate lava dupe dupe cake dupe cake cake chocolate",
            "chocolate\ndupe dupe!\n",
        ),
        (
            "cupcake cupcake dupe dupe dupe dupe dupe",
            "dupe\ndupe dupe!\n",
        ),
    ];

    for (args, expected) in test_args {
        let mut cmd = Command::cargo_bin("dupey-dupe")?;

        let split_args = args.split_whitespace();
        cmd.args(split_args).assert().stdout(expected);
    }

    Ok(())
}
